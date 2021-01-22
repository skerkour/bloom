use std::cmp;
use std::collections::HashMap;

use super::Code;
use super::Lz77Encode;
use super::Sink;

/// A `Lz77Encode` implementation used by default.
#[derive(Debug)]
pub struct DefaultLz77Encoder {
    window_size: u16,
    buf: Vec<u8>,
}
impl DefaultLz77Encoder {
    /// Makes a new encoder instance.
    ///
    /// # Examples
    /// ```
    /// use libflate::deflate;
    /// use libflate::lz77::{self, Lz77Encode, DefaultLz77Encoder};
    ///
    /// let lz77 = DefaultLz77Encoder::new();
    /// assert_eq!(lz77.window_size(), lz77::MAX_WINDOW_SIZE);
    ///
    /// let options = deflate::EncodeOptions::with_lz77(lz77);
    /// let _deflate = deflate::Encoder::with_options(Vec::new(), options);
    /// ```
    pub fn new() -> Self {
        Self::with_window_size(super::MAX_WINDOW_SIZE)
    }

    /// Makes a new encoder instance with specified window size.
    ///
    /// Larger window size is prefered to raise compression ratio,
    /// but it may require more working memory to encode and decode data.
    ///
    /// # Examples
    /// ```
    /// use libflate::deflate;
    /// use libflate::lz77::{self, Lz77Encode, DefaultLz77Encoder};
    ///
    /// let lz77 = DefaultLz77Encoder::with_window_size(1024);
    /// assert_eq!(lz77.window_size(), 1024);
    ///
    /// let options = deflate::EncodeOptions::with_lz77(lz77);
    /// let _deflate = deflate::Encoder::with_options(Vec::new(), options);
    /// ```
    pub fn with_window_size(size: u16) -> Self {
        DefaultLz77Encoder {
            window_size: cmp::min(size, super::MAX_WINDOW_SIZE),
            buf: Vec::new(),
        }
    }
}
impl Default for DefaultLz77Encoder {
    fn default() -> Self {
        Self::new()
    }
}
impl Lz77Encode for DefaultLz77Encoder {
    fn encode<S>(&mut self, buf: &[u8], sink: S)
    where
        S: Sink,
    {
        self.buf.extend_from_slice(buf);
        if self.buf.len() >= self.window_size as usize * 8 {
            self.flush(sink);
        }
    }
    fn flush<S>(&mut self, mut sink: S)
    where
        S: Sink,
    {
        let mut prefix_table = PrefixTable::new(self.buf.len());
        let mut i = 0;
        let end = cmp::max(3, self.buf.len()) - 3;
        while i < end {
            let key = prefix(&self.buf[i..]);
            let matched = prefix_table.insert(key, i as u32);
            if let Some(j) = matched.map(|j| j as usize) {
                let distance = i - j;
                if distance <= self.window_size as usize {
                    let length = 3 + longest_common_prefix(&self.buf, i + 3, j + 3);
                    sink.consume(Code::Pointer {
                        length,
                        backward_distance: distance as u16,
                    });
                    for k in (i..).take(length as usize).skip(1) {
                        if k >= end {
                            break;
                        }
                        prefix_table.insert(prefix(&self.buf[k..]), k as u32);
                    }
                    i += length as usize;
                    continue;
                }
            }
            sink.consume(Code::Literal(self.buf[i]));
            i += 1;
        }
        for b in &self.buf[i..] {
            sink.consume(Code::Literal(*b));
        }
        self.buf.clear();
    }
    fn window_size(&self) -> u16 {
        self.window_size
    }
}

#[inline]
fn prefix(input_buf: &[u8]) -> [u8; 3] {
    let buf: &[u8] = &input_buf[..3]; // perform bounds check once
    [buf[0], buf[1], buf[2]]
}

#[inline]
fn longest_common_prefix(buf: &[u8], i: usize, j: usize) -> u16 {
    buf[i..]
        .iter()
        .take(super::MAX_LENGTH as usize - 3)
        .zip(&buf[j..])
        .take_while(|&(x, y)| x == y)
        .count() as u16
}

#[derive(Debug)]
enum PrefixTable {
    Small(HashMap<[u8; 3], u32>),
    Large(LargePrefixTable),
}
impl PrefixTable {
    fn new(bytes: usize) -> Self {
        if bytes < super::MAX_WINDOW_SIZE as usize {
            PrefixTable::Small(HashMap::new())
        } else {
            PrefixTable::Large(LargePrefixTable::new())
        }
    }

    #[inline]
    fn insert(&mut self, prefix: [u8; 3], position: u32) -> Option<u32> {
        match *self {
            PrefixTable::Small(ref mut x) => x.insert(prefix, position),
            PrefixTable::Large(ref mut x) => x.insert(prefix, position),
        }
    }
}

#[derive(Debug)]
struct LargePrefixTable {
    table: Vec<Vec<(u8, u32)>>,
}
impl LargePrefixTable {
    fn new() -> Self {
        LargePrefixTable {
            table: (0..=0xFFFF).map(|_| Vec::new()).collect(),
        }
    }

    #[inline]
    fn insert(&mut self, prefix: [u8; 3], position: u32) -> Option<u32> {
        let p0 = prefix[0] as usize;
        let p1 = prefix[1] as usize;
        let p2 = prefix[2];

        let i = (p0 << 8) + p1;
        let positions = &mut self.table[i];
        for &mut (key, ref mut value) in positions.iter_mut() {
            if key == p2 {
                let old = *value;
                *value = position;
                return Some(old);
            }
        }
        positions.push((p2, position));
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deflate::symbol::Symbol;

    #[test]
    // See: https://github.com/sile/libflate/issues/21
    fn issue21() {
        let mut enc = DefaultLz77Encoder::new();
        let mut sink = Vec::new();
        enc.encode(b"aaaaa", &mut sink);
        enc.flush(&mut sink);
        assert_eq!(
            sink,
            vec![
                Symbol::Literal(97),
                Symbol::Share {
                    length: 4,
                    distance: 1
                }
            ]
        );
    }
}
