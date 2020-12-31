use crate::{DesktopEnv, Platform};

use std::{
    convert::TryInto,
    ffi::OsString,
    os::{
        raw::{c_char, c_int, c_uchar, c_ulong},
        windows::ffi::OsStringExt,
    },
    ptr,
};

#[allow(unused)]
#[repr(C)]
enum ExtendedNameFormat {
    Unknown,          // Nothing
    FullyQualifiedDN, // Nothing
    SamCompatible,    // Hostname Followed By Username
    Display,          // Full Name
    UniqueId,         // Nothing
    Canonical,        // Nothing
    UserPrincipal,    // Nothing
    CanonicalEx,      // Nothing
    ServicePrincipal, // Nothing
    DnsDomain,        // Nothing
    GivenName,        // Nothing
    Surname,          // Nothing
}

#[allow(unused)]
#[repr(C)]
enum ComputerNameFormat {
    NetBIOS,                   // Same as GetComputerNameW
    DnsHostname,               // Fancy Name
    DnsDomain,                 // Nothing
    DnsFullyQualified,         // Fancy Name with, for example, .com
    PhysicalNetBIOS,           // Same as GetComputerNameW
    PhysicalDnsHostname,       // Same as GetComputerNameW
    PhysicalDnsDomain,         // Nothing
    PhysicalDnsFullyQualified, // Fancy Name with, for example, .com
    Max,
}

#[link(name = "secur32")]
extern "system" {
    fn GetLastError() -> c_ulong;
    fn GetUserNameExW(
        a: ExtendedNameFormat,
        b: *mut c_char,
        c: *mut c_ulong,
    ) -> c_uchar;
    fn GetUserNameW(a: *mut c_char, b: *mut c_ulong) -> c_int;
    fn GetComputerNameExW(
        a: ComputerNameFormat,
        b: *mut c_char,
        c: *mut c_ulong,
    ) -> c_int;
}

// Convert an OsString into a String
fn string_from_os(string: OsString) -> String {
    match string.into_string() {
        Ok(string) => string,
        Err(string) => string.to_string_lossy().to_string(),
    }
}

pub fn username() -> String {
    string_from_os(username_os())
}

pub fn username_os() -> OsString {
    // Step 1. Retreive the entire length of the username
    let mut size = 0;
    let fail = unsafe {
        // Ignore error, we know that it will be ERROR_INSUFFICIENT_BUFFER
        GetUserNameW(ptr::null_mut(), &mut size) == 0
    };
    debug_assert_eq!(fail, true);

    // Step 2. Allocate memory to put the Windows (UTF-16) string.
    let mut name: Vec<u16> = Vec::with_capacity(size.try_into().unwrap());
    let orig_size = size;
    let fail =
        unsafe { GetUserNameW(name.as_mut_ptr().cast(), &mut size) == 0 };
    if fail {
        panic!(
            "Failed to get username: {}, report at https://github.com/libcala/whoami/issues",
            unsafe { GetLastError() }
        );
    }
    debug_assert_eq!(orig_size, size);
    unsafe {
        name.set_len(size.try_into().unwrap());
    }
    let terminator = name.pop(); // Remove Trailing Null
    debug_assert_eq!(terminator, Some(0u16));

    // Step 3. Convert to Rust String
    OsString::from_wide(&name)
}

#[inline(always)]
pub fn realname() -> String {
    string_from_os(realname_os())
}

#[inline(always)]
pub fn realname_os() -> OsString {
    // Step 1. Retrieve the entire length of the username
    let mut buf_size = 0;
    let fail = unsafe {
        GetUserNameExW(
            ExtendedNameFormat::Display,
            ptr::null_mut(),
            &mut buf_size,
        ) == 0
    };
    debug_assert_eq!(fail, true);
    match unsafe { GetLastError() } {
		0x00EA /* more data */ => { /* Success, continue */ }
		0x054B /* no such domain */ => {
			// If domain controller over the network can't be contacted, return
			// "Unknown".
			return "Unknown".into()
		}
		0x0534 /* none mapped */ => {
			// Fallback to username
			return username_os();
		}
		u => {
			eprintln!("Unknown error code: {}, report at https://github.com/libcala/whoami/issues", u);
			unreachable!();
		}
	}

    // Step 2. Allocate memory to put the Windows (UTF-16) string.
    let mut name: Vec<u16> = Vec::with_capacity(buf_size.try_into().unwrap());
    let mut name_len = buf_size;
    let fail = unsafe {
        GetUserNameExW(
            ExtendedNameFormat::Display,
            name.as_mut_ptr().cast(),
            &mut name_len,
        ) == 0
    };
    if fail {
        panic!(
            "Failed to get username: {}, report at https://github.com/libcala/whoami/issues",
            unsafe { GetLastError() }
        );
    }
    debug_assert_eq!(buf_size, name_len + 1);
    unsafe {
        name.set_len(name_len.try_into().unwrap());
    }

    // Step 3. Convert to Rust String
    OsString::from_wide(&name)
}

#[inline(always)]
pub fn devicename() -> String {
    string_from_os(devicename_os())
}

#[inline(always)]
pub fn devicename_os() -> OsString {
    // Step 1. Retreive the entire length of the username
    let mut size = 0;
    let fail = unsafe {
        // Ignore error, we know that it will be ERROR_INSUFFICIENT_BUFFER
        GetComputerNameExW(
            ComputerNameFormat::DnsHostname,
            ptr::null_mut(),
            &mut size,
        ) == 0
    };
    debug_assert_eq!(fail, true);

    // Step 2. Allocate memory to put the Windows (UTF-16) string.
    let mut name: Vec<u16> = Vec::with_capacity(size.try_into().unwrap());
    let fail = unsafe {
        GetComputerNameExW(
            ComputerNameFormat::DnsHostname,
            name.as_mut_ptr().cast(),
            &mut size,
        ) == 0
    };
    if fail {
        panic!(
            "Failed to get computer name: {}, report at https://github.com/libcala/whoami/issues",
            unsafe { GetLastError() }
        );
    }
    unsafe {
        name.set_len(size.try_into().unwrap());
    }

    // Step 3. Convert to Rust String
    OsString::from_wide(&name)
}

pub fn hostname() -> String {
    string_from_os(hostname_os())
}

pub fn hostname_os() -> OsString {
    // Step 1. Retreive the entire length of the username
    let mut size = 0;
    let fail = unsafe {
        // Ignore error, we know that it will be ERROR_INSUFFICIENT_BUFFER
        GetComputerNameExW(
            ComputerNameFormat::NetBIOS,
            ptr::null_mut(),
            &mut size,
        ) == 0
    };
    debug_assert_eq!(fail, true);

    // Step 2. Allocate memory to put the Windows (UTF-16) string.
    let mut name: Vec<u16> = Vec::with_capacity(size.try_into().unwrap());
    let fail = unsafe {
        GetComputerNameExW(
            ComputerNameFormat::NetBIOS,
            name.as_mut_ptr().cast(),
            &mut size,
        ) == 0
    };
    if fail {
        panic!(
            "Failed to get computer name: {}, report at https://github.com/libcala/whoami/issues",
            unsafe { GetLastError() }
        );
    }
    unsafe {
        name.set_len(size.try_into().unwrap());
    }

    // Step 3. Convert to Rust String
    OsString::from_wide(&name)
}

pub fn distro_os() -> Option<OsString> {
    distro().map(|a| a.into())
}

pub fn distro() -> Option<String> {
    extern "system" {
        fn GetVersion() -> usize;
    }

    let bits = unsafe { GetVersion() } as u32;

    let mut out = "Windows ".to_string();

    let major: u8 = (bits & 0b00000000_00000000_00000000_11111111) as u8;
    let minor: u8 = ((bits & 0b00000000_00000000_11111111_00000000) >> 8) as u8;
    let build: u16 =
        ((bits & 0b11111111_11111111_00000000_00000000) >> 16) as u16;

    match major {
        5 => out.push_str("XP"),
        6 => match minor {
            0 => out.push_str("Vista"),
            1 => out.push_str("7"),
            2 => match build {
                9200 => out.push_str("10"),
                _ => out.push_str("8"),
            },
            _ => out.push_str("8"),
        },
        _ => out.push_str("Unknown"),
    }

    Some(out)
}

#[inline(always)]
pub const fn desktop_env() -> DesktopEnv {
    DesktopEnv::Windows
}

#[inline(always)]
pub const fn platform() -> Platform {
    Platform::Windows
}
