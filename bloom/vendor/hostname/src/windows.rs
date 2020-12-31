use std::io;
use std::ptr;
#[cfg(feature = "set")]
use std::ffi::OsStr;
use std::ffi::OsString;
#[cfg(feature = "set")]
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;

use winapi::um::sysinfoapi;

pub fn get() -> io::Result<OsString> {
    let mut size = 0;
    unsafe {
        // Don't care much about the result here,
        // it is guaranteed to return an error,
        // since we passed the NULL pointer as a buffer
        let result = sysinfoapi::GetComputerNameExW(
            sysinfoapi::ComputerNamePhysicalDnsHostname,
            ptr::null_mut(),
            &mut size,
        );
        debug_assert_eq!(result, 0);
    };

    let mut buffer = Vec::with_capacity(size as usize);
    let result = unsafe {
        sysinfoapi::GetComputerNameExW(
            sysinfoapi::ComputerNamePhysicalDnsHostname,
            buffer.as_mut_ptr(),
            &mut size,
        )
    };

    if result == 0 {
        Err(io::Error::last_os_error())
    } else {
        unsafe {
            buffer.set_len(size as usize);
        }

        Ok(OsString::from_wide(&buffer))
    }
}

#[cfg(feature = "set")]
pub fn set(hostname: &OsStr) -> io::Result<()> {
    let buffer = hostname.encode_wide().collect::<Vec<_>>();
    let result = unsafe {
        sysinfoapi::SetComputerNameExW(
            sysinfoapi::ComputerNamePhysicalDnsHostname,
            buffer.as_ptr(),
        )
    };

    if result == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
