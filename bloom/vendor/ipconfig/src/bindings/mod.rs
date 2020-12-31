#![allow(clippy::all)]

#[cfg(target_pointer_width = "32")]
#[path = "win32.rs"]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod platform;

#[cfg(target_pointer_width = "64")]
#[path = "win64.rs"]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod platform;

pub use self::platform::*;
