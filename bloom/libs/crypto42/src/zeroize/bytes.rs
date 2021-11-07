//! `Zeroize` impl for the `BytesMut` type from the `bytes` crate

// TODO(tarcieri): upstream this?

use super::Zeroize;
use ::bytes::BytesMut;

#[cfg(feature = "zeroize_bytes")]
impl Zeroize for BytesMut {
    fn zeroize(&mut self) {
        self.resize(self.capacity(), Default::default());
        self.as_mut().zeroize();
        self.clear();
        debug_assert!(self.iter().all(|b| *b == 0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroize_bytes() {
        let mut data = BytesMut::from("data");
        data.zeroize();
        assert!(data.is_empty());

        let mut data = BytesMut::from("data");
        data.zeroize();
        assert!(data.is_empty());
    }
}
