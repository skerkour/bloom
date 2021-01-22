use crate::base32;
use crate::crypto;
use byteorder::{BigEndian, ReadBytesExt};
use chrono::Utc;
use rand::{thread_rng, Rng};
use ring::hmac;
use std::io::Cursor;

const DIGITS: usize = 6;
const SECRET_SIZE: usize = crypto::KEY_SIZE_512;
const PERIOD: usize = 30;
const ALGORITHM: &str = "SHA1";
const DISCREPANCY: i64 = 1;

pub fn validate(code: &str, secret: &str) -> bool {
    if code.len() != DIGITS {
        return false;
    }

    let curr_time_slice = Utc::now().timestamp() / 30;
    let start_time = curr_time_slice.saturating_sub(DISCREPANCY) as u64;
    let end_time = curr_time_slice.saturating_add(DISCREPANCY + 1) as u64;

    for time_slice in start_time..end_time {
        let valid_code = generate_code(secret.as_bytes(), time_slice);
        if crypto::constant_time_compare(code.as_bytes(), valid_code.as_bytes()) {
            return true;
        }
    }
    false
}

// url format: otpauth://totp/Example:alice@google.com?secret=JBSWY3DPEHPK3PXP&issuer=Example
pub fn generate(issuer: &str, account_name: &str) -> super::Key {
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
}

fn generate_code(secret: &[u8], counter: u64) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, secret);
    let wtr = counter.to_be_bytes().to_vec();
    let result = hmac::sign(&key, &wtr);
    let digest = result.as_ref();
    let ob = digest[19];
    let pos = (ob & 15) as usize;
    let mut cursor = Cursor::new(digest[pos..pos + 4].to_vec());
    let base = cursor.read_u32::<BigEndian>().expect("otp.generate_code") & 0x7fff_ffff;

    (base % 1_000_000).to_string()
}
