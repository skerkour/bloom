#![allow(dead_code)]

v_escape::new!(MyEscape, "60->foo");

macro_rules! test {
    ($name:ident, $escapes:expr, $escaped:expr) => {
        use std::borrow::Cow;
        use std::char::from_u32;

        fn all_utf8_less(less: &str) -> String {
            assert_eq!(less.len(), less.as_bytes().len());

            let less = less.as_bytes();
            let mut buf = String::with_capacity(204_672 - less.len());

            for i in 0..0x80u8 {
                if !less.contains(&i) {
                    buf.push(from_u32(i as u32).unwrap())
                }
            }
            for i in 0x80..0xD800 {
                buf.push(from_u32(i).unwrap());
            }
            for i in 0xE000..0x11000 {
                buf.push(from_u32(i).unwrap());
            }

            buf
        }

        let empty = "";
        let escapes = $escapes;
        let escaped = $escaped;
        let utf8: &str = &all_utf8_less($escapes);
        let empty_heap = String::new();
        let short = "foobar";
        let string_long: &str = &short.repeat(1024);
        let string = $escapes.to_string();
        let cow = Cow::Owned($escapes.to_string());

        let mut buf = String::with_capacity(escaped.len());
        for c in escapes.chars() {
            use std::fmt::Write;
            write!(buf, "{}", escape_char(c)).unwrap();
        }
        assert_eq!(buf, escaped);

        for c in utf8.chars() {
            assert_eq!(escape_char(c).to_string(), c.to_string());
        }

        assert_eq!($name::from(empty).to_string(), empty);
        assert_eq!($name::from(escapes).to_string(), escaped);
        assert_eq!(escape(&empty_heap).to_string(), empty);
        assert_eq!(escape(&cow).to_string(), escaped);
        assert_eq!(escape(&string).to_string(), escaped);
        assert_eq!(escape(&utf8).to_string(), utf8);
        assert_eq!($name::from(string_long).to_string(), string_long);
        assert_eq!(
            $name::from(escapes.repeat(1024).as_ref()).to_string(),
            escaped.repeat(1024)
        );
        assert_eq!(
            $name::from([short, escapes, short].join("").as_ref()).to_string(),
            [short, escaped, short].join("")
        );
        assert_eq!(
            $name::from([escapes, short].join("").as_ref()).to_string(),
            [escaped, short].join("")
        );
        assert_eq!(
            $name::from(["f", escapes, short].join("").as_ref()).to_string(),
            ["f", escaped, short].join("")
        );
        assert_eq!(
            $name::from(["f", escapes].join("").as_ref()).to_string(),
            ["f", escaped].join("")
        );
        assert_eq!(
            $name::from(["fo", escapes].join("").as_ref()).to_string(),
            ["fo", escaped].join("")
        );
        assert_eq!(
            $name::from(["fo", escapes, "b"].join("").as_ref()).to_string(),
            ["fo", escaped, "b"].join("")
        );
        assert_eq!(
            $name::from(escapes.repeat(2).as_ref()).to_string(),
            escaped.repeat(2)
        );
        assert_eq!(
            $name::from(escapes.repeat(3).as_ref()).to_string(),
            escaped.repeat(3)
        );
        assert_eq!(
            $name::from(["f", &escapes.repeat(2)].join("").as_ref()).to_string(),
            ["f", &escaped.repeat(2)].join("")
        );
        assert_eq!(
            $name::from(["fo", &escapes.repeat(2)].join("").as_ref()).to_string(),
            ["fo", &escaped.repeat(2)].join("")
        );
        assert_eq!(
            $name::from(["fo", &escapes.repeat(2), "bar"].join("").as_ref()).to_string(),
            ["fo", &escaped.repeat(2), "bar"].join("")
        );
        assert_eq!(
            $name::from(["fo", &escapes.repeat(3), "bar"].join("").as_ref()).to_string(),
            ["fo", &escaped.repeat(3), "bar"].join("")
        );
        assert_eq!(
            $name::from([&escapes.repeat(3), "bar"].join("").as_ref()).to_string(),
            [&escaped.repeat(3), "bar"].join("")
        );
        assert_eq!(
            $name::from([short, &escapes.repeat(3), "bar"].join("").as_ref()).to_string(),
            [short, &escaped.repeat(3), "bar"].join("")
        );
        assert_eq!(
            $name::from([short, &escapes.repeat(5), "bar"].join("").as_ref()).to_string(),
            [short, &escaped.repeat(5), "bar"].join("")
        );
        assert_eq!(
            $name::from(
                [string_long, &escapes.repeat(13)]
                    .join("")
                    .repeat(1024)
                    .as_ref()
            )
            .to_string(),
            [string_long, &escaped.repeat(13)].join("").repeat(1024)
        );
        assert_eq!(
            $name::from([utf8, escapes, short].join("").as_ref()).to_string(),
            [utf8, escaped, short].join("")
        );
        assert_eq!(
            $name::from([utf8, escapes, utf8].join("").as_ref()).to_string(),
            [utf8, escaped, utf8].join("")
        );
        assert_eq!(
            $name::from([&utf8.repeat(124), escapes, utf8].join("").as_ref()).to_string(),
            [&utf8.repeat(124), escaped, utf8].join("")
        );
        assert_eq!(
            $name::from(
                [escapes, &utf8.repeat(124), escapes, utf8]
                    .join("")
                    .as_ref()
            )
            .to_string(),
            [escaped, &utf8.repeat(124), escaped, utf8].join("")
        );
        assert_eq!(
            $name::from(
                [escapes, &utf8.repeat(124), escapes, utf8, escapes]
                    .join("")
                    .as_ref()
            )
            .to_string(),
            [escaped, &utf8.repeat(124), escaped, utf8, escaped].join("")
        );
    };
}

macro_rules! maybe_init {
    ($b:ident, $l:expr) => {
        unsafe { from_raw_parts(&$b as *const _ as *const u8, $l) }
    };
}

macro_rules! test_ptr {
    ($escapes:expr, $escaped:expr) => {{
        use std::mem::MaybeUninit;
        use std::slice::from_raw_parts;

        let empty = "";
        let escapes = $escapes;
        let escaped = $escaped;
        let short = "foobar";
        let long = "foobar".repeat(100);
        let mix = long.clone() + escapes + short + &long;
        let mix_escaped = long.clone() + escaped + short + &long;
        let mix_2 = long.repeat(3) + &escapes.repeat(3) + short + &escapes.repeat(2) + &long;
        let mix_escaped_2 =
            long.repeat(3) + &escaped.repeat(3) + short + &escaped.repeat(2) + &long;
        let mut buf = [MaybeUninit::uninit(); 2048];
        assert_eq!(f_escape(empty.as_bytes(), &mut buf), Some(empty.len()));
        assert_eq!(f_escape(short.as_bytes(), &mut buf), Some(short.len()));
        assert_eq!(maybe_init!(buf, short.len()), short.as_bytes());
        let mut buf = [MaybeUninit::uninit(); 2048];
        assert_eq!(f_escape(long.as_bytes(), &mut buf), Some(long.len()));
        assert_eq!(maybe_init!(buf, long.len()), long.as_bytes());
        let mut buf = [MaybeUninit::uninit(); 2048];
        assert_eq!(f_escape(escapes.as_bytes(), &mut buf), Some(escaped.len()));
        assert_eq!(maybe_init!(buf, escaped.len()), escaped.as_bytes());
        let mut buf = [MaybeUninit::uninit(); 2048];
        assert_eq!(f_escape(mix.as_bytes(), &mut buf), Some(mix_escaped.len()));
        assert_eq!(maybe_init!(buf, mix_escaped.len()), mix_escaped.as_bytes());

        let mut buf = [MaybeUninit::uninit(); 10240];
        assert_eq!(
            f_escape(mix_2.as_bytes(), &mut buf),
            Some(mix_escaped_2.len())
        );
        assert_eq!(
            maybe_init!(buf, mix_escaped_2.len()),
            mix_escaped_2.as_bytes()
        );

        let mut buf = [MaybeUninit::uninit(); 2048];
        let mut cur = 0;
        for c in escapes.chars() {
            if let Some(i) = f_escape_char(c, &mut buf[cur..]) {
                cur += i;
            } else {
                panic!("overflow");
            }
        }
        assert_eq!(maybe_init!(buf, escaped.len()), escaped.as_bytes());
        let mut buf = [MaybeUninit::uninit(); 0];
        assert_eq!(f_escape(empty.as_bytes(), &mut buf), Some(empty.len()));
        assert_eq!(f_escape(short.as_bytes(), &mut buf), None);
        let mut buf = [MaybeUninit::uninit(); 599];
        assert_eq!(f_escape(long.as_bytes(), &mut buf), None);
        let mut buf = [MaybeUninit::uninit(); 600];
        assert_eq!(f_escape(long.as_bytes(), &mut buf), Some(long.len()));
        assert_eq!(maybe_init!(buf, long.len()), long.as_bytes());
    }};
}

#[test]
fn test_escape() {
    test!(MyEscape, "<", "foo");
    test_ptr!("<", "foo")
}

mod bytes_buff {
    v_escape::new!(MyE, "65->a || 60->b || 61->c || 66->d || 80->e || 81->f");
    #[test]
    fn test_escape() {
        use bytes::BytesMut;

        let empty = "";
        let escapes = "<=ABPQ";
        let escaped = "bcadef";
        let short = "foobar";
        let long = "foobar".repeat(100);
        let mix = long.clone() + escapes + short + &long;
        let mix_escaped = long.clone() + escaped + short + &long;
        let mix_2 = long.repeat(3) + &escapes.repeat(3) + short + &escapes.repeat(2) + &long;
        let mix_escaped_2 =
            long.repeat(3) + &escaped.repeat(3) + short + &escaped.repeat(2) + &long;
        let mut buf = BytesMut::new();
        b_escape(empty.as_bytes(), &mut buf);
        assert_eq!(buf.len(), 0);
        b_escape(short.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), short.as_bytes());
        let mut buf = BytesMut::new();
        b_escape(long.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), long.as_bytes());
        let mut buf = BytesMut::new();
        b_escape(escapes.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), escaped.as_bytes());
        let mut buf = BytesMut::new();
        b_escape(mix.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), mix_escaped.as_bytes());

        let mut buf = BytesMut::new();
        b_escape(mix_2.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), mix_escaped_2.as_bytes());

        let mut buf = BytesMut::with_capacity(4);
        for c in escapes.chars() {
            b_escape_char(c, &mut buf);
        }
        assert_eq!(buf.as_ref(), escaped.as_bytes());
        let mut buf = BytesMut::with_capacity(0);

        b_escape_char('\u{3A3}', &mut buf);
        assert_eq!(buf.as_ref(), "\u{3A3}".as_bytes())
    }
}

mod bytes_buff_nosimd {
    v_escape::new!(
        MyE,
        "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
        simd = false
    );
    #[test]
    fn test_escape() {
        use bytes::BytesMut;

        let empty = "";
        let escapes = "<=ABPQ";
        let escaped = "bcadef";
        let short = "foobar";
        let long = "foobar".repeat(100);
        let mix = long.clone() + escapes + short + &long;
        let mix_escaped = long.clone() + escaped + short + &long;
        let mix_2 = long.repeat(3) + &escapes.repeat(3) + short + &escapes.repeat(2) + &long;
        let mix_escaped_2 =
            long.repeat(3) + &escaped.repeat(3) + short + &escaped.repeat(2) + &long;
        let mut buf = BytesMut::new();
        b_escape(empty.as_bytes(), &mut buf);
        assert_eq!(buf.len(), 0);
        b_escape(short.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), short.as_bytes());
        let mut buf = BytesMut::new();
        b_escape(long.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), long.as_bytes());
        let mut buf = BytesMut::new();
        b_escape(escapes.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), escaped.as_bytes());
        let mut buf = BytesMut::new();
        b_escape(mix.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), mix_escaped.as_bytes());

        let mut buf = BytesMut::new();
        b_escape(mix_2.as_bytes(), &mut buf);
        assert_eq!(buf.as_ref(), mix_escaped_2.as_bytes());

        let mut buf = BytesMut::with_capacity(4);
        for c in escapes.chars() {
            b_escape_char(c, &mut buf);
        }
        assert_eq!(buf.as_ref(), escaped.as_bytes());
        let mut buf = BytesMut::with_capacity(0);

        b_escape_char('\u{3A3}', &mut buf);
        assert_eq!(buf.as_ref(), "\u{3A3}".as_bytes())
    }
}

mod no_simd {
    mod a {
        v_escape::new!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            simd = false
        );

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
            test_ptr!("<=ABPQ", "bcadef");
        }
    }

    mod b {
        v_escape::new!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            simd = false
        );

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
            test_ptr!("<=ABPQ", "bcadef");
        }
    }
}

#[cfg(target_arch = "x86_64")]
mod no_avx {
    mod a {
        v_escape::new!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            avx = false
        );

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
            test_ptr!("<=ABPQ", "bcadef");
        }
    }

    mod b {
        v_escape::new!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            avx = false
        );

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
            test_ptr!("<=ABPQ", "bcadef");
        }
    }
}

mod empty {
    v_escape::new!(MyE, "65->");

    #[test]
    fn test_escape() {
        test!(MyE, "A", "");
        test_ptr!("A", "");
    }
}

#[cfg(target_arch = "x86_64")]
mod test_avx {
    mod numbers {
        v_escape::new!(
            MyE,
            "#0->zero || #1->one || #2->two || #3->three || #4->four || #5->five || \
             #6->six || #7->seven || #8->eight || #9->nine"
        );

        #[test]
        fn test_escape_a() {
            test!(
                MyE,
                "0123456789",
                "zeroonetwothreefourfivesixseveneightnine"
            );
            test_ptr!("0123456789", "zeroonetwothreefourfivesixseveneightnine");
        }

        #[test]
        fn test_escape_b() {
            test!(
                MyE,
                "0 1-2 3-4 56789",
                "zero one-two three-four fivesixseveneightnine"
            );
            test_ptr!(
                "0 1-2 3-4 56789",
                "zero one-two three-four fivesixseveneightnine"
            );
        }
    }

    mod a {
        // 3 ranges
        v_escape::new!(MyE, "65->a || 60->b || 61->c || 66->d || 80->e || 81->f");

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
            test_ptr!("<=ABPQ", "bcadef");
        }
    }

    mod b {
        // 2 ranges and 1 escape
        v_escape::new!(MyE, "60->a || 61->b || 65->c || 80->d || 81->e");

        #[test]
        fn test_escape() {
            test!(MyE, "<=APQ", "abcde");
            test_ptr!("<=APQ", "abcde");
        }
    }

    mod c {
        // 1 range and 2 escapes
        v_escape::new!(MyE, "60->a || 65->c || 80->d || 62->e");

        #[test]
        fn test_escape() {
            test!(MyE, "<>AP", "aecd");
            test_ptr!("<>AP", "aecd");
        }
    }

    mod d {
        // 3 escapes
        v_escape::new!(MyE, "60->a || 80->b || 65->c");

        #[test]
        fn test_escape() {
            test!(MyE, "<AP", "acb");
            test_ptr!("<AP", "acb");
        }
    }

    mod e {
        // 2 ranges
        v_escape::new!(MyE, "60->a || 61->b || 81->c || 80->d || 62->e");

        #[test]
        fn test_escape() {
            test!(MyE, "<=>PQ", "abedc");
            test_ptr!("<=>PQ", "abedc");
        }
    }

    mod f {
        // 1 range and 1 escape
        v_escape::new!(MyE, "60->a || 61->b || 80->c || 62->d");

        #[test]
        fn test_escape() {
            test!(MyE, "<=>P", "abdc");
            test_ptr!("<=>P", "abdc");
        }
    }

    mod g {
        // 2 escapes
        v_escape::new!(MyE, "60->a || 80->b");

        #[test]
        fn test_escape() {
            test!(MyE, "<P", "ab");
            test_ptr!("<P", "ab");
        }
    }

    mod h {
        // 1 range
        v_escape::new!(MyE, "60->a || 61->b");

        #[test]
        fn test_escape() {
            test!(MyE, "<=", "ab");
            test_ptr!("<=", "ab");
        }
    }

    mod i {
        // 1 escapes
        v_escape::new!(MyE, "60->f");

        #[test]
        fn test_escape() {
            test!(MyE, "<", "f");
            test_ptr!("<", "f");
        }
    }
}

mod char_syntax {
    mod a {
        v_escape::new!(MyE, " ->f");

        #[test]
        fn test_escape() {
            test!(MyE, " ", "f");
            test_ptr!(" ", "f");
        }
    }
}
