use std::{convert::TryInto, i8, str};

use nom::{AsBytes, Needed};

type Input<'a> = nom::types::CompleteByteSlice<'a>;

#[allow(non_snake_case)]
fn Input(input: &[u8]) -> Input {
    nom::types::CompleteByteSlice(input)
}

#[derive(Debug, PartialEq)]
pub struct Pair<'a> {
    pub char: u8,
    pub quote: &'a [u8],
}

impl<'a> Pair<'a> {
    pub fn new(char: u8, quote: &[u8]) -> Pair {
        Pair { char, quote }
    }
}

named!(parse_syntax<Input, Vec<Pair>>, many1!(parse_pair));

named!(parse_pair<Input, Pair>, map!(
    separated_pair!(is_char, tag!("->"), alt!(take_until_and_consume!(" || ") | nom::rest)),
    |s| Pair::new(s.0, &s.1)
));

macro_rules! is_digit {
    ($name:ident, $base:expr) => {
        fn $name(s: Input) -> Result<u8, nom::Err<Input>> {
            if let Ok(n) = i8::from_str_radix(
                str::from_utf8(&s.as_bytes())
                    .map_err(|_| nom::Err::Failure(error_position!(s, ERR_UTF8)))?,
                $base,
            ) {
                n.try_into()
                    .map_err(|_| nom::Err::Failure(error_position!(s, ERR_OVERFLOW)))
            } else {
                Err(nom::Err::Failure(error_position!(s, ERR_OVERFLOW)))
            }
        }
    };
}

is_digit!(is_digit_8, 8);
is_digit!(is_digit_10, 10);
is_digit!(is_digit_16, 16);

fn try_into_i8(s: Input) -> Result<u8, nom::Err<Input>> {
    let b = s.as_bytes();
    if b.len() == 1 {
        b[0].try_into()
            // It is panic-free since previously it was an u8
            .map(|n: i8| n as u8)
            .map_err(|_| nom::Err::Failure(error_position!(s, ERR_OVERFLOW)))
    } else {
        Err(nom::Err::Incomplete(Needed::Size(1)))
    }
}

named!(is_char<Input, u8>, alt!(
    map_res!(preceded!(tag!("0x"), take_while1!(nom::is_hex_digit)), is_digit_16) |
    map_res!(preceded!(tag!("0o"), take_while1!(nom::is_oct_digit)), is_digit_8) |
    map_res!(preceded!(tag!("#"), take_while1!(nom::is_digit)), try_into_i8) |
    map_res!(take_while1!(nom::is_digit), is_digit_10) |
    map_res!(take!(1), try_into_i8)
));

pub fn parse(src: &str) -> Vec<Pair> {
    let mut pairs = match parse_syntax(Input(src.as_bytes())) {
        Ok((left, res)) => {
            if !left.is_empty() {
                let s = str::from_utf8(left.0).unwrap();
                panic!("Unable to parse syntax:\n\n{:?}", s);
            } else {
                res
            }
        }
        Err(nom::Err::Error(err)) => panic!("Unable to parse pairs parameter:\n\n{:?}", err),
        Err(nom::Err::Failure(err)) => match err.clone().into_error_kind() {
            ERR_OVERFLOW => panic!(
                "Number has to be between 0 and 127.\nOverflow at character:\n\n{:?}",
                err
            ),
            ERR_UTF8 => panic!("Need valid utf-8 characters.\n\n{:?}", err),
            _ => panic!("Unable to parse pairs parameter:\n\n{:?}", err),
        },
        Err(nom::Err::Incomplete(err)) => panic!("Parsing incomplete: {:?}", err),
    };

    let len = pairs.len();

    // check minimum length
    assert_ne!(len, 0);

    // need order for calculate ranges
    pairs.sort_unstable_by_key(|p| p.char);

    // check repeated
    for i in 0..len - 1 {
        let p1 = &pairs[i];
        let p2 = &pairs[i + 1];
        if p1.char == p2.char {
            panic!("{:?} and {:?} are repeated", p1, p2);
        }
    }

    pairs
}

const ERR_OVERFLOW: nom::ErrorKind = nom::ErrorKind::Custom(0);
const ERR_UTF8: nom::ErrorKind = nom::ErrorKind::Custom(1);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse("123->&lt; || "), vec![Pair::new(123, b"&lt;")]);
        assert_eq!(
            parse("123->&lt; || 10->&  || "),
            vec![Pair::new(10, b"& "), Pair::new(123, b"&lt;"),]
        );
    }

    #[test]
    fn test_syntax() {
        assert_eq!(parse("b->& || "), vec![Pair::new(b'b', b"&")]);
        assert_eq!(parse("b->&"), vec![Pair::new(b'b', b"&")]);
        assert_eq!(parse("#->& || "), vec![Pair::new(b'#', b"&")]);

        assert_eq!(parse("#6->& || "), vec![Pair::new(b'6', b"&")]);
        assert_eq!(parse("0x34->& || "), vec![Pair::new(0x34, b"&")]);
        assert_eq!(parse("0o34->& || "), vec![Pair::new(0o34, b"&")]);

        assert_eq!(parse(" ->- || "), vec![Pair::new(b' ', b"-")]);
        assert_eq!(
            parse("<->& || >->- || "),
            vec![Pair::new(b'<', b"&"), Pair::new(b'>', b"-"),]
        );
        assert_eq!(
            parse("\"->& || a->- || "),
            vec![Pair::new(b'"', b"&"), Pair::new(b'a', b"-"),]
        );
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_a() {
        parse("-f");
    }

    #[should_panic]
    #[test]
    fn test_panic_empty() {
        parse("");
    }

    #[should_panic]
    #[test]
    fn test_panic_repeated() {
        parse("a->f || a->");
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_b() {
        parse("->f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_c() {
        parse("1>f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_d() {
        parse("1-f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_e() {
        parse("1-f ||");
    }

    #[should_panic]
    #[test]
    fn test_panic_overflow_u8() {
        parse("256->f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_overflow_i8() {
        parse("128->f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_overflow_negative() {
        parse("-1->f || ");
    }
}
