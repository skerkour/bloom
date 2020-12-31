#[macro_export]
#[doc(hidden)]
macro_rules! escape_char {
    ($($t:tt)+) => {
        pub fn escape_char(c: char, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            if c.is_ascii() {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == c as u8 {
                            return fmt.write_str($quote)
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        let c = $T[c as usize] as usize;
                        if c < $Q_LEN {
                          return fmt.write_str($Q[c]);
                        }
                    };
                }

                _inside!(impl $($t)+);
            }

            use std::fmt::Write;
            fmt.write_char(c)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! escape_char_ptr {
    ($($t:tt)+) => {
        pub unsafe fn f_escape_char(c: char, buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            let len = c.len_utf8();
            if len == 1 {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == c as u8 {
                            let mut buf_cur = 0;
                            $crate::write_ptr!(buf_cur, buf, ($quote.as_bytes() as *const _ as *const u8), $quote.len());
                            return Some(buf_cur);
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        let c = $T[c as usize] as usize;
                        if c < $Q_LEN {
                            let mut buf_cur = 0;
                            let quote = $Q[c];
                            $crate::write_ptr!(buf_cur, buf, (quote.as_bytes() as *const _ as *const u8), quote.len());
                            return Some(buf_cur);
                        }
                    };
                }

                _inside!(impl $($t)+);
                // Ascii length is one byte
                if 0 < buf.len() {
                    *buf.as_mut_ptr() = std::mem::MaybeUninit::new(c as u8);
                    Some(1)
                } else {
                    None
                }
            } else if len < buf.len() {
                // safety, encode_utf8 not read
                Some(c.encode_utf8(std::mem::transmute(buf)).len())
            } else {
                None
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! escape_char_bytes {
    ($($t:tt)+) => {
        pub unsafe fn b_escape_char<B: $crate::Buffer>(c: char, buf: &mut B) {
            let len = c.len_utf8();
            buf.reserve(len);
            if len == 1 {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == c as u8 {
                            $crate::write_bytes!($quote.as_bytes(), buf);
                            return;
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        let c = $T[c as usize] as usize;
                        if c < $Q_LEN {
                            $crate::write_bytes!($Q[c].as_bytes(), buf);
                            return;
                        }
                    };
                }

                _inside!(impl $($t)+);
                *buf.buf_ptr() = c as u8;
            } else {
                c.encode_utf8(std::slice::from_raw_parts_mut(buf.buf_ptr(), len));
            }
            buf.advance(len);
        }
    };
}
