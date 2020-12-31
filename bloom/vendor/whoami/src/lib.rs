//! Crate for getting the user's username, realname and environment.
//!
//! ## Getting Started
//! Using the whoami crate is super easy!  All of the public items are simple
//! functions with no parameters that return [`String`](std::string::String)s or
//! [`OsString`](std::ffi::OsString)s (with the exception of
//! [`desktop_env()`](crate::desktop_env), and [`platform()`](crate::platform)
//! which return enums).  The following example shows how to use all of the
//! functions (except those that return [`OsString`](std::ffi::OsString)):
//!
//! ```rust
//! fn main() {
//!     println!(
//!         "User's Name            whoami::realname():    {}",
//!         whoami::realname()
//!     );
//!     println!(
//!         "User's Username        whoami::username():    {}",
//!         whoami::username()
//!     );
//!     println!(
//!         "Device's Pretty Name   whoami::devicename():  {}",
//!         whoami::devicename()
//!     );
//!     println!(
//!         "Device's Hostname      whoami::hostname():    {}",
//!         whoami::hostname()
//!     );
//!     println!(
//!         "Device's Platform      whoami::platform():    {}",
//!         whoami::platform()
//!     );
//!     println!(
//!         "Device's OS Distro     whoami::distro():      {}",
//!         whoami::distro()
//!     );
//!     println!(
//!         "Device's Desktop Env.  whoami::desktop_env(): {}",
//!         whoami::desktop_env()
//!     );
//! }
//! ```

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/whoami/res/icon.svg",
    html_favicon_url = "https://libcala.github.io/whoami/res/icon.svg"
)]

use std::ffi::OsString;

/// Which Desktop Environment
#[allow(missing_docs)]
#[derive(Debug)]
#[non_exhaustive]
pub enum DesktopEnv {
    Gnome,
    /// One of the desktop environments for a specific version of Windows
    Windows,
    Lxde,
    Openbox,
    Mate,
    Xfce,
    Kde,
    Cinnamon,
    I3,
    /// Default desktop environment for MacOS
    Aqua,
    /// Desktop environment for iOS
    Ios,
    /// Desktop environment for Android
    Android,
    /// Running as Web Assembly on a web page
    WebBrowser,
    /// A desktop environment for a video game console
    Console,
    /// Ubuntu-branded GNOME
    Ubuntu,
    /// Default shell for Fuchsia
    Ermine,
    /// Default desktop environment for Redox
    Orbital,
    /// Unknown desktop environment
    Unknown(String),
}

impl std::fmt::Display for DesktopEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let DesktopEnv::Unknown(_) = self {
            write!(f, "Unknown: ")?;
        }

        write!(
            f,
            "{}",
            match self {
                DesktopEnv::Gnome => "Gnome",
                DesktopEnv::Windows => "Windows",
                DesktopEnv::Lxde => "LXDE",
                DesktopEnv::Openbox => "Openbox",
                DesktopEnv::Mate => "Mate",
                DesktopEnv::Xfce => "XFCE",
                DesktopEnv::Kde => "KDE",
                DesktopEnv::Cinnamon => "Cinnamon",
                DesktopEnv::I3 => "I3",
                DesktopEnv::Aqua => "Aqua",
                DesktopEnv::Ios => "IOS",
                DesktopEnv::Android => "Android",
                DesktopEnv::WebBrowser => "Web Browser",
                DesktopEnv::Console => "Console",
                DesktopEnv::Ubuntu => "Ubuntu",
                DesktopEnv::Ermine => "Ermine",
                DesktopEnv::Orbital => "Orbital",
                DesktopEnv::Unknown(a) => &a,
            }
        )
    }
}

/// Which Platform
#[allow(missing_docs)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Platform {
    Linux,
    Bsd,
    Windows,
    MacOS,
    Ios,
    Android,
    Nintendo,
    Xbox,
    PlayStation,
    Fuchsia,
    Redox,
    Unknown(String),
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Platform::Unknown(_) = self {
            write!(f, "Unknown: ")?;
        }

        write!(
            f,
            "{}",
            match self {
                Platform::Linux => "Linux",
                Platform::Bsd => "BSD",
                Platform::Windows => "Windows",
                Platform::MacOS => "Mac OS",
                Platform::Ios => "iOS",
                Platform::Android => "Android",
                Platform::Nintendo => "Nintendo",
                Platform::Xbox => "XBox",
                Platform::PlayStation => "PlayStation",
                Platform::Fuchsia => "Fuchsia",
                Platform::Redox => "Redox",
                Platform::Unknown(a) => a,
            }
        )
    }
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
mod windows;
#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
use self::windows as native;
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use self::wasm as native;
#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
mod unix;
#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
use self::unix as native;

/// Get the user's username.
#[inline(always)]
pub fn username() -> String {
    native::username()
}

/// Get the user's username.
#[inline(always)]
pub fn username_os() -> OsString {
    native::username_os()
}

/// Get the user's real name.
#[inline(always)]
pub fn realname() -> String {
    native::realname()
}

/// Get the user's real name.
#[inline(always)]
pub fn realname_os() -> OsString {
    native::realname_os()
}

/// Get the device name (also known as "Pretty Name"), used to identify device
/// for bluetooth pairing.
#[inline(always)]
pub fn devicename() -> String {
    native::devicename()
}

/// Get the device name (also known as "Pretty Name"), used to identify device
/// for bluetooth pairing.
#[inline(always)]
pub fn devicename_os() -> OsString {
    native::devicename_os()
}

/// Get the host device's hostname.
#[inline(always)]
pub fn hostname() -> String {
    native::hostname()
}

/// Get the host device's hostname.
#[inline(always)]
pub fn hostname_os() -> OsString {
    native::hostname_os()
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro() -> String {
    native::distro().unwrap_or_else(|| "Unknown".to_string())
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro_os() -> OsString {
    native::distro_os().unwrap_or_else(|| "Unknown".to_string().into())
}

/// Get the desktop environment.
///
/// Example: "gnome" or "windows"
#[inline(always)]
pub fn desktop_env() -> DesktopEnv {
    native::desktop_env()
}

/// Get the platform.
#[inline(always)]
pub fn platform() -> Platform {
    native::platform()
}
