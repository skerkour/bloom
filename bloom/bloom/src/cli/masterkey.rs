use kernel::Error;
use stdx::encoding::base64;
use stdx::rand::{thread_rng, Rng};

pub fn run() -> Result<(), Error> {
    let mut rng = thread_rng();
    let key: [u8; 32] = rng.gen();
    let base64_key = base64::encode(key);
    println!("{}", base64_key);
    Ok(())
}
