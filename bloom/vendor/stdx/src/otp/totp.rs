use super::Error;
use crate::crypto;
use crate::{base32, sync::threadpool::spawn_blocking};
use byteorder::{BigEndian, ReadBytesExt};
use chrono::Utc;
use rand::{thread_rng, Rng};
use ring::hmac;
use std::io::Cursor;

const DIGITS: usize = 6;
const SECRET_SIZE: usize = 20; // 20 bytes: sha1 size
const PERIOD: usize = 30;
const ALGORITHM: &str = "SHA1";
const DISCREPANCY: i64 = 0;

pub async fn validate(code: String, secret: String) -> Result<bool, Error> {
    spawn_blocking(move || {
        if code.len() != DIGITS {
            return Ok(false);
        }

        // Support for TOTP secrets that are
        // missing their padding.
        let mut secret = secret.trim().to_uppercase();
        let secret_padding = secret.len() % 8;
        if secret_padding != 0 {
            secret = secret + "=".repeat(secret_padding).as_str();
        }

        let secret = base32::decode(
            base32::Alphabet::RFC4648 {
                padding: true,
            },
            &secret,
        )
        .ok_or(Error::DecodingSecret)?;

        let curr_time_slice = Utc::now().timestamp() / 30;
        let start_time = curr_time_slice.saturating_sub(DISCREPANCY) as u64;
        let end_time = curr_time_slice.saturating_add(DISCREPANCY + 1) as u64;

        for time_slice in start_time..end_time {
            let valid_code = generate_code(&secret, time_slice);
            if crypto::constant_time_compare(code.as_bytes(), valid_code.as_bytes()) {
                return Ok(true);
            }
        }
        Ok(false)
    })
    .await?
}

// url format: otpauth://totp/Example:alice@google.com?secret=JBSWY3DPEHPK3PXP&issuer=Example
pub async fn generate(issuer: String, account_name: String) -> Result<super::Key, Error> {
    Ok(spawn_blocking(move || {
        let mut secret = [0u8; SECRET_SIZE];
        thread_rng().fill(&mut secret[..]);

        let secret_str = base32::encode(
            base32::Alphabet::RFC4648 {
                padding: false,
            },
            &secret,
        );

        let url = format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&period={}&algorithm={}&digits={}",
            issuer, account_name, &secret_str, issuer, PERIOD, ALGORITHM, DIGITS
        );

        super::Key::new(secret_str, url)
    })
    .await?)
}

// see https://github.com/bloom42/gobox/blob/c19cbcf2c3e55d27c59bce453056099e9eb58483/otp/hotp/hotp.go#L73
// for a more correct implementation
fn generate_code(secret: &[u8], counter: u64) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, secret);
    let counter_buffer = counter.to_be_bytes().to_vec();
    let result = hmac::sign(&key, &counter_buffer);
    let digest = result.as_ref();
    let last_byte = digest[19];

    // "Dynamic truncation" in RFC 4226
    // http://tools.ietf.org/html/rfc4226#section-5.4
    let offset = (last_byte & 0xf) as usize;
    let mut cursor = Cursor::new(digest[offset..offset + 4].to_vec());
    let value = cursor.read_u32::<BigEndian>().expect("otp.generate_code") & 0x7f_ff_ff_ff;

    let value = value % 1_000_000;

    format!("{:06}", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    const ISSUER: &str = "Bloom";
    const ACCOUNT_NAME: &str = "Sylvain";

    #[tokio::test]
    async fn generate_and_valdiate() {
        let key = generate(ISSUER.to_string(), ACCOUNT_NAME.to_string()).await.unwrap();
        let secret = key.secret();

        let counter = (Utc::now().timestamp() / 30) as u64;
        let code = generate_code(secret.as_bytes(), counter);

        let result = validate(code, secret).await.unwrap();

        assert!(result);
    }
}
