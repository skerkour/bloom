//! Find the optimal data mode sequence to encode a piece of data.
use super::types::{Mode, Version};
use std::slice::Iter;

#[cfg(feature = "bench")]
extern crate test;

//------------------------------------------------------------------------------
//{{{ Segment

/// A segment of data committed to an encoding mode.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Segment {
    /// The encoding mode of the segment of data.
    pub mode: Mode,

    /// The start index of the segment.
    pub begin: usize,

    /// The end index (exclusive) of the segment.
    pub end: usize,
}

impl Segment {
    /// Compute the number of bits (including the size of the mode indicator and
    /// length bits) when this segment is encoded.
    pub fn encoded_len(&self, version: Version) -> usize {
        let byte_size = self.end - self.begin;
        let chars_count = if self.mode == Mode::Kanji {
            byte_size / 2
        } else {
            byte_size
        };

        let mode_bits_count = version.mode_bits_count();
        let length_bits_count = self.mode.length_bits_count(version);
        let data_bits_count = self.mode.data_bits_count(chars_count);

        mode_bits_count + length_bits_count + data_bits_count
    }
}

//}}}
//------------------------------------------------------------------------------
//{{{ Parser

/// This iterator is basically equivalent to
///
/// ```ignore
/// data.map(|c| ExclCharSet::from_u8(*c))
///     .chain(Some(ExclCharSet::End).move_iter())
///     .enumerate()
/// ```
///
/// But the type is too hard to write, thus the new type.
///
struct EcsIter<I> {
    base: I,
    index: usize,
    ended: bool,
}

impl<'a, I: Iterator<Item = &'a u8>> Iterator for EcsIter<I> {
    type Item = (usize, ExclCharSet);

    fn next(&mut self) -> Option<(usize, ExclCharSet)> {
        if self.ended {
            return None;
        }

        match self.base.next() {
            None => {
                self.ended = true;
                Some((self.index, ExclCharSet::End))
            }
            Some(c) => {
                let old_index = self.index;
                self.index += 1;
                Some((old_index, ExclCharSet::from_u8(*c)))
            }
        }
    }
}

/// QR code data parser to classify the input into distinct segments.
pub struct Parser<'a> {
    ecs_iter: EcsIter<Iter<'a, u8>>,
    state: State,
    begin: usize,
    pending_single_byte: bool,
}

impl<'a> Parser<'a> {
    /// Creates a new iterator which parse the data into segments that only
    /// contains their exclusive subsets. No optimization is done at this point.
    ///
    ///     use qrcode::optimize::{Parser, Segment};
    ///     use qrcode::types::Mode::{Alphanumeric, Numeric, Byte};
    ///
    ///     let parse_res = Parser::new(b"ABC123abcd").collect::<Vec<Segment>>();
    ///     assert_eq!(parse_res, vec![Segment { mode: Alphanumeric, begin: 0, end: 3 },
    ///                                Segment { mode: Numeric, begin: 3, end: 6 },
    ///                                Segment { mode: Byte, begin: 6, end: 10 }]);
    ///
    pub fn new(data: &[u8]) -> Parser {
        Parser {
            ecs_iter: EcsIter {
                base: data.iter(),
                index: 0,
                ended: false,
            },
            state: State::Init,
            begin: 0,
            pending_single_byte: false,
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Segment;

    fn next(&mut self) -> Option<Segment> {
        if self.pending_single_byte {
            self.pending_single_byte = false;
            self.begin += 1;
            return Some(Segment {
                mode: Mode::Byte,
                begin: self.begin - 1,
                end: self.begin,
            });
        }

        loop {
            let (i, ecs) = match self.ecs_iter.next() {
                None => return None,
                Some(a) => a,
            };
            let (next_state, action) = STATE_TRANSITION[self.state as usize + ecs as usize];
            self.state = next_state;

            let old_begin = self.begin;
            let push_mode = match action {
                Action::Idle => continue,
                Action::Numeric => Mode::Numeric,
                Action::Alpha => Mode::Alphanumeric,
                Action::Byte => Mode::Byte,
                Action::Kanji => Mode::Kanji,
                Action::KanjiAndSingleByte => {
                    let next_begin = i - 1;
                    if self.begin == next_begin {
                        Mode::Byte
                    } else {
                        self.pending_single_byte = true;
                        self.begin = next_begin;
                        return Some(Segment {
                            mode: Mode::Kanji,
                            begin: old_begin,
                            end: next_begin,
                        });
                    }
                }
            };

            self.begin = i;
            return Some(Segment {
                mode: push_mode,
                begin: old_begin,
                end: i,
            });
        }
    }
}

#[cfg(test)]
mod parse_tests {
    use crate::qrcode::optimize::{Parser, Segment};
    use crate::qrcode::types::Mode;

    fn parse(data: &[u8]) -> Vec<Segment> {
        Parser::new(data).collect()
    }

    #[test]
    fn test_parse_1() {
        let segs = parse(b"01049123451234591597033130128%10ABC123");
        assert_eq!(
            segs,
            vec![
                Segment {
                    mode: Mode::Numeric,
                    begin: 0,
                    end: 29
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 29,
                    end: 30
                },
                Segment {
                    mode: Mode::Numeric,
                    begin: 30,
                    end: 32
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 32,
                    end: 35
                },
                Segment {
                    mode: Mode::Numeric,
                    begin: 35,
                    end: 38
                },
            ]
        );
    }

    #[test]
    fn test_parse_shift_jis_example_1() {
        let segs = parse(b"\x82\xa0\x81\x41\x41\xb1\x81\xf0"); // "あ、AｱÅ"
        assert_eq!(
            segs,
            vec![
                Segment {
                    mode: Mode::Kanji,
                    begin: 0,
                    end: 4
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 4,
                    end: 5
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 5,
                    end: 6
                },
                Segment {
                    mode: Mode::Kanji,
                    begin: 6,
                    end: 8
                },
            ]
        );
    }

    #[test]
    fn test_parse_utf_8() {
        // Mojibake?
        let segs = parse(b"\xe3\x81\x82\xe3\x80\x81A\xef\xbd\xb1\xe2\x84\xab");
        assert_eq!(
            segs,
            vec![
                Segment {
                    mode: Mode::Kanji,
                    begin: 0,
                    end: 4
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 4,
                    end: 5
                },
                Segment {
                    mode: Mode::Kanji,
                    begin: 5,
                    end: 7
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 7,
                    end: 10
                },
                Segment {
                    mode: Mode::Kanji,
                    begin: 10,
                    end: 12
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 12,
                    end: 13
                },
            ]
        );
    }

    #[test]
    fn test_not_kanji_1() {
        let segs = parse(b"\x81\x30");
        assert_eq!(
            segs,
            vec![
                Segment {
                    mode: Mode::Byte,
                    begin: 0,
                    end: 1
                },
                Segment {
                    mode: Mode::Numeric,
                    begin: 1,
                    end: 2
                },
            ]
        );
    }

    #[test]
    fn test_not_kanji_2() {
        // Note that it's implementation detail that the byte seq is split into
        // two. Perhaps adjust the test to check for this.
        let segs = parse(b"\xeb\xc0");
        assert_eq!(
            segs,
            vec![
                Segment {
                    mode: Mode::Byte,
                    begin: 0,
                    end: 1
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 1,
                    end: 2
                },
            ]
        );
    }

    #[test]
    fn test_not_kanji_3() {
        let segs = parse(b"\x81\x7f");
        assert_eq!(
            segs,
            vec![
                Segment {
                    mode: Mode::Byte,
                    begin: 0,
                    end: 1
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 1,
                    end: 2
                },
            ]
        );
    }

    #[test]
    fn test_not_kanji_4() {
        let segs = parse(b"\x81\x40\x81");
        assert_eq!(
            segs,
            vec![
                Segment {
                    mode: Mode::Kanji,
                    begin: 0,
                    end: 2
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 2,
                    end: 3
                },
            ]
        );
    }
}

//}}}
//------------------------------------------------------------------------------
//{{{ Optimizer

pub struct Optimizer<I> {
    parser: I,
    last_segment: Segment,
    last_segment_size: usize,
    version: Version,
    ended: bool,
}

impl<I: Iterator<Item = Segment>> Optimizer<I> {
    /// Optimize the segments by combining adjacent segments when possible.
    ///
    /// Currently this method uses a greedy algorithm by combining segments from
    /// left to right until the new segment is longer than before. This method
    /// does *not* use Annex J from the ISO standard.
    ///
    pub fn new(mut segments: I, version: Version) -> Self {
        match segments.next() {
            None => Self {
                parser: segments,
                last_segment: Segment {
                    mode: Mode::Numeric,
                    begin: 0,
                    end: 0,
                },
                last_segment_size: 0,
                version,
                ended: true,
            },
            Some(segment) => Self {
                parser: segments,
                last_segment: segment,
                last_segment_size: segment.encoded_len(version),
                version,
                ended: false,
            },
        }
    }
}

impl<'a> Parser<'a> {
    pub fn optimize(self, version: Version) -> Optimizer<Parser<'a>> {
        Optimizer::new(self, version)
    }
}

impl<I: Iterator<Item = Segment>> Iterator for Optimizer<I> {
    type Item = Segment;

    fn next(&mut self) -> Option<Segment> {
        if self.ended {
            return None;
        }

        loop {
            match self.parser.next() {
                None => {
                    self.ended = true;
                    return Some(self.last_segment);
                }
                Some(segment) => {
                    let seg_size = segment.encoded_len(self.version);

                    let new_segment = Segment {
                        mode: self.last_segment.mode.max(segment.mode),
                        begin: self.last_segment.begin,
                        end: segment.end,
                    };
                    let new_size = new_segment.encoded_len(self.version);

                    if self.last_segment_size + seg_size >= new_size {
                        self.last_segment = new_segment;
                        self.last_segment_size = new_size;
                    } else {
                        let old_segment = self.last_segment;
                        self.last_segment = segment;
                        self.last_segment_size = seg_size;
                        return Some(old_segment);
                    }
                }
            }
        }
    }
}

/// Computes the total encoded length of all segments.
pub fn total_encoded_len(segments: &[Segment], version: Version) -> usize {
    segments.iter().map(|seg| seg.encoded_len(version)).sum()
}

#[cfg(test)]
mod optimize_tests {
    use crate::qrcode::optimize::{total_encoded_len, Optimizer, Segment};
    use crate::qrcode::types::{Mode, Version};

    fn test_optimization_result(given: Vec<Segment>, expected: Vec<Segment>, version: Version) {
        let prev_len = total_encoded_len(&*given, version);
        let opt_segs = Optimizer::new(given.iter().map(|seg| *seg), version).collect::<Vec<_>>();
        let new_len = total_encoded_len(&*opt_segs, version);
        if given != opt_segs {
            assert!(prev_len > new_len, "{} > {}", prev_len, new_len);
        }
        assert!(
            opt_segs == expected,
            "Optimization gave something better: {} < {} ({:?})",
            new_len,
            total_encoded_len(&*expected, version),
            opt_segs
        );
    }

    #[test]
    fn test_example_1() {
        test_optimization_result(
            vec![
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 0,
                    end: 3,
                },
                Segment {
                    mode: Mode::Numeric,
                    begin: 3,
                    end: 6,
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 6,
                    end: 10,
                },
            ],
            vec![
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 0,
                    end: 6,
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 6,
                    end: 10,
                },
            ],
            Version::Normal(1),
        );
    }

    #[test]
    fn test_example_2() {
        test_optimization_result(
            vec![
                Segment {
                    mode: Mode::Numeric,
                    begin: 0,
                    end: 29,
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 29,
                    end: 30,
                },
                Segment {
                    mode: Mode::Numeric,
                    begin: 30,
                    end: 32,
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 32,
                    end: 35,
                },
                Segment {
                    mode: Mode::Numeric,
                    begin: 35,
                    end: 38,
                },
            ],
            vec![
                Segment {
                    mode: Mode::Numeric,
                    begin: 0,
                    end: 29,
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 29,
                    end: 38,
                },
            ],
            Version::Normal(9),
        );
    }

    #[test]
    fn test_example_3() {
        test_optimization_result(
            vec![
                Segment {
                    mode: Mode::Kanji,
                    begin: 0,
                    end: 4,
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 4,
                    end: 5,
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 5,
                    end: 6,
                },
                Segment {
                    mode: Mode::Kanji,
                    begin: 6,
                    end: 8,
                },
            ],
            vec![Segment {
                mode: Mode::Byte,
                begin: 0,
                end: 8,
            }],
            Version::Normal(1),
        );
    }

    #[test]
    fn test_example_4() {
        test_optimization_result(
            vec![
                Segment {
                    mode: Mode::Kanji,
                    begin: 0,
                    end: 10,
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 10,
                    end: 11,
                },
            ],
            vec![
                Segment {
                    mode: Mode::Kanji,
                    begin: 0,
                    end: 10,
                },
                Segment {
                    mode: Mode::Byte,
                    begin: 10,
                    end: 11,
                },
            ],
            Version::Normal(1),
        );
    }

    #[test]
    fn test_annex_j_guideline_1a() {
        test_optimization_result(
            vec![
                Segment {
                    mode: Mode::Numeric,
                    begin: 0,
                    end: 3,
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 3,
                    end: 4,
                },
            ],
            vec![
                Segment {
                    mode: Mode::Numeric,
                    begin: 0,
                    end: 3,
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 3,
                    end: 4,
                },
            ],
            Version::Micro(2),
        );
    }

    #[test]
    fn test_annex_j_guideline_1b() {
        test_optimization_result(
            vec![
                Segment {
                    mode: Mode::Numeric,
                    begin: 0,
                    end: 2,
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 2,
                    end: 4,
                },
            ],
            vec![Segment {
                mode: Mode::Alphanumeric,
                begin: 0,
                end: 4,
            }],
            Version::Micro(2),
        );
    }

    #[test]
    fn test_annex_j_guideline_1c() {
        test_optimization_result(
            vec![
                Segment {
                    mode: Mode::Numeric,
                    begin: 0,
                    end: 3,
                },
                Segment {
                    mode: Mode::Alphanumeric,
                    begin: 3,
                    end: 4,
                },
            ],
            vec![Segment {
                mode: Mode::Alphanumeric,
                begin: 0,
                end: 4,
            }],
            Version::Micro(3),
        );
    }
}

#[cfg(feature = "bench")]
#[bench]
fn bench_optimize(bencher: &mut test::Bencher) {
    use crate::types::Version;

    let data = b"QR\x83R\x81[\x83h\x81i\x83L\x83\x85\x81[\x83A\x81[\x83\x8b\x83R\x81[\x83h\x81j\
                 \x82\xc6\x82\xcd\x81A1994\x94N\x82\xc9\x83f\x83\x93\x83\\\x81[\x82\xcc\x8aJ\
                 \x94\xad\x95\x94\x96\xe5\x81i\x8c\xbb\x8d\xdd\x82\xcd\x95\xaa\x97\xa3\x82\xb5\x83f\
                 \x83\x93\x83\\\x81[\x83E\x83F\x81[\x83u\x81j\x82\xaa\x8aJ\x94\xad\x82\xb5\x82\xbd\
                 \x83}\x83g\x83\x8a\x83b\x83N\x83X\x8c^\x93\xf1\x8e\x9f\x8c\xb3\x83R\x81[\x83h\
                 \x82\xc5\x82\xa0\x82\xe9\x81B\x82\xc8\x82\xa8\x81AQR\x83R\x81[\x83h\x82\xc6\
                 \x82\xa2\x82\xa4\x96\xbc\x8f\xcc\x81i\x82\xa8\x82\xe6\x82\xd1\x92P\x8c\xea\x81j\
                 \x82\xcd\x83f\x83\x93\x83\\\x81[\x83E\x83F\x81[\x83u\x82\xcc\x93o\x98^\x8f\xa4\
                 \x95W\x81i\x91\xe64075066\x8d\x86\x81j\x82\xc5\x82\xa0\x82\xe9\x81BQR\x82\xcd\
                 Quick Response\x82\xc9\x97R\x97\x88\x82\xb5\x81A\x8d\x82\x91\xac\x93\xc7\x82\xdd\
                 \x8e\xe6\x82\xe8\x82\xaa\x82\xc5\x82\xab\x82\xe9\x82\xe6\x82\xa4\x82\xc9\x8aJ\
                 \x94\xad\x82\xb3\x82\xea\x82\xbd\x81B\x93\x96\x8f\x89\x82\xcd\x8e\xa9\x93\xae\
                 \x8e\xd4\x95\x94\x95i\x8dH\x8f\xea\x82\xe2\x94z\x91\x97\x83Z\x83\x93\x83^\x81[\
                 \x82\xc8\x82\xc7\x82\xc5\x82\xcc\x8eg\x97p\x82\xf0\x94O\x93\xaa\x82\xc9\x8aJ\
                 \x94\xad\x82\xb3\x82\xea\x82\xbd\x82\xaa\x81A\x8c\xbb\x8d\xdd\x82\xc5\x82\xcd\x83X\
                 \x83}\x81[\x83g\x83t\x83H\x83\x93\x82\xcc\x95\x81\x8by\x82\xc8\x82\xc7\x82\xc9\
                 \x82\xe6\x82\xe8\x93\xfa\x96{\x82\xc9\x8c\xc0\x82\xe7\x82\xb8\x90\xa2\x8aE\x93I\
                 \x82\xc9\x95\x81\x8by\x82\xb5\x82\xc4\x82\xa2\x82\xe9\x81B";
    bencher.iter(|| Parser::new(data).optimize(Version::Normal(15)));
}

//}}}
//------------------------------------------------------------------------------
//{{{ Internal types and data for parsing

/// All values of `u8` can be split into 9 different character sets when
/// determining which encoding to use. This enum represents these groupings for
/// parsing purpose.
#[derive(Copy, Clone)]
enum ExclCharSet {
    /// The end of string.
    End = 0,

    /// All symbols supported by the Alphanumeric encoding, i.e. space, `$`, `%`,
    /// `*`, `+`, `-`, `.`, `/` and `:`.
    Symbol = 1,

    /// All numbers (0–9).
    Numeric = 2,

    /// All uppercase letters (A–Z). These characters may also appear in the
    /// second byte of a Shift JIS 2-byte encoding.
    Alpha = 3,

    /// The first byte of a Shift JIS 2-byte encoding, in the range 0x81–0x9f.
    KanjiHi1 = 4,

    /// The first byte of a Shift JIS 2-byte encoding, in the range 0xe0–0xea.
    KanjiHi2 = 5,

    /// The first byte of a Shift JIS 2-byte encoding, of value 0xeb. This is
    /// different from the other two range that the second byte has a smaller
    /// range.
    KanjiHi3 = 6,

    /// The second byte of a Shift JIS 2-byte encoding, in the range 0x40–0xbf,
    /// excluding letters (covered by `Alpha`), 0x81–0x9f (covered by `KanjiHi1`),
    /// and the invalid byte 0x7f.
    KanjiLo1 = 7,

    /// The second byte of a Shift JIS 2-byte encoding, in the range 0xc0–0xfc,
    /// excluding the range 0xe0–0xeb (covered by `KanjiHi2` and `KanjiHi3`).
    /// This half of byte-pair cannot appear as the second byte leaded by
    /// `KanjiHi3`.
    KanjiLo2 = 8,

    /// Any other values not covered by the above character sets.
    Byte = 9,
}

impl ExclCharSet {
    /// Determines which character set a byte is in.
    fn from_u8(c: u8) -> Self {
        match c {
            0x20 | 0x24 | 0x25 | 0x2a | 0x2b | 0x2d..=0x2f | 0x3a => ExclCharSet::Symbol,
            0x30..=0x39 => ExclCharSet::Numeric,
            0x41..=0x5a => ExclCharSet::Alpha,
            0x81..=0x9f => ExclCharSet::KanjiHi1,
            0xe0..=0xea => ExclCharSet::KanjiHi2,
            0xeb => ExclCharSet::KanjiHi3,
            0x40 | 0x5b..=0x7e | 0x80 | 0xa0..=0xbf => ExclCharSet::KanjiLo1,
            0xc0..=0xdf | 0xec..=0xfc => ExclCharSet::KanjiLo2,
            _ => ExclCharSet::Byte,
        }
    }
}

/// The current parsing state.
#[derive(Copy, Clone)]
enum State {
    /// Just initialized.
    Init = 0,

    /// Inside a string that can be exclusively encoded as Numeric.
    Numeric = 10,

    /// Inside a string that can be exclusively encoded as Alphanumeric.
    Alpha = 20,

    /// Inside a string that can be exclusively encoded as 8-Bit Byte.
    Byte = 30,

    /// Just encountered the first byte of a Shift JIS 2-byte sequence of the
    /// set `KanjiHi1` or `KanjiHi2`.
    KanjiHi12 = 40,

    /// Just encountered the first byte of a Shift JIS 2-byte sequence of the
    /// set `KanjiHi3`.
    KanjiHi3 = 50,

    /// Inside a string that can be exclusively encoded as Kanji.
    Kanji = 60,
}

/// What should the parser do after a state transition.
#[derive(Copy, Clone)]
enum Action {
    /// The parser should do nothing.
    Idle,

    /// Push the current segment as a Numeric string, and reset the marks.
    Numeric,

    /// Push the current segment as an Alphanumeric string, and reset the marks.
    Alpha,

    /// Push the current segment as a 8-Bit Byte string, and reset the marks.
    Byte,

    /// Push the current segment as a Kanji string, and reset the marks.
    Kanji,

    /// Push the current segment excluding the last byte as a Kanji string, then
    /// push the remaining single byte as a Byte string, and reset the marks.
    KanjiAndSingleByte,
}

static STATE_TRANSITION: [(State, Action); 70] = [
    // STATE_TRANSITION[current_state + next_character] == (next_state, what_to_do)

    // Init state:
    (State::Init, Action::Idle),      // End
    (State::Alpha, Action::Idle),     // Symbol
    (State::Numeric, Action::Idle),   // Numeric
    (State::Alpha, Action::Idle),     // Alpha
    (State::KanjiHi12, Action::Idle), // KanjiHi1
    (State::KanjiHi12, Action::Idle), // KanjiHi2
    (State::KanjiHi3, Action::Idle),  // KanjiHi3
    (State::Byte, Action::Idle),      // KanjiLo1
    (State::Byte, Action::Idle),      // KanjiLo2
    (State::Byte, Action::Idle),      // Byte
    // Numeric state:
    (State::Init, Action::Numeric),      // End
    (State::Alpha, Action::Numeric),     // Symbol
    (State::Numeric, Action::Idle),      // Numeric
    (State::Alpha, Action::Numeric),     // Alpha
    (State::KanjiHi12, Action::Numeric), // KanjiHi1
    (State::KanjiHi12, Action::Numeric), // KanjiHi2
    (State::KanjiHi3, Action::Numeric),  // KanjiHi3
    (State::Byte, Action::Numeric),      // KanjiLo1
    (State::Byte, Action::Numeric),      // KanjiLo2
    (State::Byte, Action::Numeric),      // Byte
    // Alpha state:
    (State::Init, Action::Alpha),      // End
    (State::Alpha, Action::Idle),      // Symbol
    (State::Numeric, Action::Alpha),   // Numeric
    (State::Alpha, Action::Idle),      // Alpha
    (State::KanjiHi12, Action::Alpha), // KanjiHi1
    (State::KanjiHi12, Action::Alpha), // KanjiHi2
    (State::KanjiHi3, Action::Alpha),  // KanjiHi3
    (State::Byte, Action::Alpha),      // KanjiLo1
    (State::Byte, Action::Alpha),      // KanjiLo2
    (State::Byte, Action::Alpha),      // Byte
    // Byte state:
    (State::Init, Action::Byte),      // End
    (State::Alpha, Action::Byte),     // Symbol
    (State::Numeric, Action::Byte),   // Numeric
    (State::Alpha, Action::Byte),     // Alpha
    (State::KanjiHi12, Action::Byte), // KanjiHi1
    (State::KanjiHi12, Action::Byte), // KanjiHi2
    (State::KanjiHi3, Action::Byte),  // KanjiHi3
    (State::Byte, Action::Idle),      // KanjiLo1
    (State::Byte, Action::Idle),      // KanjiLo2
    (State::Byte, Action::Idle),      // Byte
    // KanjiHi12 state:
    (State::Init, Action::KanjiAndSingleByte),    // End
    (State::Alpha, Action::KanjiAndSingleByte),   // Symbol
    (State::Numeric, Action::KanjiAndSingleByte), // Numeric
    (State::Kanji, Action::Idle),                 // Alpha
    (State::Kanji, Action::Idle),                 // KanjiHi1
    (State::Kanji, Action::Idle),                 // KanjiHi2
    (State::Kanji, Action::Idle),                 // KanjiHi3
    (State::Kanji, Action::Idle),                 // KanjiLo1
    (State::Kanji, Action::Idle),                 // KanjiLo2
    (State::Byte, Action::KanjiAndSingleByte),    // Byte
    // KanjiHi3 state:
    (State::Init, Action::KanjiAndSingleByte),      // End
    (State::Alpha, Action::KanjiAndSingleByte),     // Symbol
    (State::Numeric, Action::KanjiAndSingleByte),   // Numeric
    (State::Kanji, Action::Idle),                   // Alpha
    (State::Kanji, Action::Idle),                   // KanjiHi1
    (State::KanjiHi12, Action::KanjiAndSingleByte), // KanjiHi2
    (State::KanjiHi3, Action::KanjiAndSingleByte),  // KanjiHi3
    (State::Kanji, Action::Idle),                   // KanjiLo1
    (State::Byte, Action::KanjiAndSingleByte),      // KanjiLo2
    (State::Byte, Action::KanjiAndSingleByte),      // Byte
    // Kanji state:
    (State::Init, Action::Kanji),     // End
    (State::Alpha, Action::Kanji),    // Symbol
    (State::Numeric, Action::Kanji),  // Numeric
    (State::Alpha, Action::Kanji),    // Alpha
    (State::KanjiHi12, Action::Idle), // KanjiHi1
    (State::KanjiHi12, Action::Idle), // KanjiHi2
    (State::KanjiHi3, Action::Idle),  // KanjiHi3
    (State::Byte, Action::Kanji),     // KanjiLo1
    (State::Byte, Action::Kanji),     // KanjiLo2
    (State::Byte, Action::Kanji),     // Byte
];

//}}}
