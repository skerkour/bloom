use sentry_core::protocol::{Context, DeviceContext, Map, OsContext, RuntimeContext};

include!(concat!(env!("OUT_DIR"), "/constants.gen.rs"));

#[cfg(target_os = "macos")]
mod model_support {
    use libc::c_void;
    use regex::Regex;
    use std::ptr;

    lazy_static::lazy_static! {
        static ref FAMILY_RE: Regex = Regex::new(r#"([a-zA-Z]+)\d"#).unwrap();
    }

    pub fn get_model() -> Option<String> {
        unsafe {
            let mut size = 0;
            libc::sysctlbyname(
                "hw.model\x00".as_ptr() as *const i8,
                ptr::null_mut(),
                &mut size,
                ptr::null_mut(),
                0,
            );
            let mut buf = vec![0u8; size as usize];
            libc::sysctlbyname(
                "hw.model\x00".as_ptr() as *const i8,
                buf.as_mut_ptr() as *mut c_void,
                &mut size,
                ptr::null_mut(),
                0,
            );
            Some(
                String::from_utf8_lossy(if buf.ends_with(b"\x00") {
                    &buf[..size - 1]
                } else {
                    &buf
                })
                .to_string(),
            )
        }
    }

    pub fn get_family() -> Option<String> {
        get_model()
            .as_ref()
            .and_then(|model| FAMILY_RE.captures(model))
            .and_then(|m| m.get(1))
            .map(|group| group.as_str().to_string())
    }
}

#[cfg(not(target_os = "macos"))]
mod model_support {
    pub fn get_model() -> Option<String> {
        None
    }

    pub fn get_family() -> Option<String> {
        None
    }
}

/// Returns the server name (hostname) if available.
pub fn server_name() -> Option<String> {
    hostname::get().ok().and_then(|s| s.into_string().ok())
}

/// Returns the OS context
pub fn os_context() -> Option<Context> {
    #[cfg(not(windows))]
    {
        use uname::uname;
        if let Ok(info) = uname() {
            Some(
                OsContext {
                    name: Some(info.sysname),
                    kernel_version: Some(info.version),
                    version: Some(info.release),
                    ..Default::default()
                }
                .into(),
            )
        } else {
            None
        }
    }
    #[cfg(windows)]
    {
        Some(
            OsContext {
                name: Some(PLATFORM.into()),
                ..Default::default()
            }
            .into(),
        )
    }
}

/// Returns the rust info.
pub fn rust_context() -> Context {
    RuntimeContext {
        name: Some("rustc".into()),
        version: RUSTC_VERSION.map(|x| x.into()),
        other: {
            let mut map = Map::default();
            if let Some(channel) = RUSTC_CHANNEL {
                map.insert("channel".to_string(), channel.into());
            }
            map
        },
    }
    .into()
}

/// Returns the device context.
pub fn device_context() -> Context {
    DeviceContext {
        model: model_support::get_model(),
        family: model_support::get_family(),
        arch: Some(ARCH.into()),
        ..Default::default()
    }
    .into()
}
