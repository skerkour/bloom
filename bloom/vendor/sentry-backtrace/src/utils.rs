use regex::{Captures, Regex};

lazy_static::lazy_static! {
    static ref HASH_FUNC_RE: Regex = Regex::new(r#"(?x)
        ^(.*)::h[a-f0-9]{16}$
    "#).unwrap();

    static ref CRATE_RE: Regex = Regex::new(r#"(?x)
        ^
        (?:_?<)?           # trait impl syntax
        (?:\w+\ as \ )?    # anonymous implementor
        ([a-zA-Z0-9_]+?)   # crate name
        (?:\.\.|::)        # crate delimiter (.. or ::)
    "#).unwrap();

    static ref COMMON_RUST_SYMBOL_ESCAPES_RE: Regex = Regex::new(r#"(?x)
        \$
            (SP|BP|RF|LT|GT|LP|RP|C|
                u7e|u20|u27|u5b|u5d|u7b|u7d|u3b|u2b|u22)
        \$
    "#).unwrap();
}

/// Tries to parse the rust crate from a function name.
pub fn parse_crate_name(func_name: &str) -> Option<String> {
    CRATE_RE
        .captures(func_name)
        .and_then(|caps| caps.get(1))
        .map(|cr| cr.as_str().into())
}

pub fn filename(s: &str) -> &str {
    s.rsplitn(2, &['/', '\\'][..]).next().unwrap()
}

pub fn strip_symbol(s: &str) -> &str {
    HASH_FUNC_RE
        .captures(s)
        .map(|c| c.get(1).unwrap().as_str())
        .unwrap_or(s)
}

pub fn demangle_symbol(s: &str) -> String {
    COMMON_RUST_SYMBOL_ESCAPES_RE
        .replace_all(s, |caps: &Captures<'_>| match &caps[1] {
            "SP" => "@",
            "BP" => "*",
            "RF" => "&",
            "LT" => "<",
            "GT" => ">",
            "LP" => "(",
            "RP" => ")",
            "C" => ",",
            "u7e" => "~",
            "u20" => " ",
            "u27" => "'",
            "u5b" => "[",
            "u5d" => "]",
            "u7b" => "{",
            "u7d" => "}",
            "u3b" => ";",
            "u2b" => "+",
            "u22" => "\"",
            _ => unreachable!(),
        })
        .to_string()
}

/// Checks whether the function name starts with the given pattern.
///
/// In trait implementations, the original type name is wrapped in "_< ... >" and colons are
/// replaced with dots. This function accounts for differences while checking.
pub fn function_starts_with(mut func_name: &str, mut pattern: &str) -> bool {
    if pattern.starts_with('<') {
        while pattern.starts_with('<') {
            pattern = &pattern[1..];

            if func_name.starts_with('<') {
                func_name = &func_name[1..];
            } else if func_name.starts_with("_<") {
                func_name = &func_name[2..];
            } else {
                return false;
            }
        }
    } else {
        func_name = func_name.trim_start_matches('<').trim_start_matches("_<");
    }

    if !func_name.is_char_boundary(pattern.len()) {
        return false;
    }

    func_name
        .chars()
        .zip(pattern.chars())
        .all(|(f, p)| f == p || f == '.' && p == ':')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_starts_with() {
        assert!(function_starts_with(
            "futures::task_impl::std::set",
            "futures::"
        ));

        assert!(!function_starts_with(
            "futures::task_impl::std::set",
            "tokio::"
        ));
    }

    #[test]
    fn test_function_starts_with_impl() {
        assert!(function_starts_with(
            "_<futures..task_impl..Spawn<T>>::enter::_{{closure}}",
            "futures::"
        ));

        assert!(!function_starts_with(
            "_<futures..task_impl..Spawn<T>>::enter::_{{closure}}",
            "tokio::"
        ));
    }

    #[test]
    fn test_function_starts_with_newimpl() {
        assert!(function_starts_with(
            "<futures::task_impl::Spawn<T>>::enter::{{closure}}",
            "futures::"
        ));

        assert!(!function_starts_with(
            "<futures::task_impl::Spawn<T>>::enter::{{closure}}",
            "tokio::"
        ));
    }

    #[test]
    fn test_function_starts_with_impl_pattern() {
        assert!(function_starts_with(
            "_<futures..task_impl..Spawn<T>>::enter::_{{closure}}",
            "<futures::"
        ));

        assert!(function_starts_with(
            "<futures::task_impl::Spawn<T>>::enter::{{closure}}",
            "<futures::"
        ));

        assert!(!function_starts_with(
            "futures::task_impl::std::set",
            "<futures::"
        ));
    }

    #[test]
    fn test_parse_crate_name() {
        assert_eq!(
            parse_crate_name("futures::task_impl::std::set"),
            Some("futures".into())
        );
    }

    #[test]
    fn test_parse_crate_name_impl() {
        assert_eq!(
            parse_crate_name("_<futures..task_impl..Spawn<T>>::enter::_{{closure}}"),
            Some("futures".into())
        );
    }

    #[test]
    fn test_parse_crate_name_anonymous_impl() {
        assert_eq!(
            parse_crate_name("_<F as alloc..boxed..FnBox<A>>::call_box"),
            Some("alloc".into())
        );
    }

    #[test]
    fn test_parse_crate_name_none() {
        assert_eq!(parse_crate_name("main"), None);
    }

    #[test]
    fn test_parse_crate_name_newstyle() {
        assert_eq!(
            parse_crate_name("<failure::error::Error as core::convert::From<F>>::from"),
            Some("failure".into())
        );
    }
}
