use crate::protocol::{ClientSdkInfo, ClientSdkPackage};

/// The version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static::lazy_static! {
    pub static ref USER_AGENT: String = format!("sentry.rust/{}", VERSION);
    pub static ref SDK_INFO: ClientSdkInfo = ClientSdkInfo {
        name: "sentry.rust".into(),
        version: VERSION.into(),
        packages: vec![ClientSdkPackage {
            name: "cargo:sentry".into(),
            version: VERSION.into(),
        }],
        integrations: vec![],
    };
}
