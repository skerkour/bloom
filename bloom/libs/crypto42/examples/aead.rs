use crypto42::{aead::xchacha20poly1305_ietf, zeroize::Zeroize, Error as Crypto42Error};
use std::str;

fn main() -> Result<(), Crypto42Error> {
    crypto42::init()?;

    let plaintext = "Hello world!";
    let mut key = xchacha20poly1305_ietf::gen_key();
    let mut nonce = xchacha20poly1305_ietf::gen_nonce();

    let ciphertext = xchacha20poly1305_ietf::encrypt(&plaintext.as_bytes(), None, &nonce, &key);
    let plaintext2 = xchacha20poly1305_ietf::decrypt(&ciphertext, None, &nonce, &key)
        .expect("Error decrypting ciphertext");
    let plaintext2 = str::from_utf8(&plaintext2).expect("error converting plaintext buffer to str");

    assert_eq!(plaintext, plaintext2);
    println!("{}", plaintext2);

    // zero memory
    key.0.zeroize();
    nonce.0.zeroize();

    return Ok(());
}
