pub use ::rand::*;

use seq::SliceRandom;

pub fn alphabet(alphabet: &str, n: usize) -> String {
    let mut rng = thread_rng();
    alphabet
        .as_bytes()
        .choose_multiple(&mut rng, n)
        .map(|c| *c as char)
        .collect()
}
