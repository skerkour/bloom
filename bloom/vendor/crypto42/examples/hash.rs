use crypto42::{hash::blake2b, Error as Crypto42Error};

fn main() -> Result<(), Crypto42Error> {
    crypto42::init().expect("error initializing crypto42");

    let data = "Hello world!";
    let mut hash_state = blake2b::State::new(blake2b::DIGEST_512, None)?;
    hash_state
        .update(data.as_bytes())
        .expect("error updating hash state");
    let digest = hash_state.finalize().expect("error getting hash digest");

    println!("{}", hex::encode(digest));

    return Ok(());
}
