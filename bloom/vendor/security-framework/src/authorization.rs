//! Authorization Services support.

/// # Potential improvements
///
/// * When generic specialization stabilizes prevent copying from CString
///   arguments.
/// * AuthorizationCopyRightsAsync
/// * Provide constants for well known item names
use crate::base::{Error, Result};
use core_foundation::base::{CFTypeRef, TCFType};
use core_foundation::bundle::CFBundleRef;
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::{CFString, CFStringRef};
use security_framework_sys::authorization as sys;
use security_framework_sys::base::errSecConversionError;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::raw::c_void;

macro_rules! optional_str_to_cfref {
    ($string:ident) => {{
        $string
            .map(CFString::new)
            .map_or(std::ptr::null(), |cfs| cfs.as_concrete_TypeRef())
    }};
}

macro_rules! cstring_or_err {
    ($x:expr) => {{
        CString::new($x).map_err(|_| Error::from_code(errSecConversionError))
    }};
}

bitflags::bitflags! {
    /// The flags used to specify authorization options.
    pub struct Flags: sys::AuthorizationFlags {
        /// An empty flag set that you use as a placeholder when you don't want
        /// any of the other flags.
        const DEFAULTS = sys::kAuthorizationFlagDefaults;

        /// A flag that permits user interaction as needed.
        const INTERACTION_ALLOWED = sys::kAuthorizationFlagInteractionAllowed;

        /// A flag that permits the Security Server to attempt to grant the
        /// rights requested.
        const EXTEND_RIGHTS = sys::kAuthorizationFlagExtendRights;

        /// A flag that permits the Security Server to grant rights on an
        /// individual basis.
        const PARTIAL_RIGHTS = sys::kAuthorizationFlagPartialRights;

        /// A flag that instructs the Security Server to revoke authorization.
        const DESTROY_RIGHTS = sys::kAuthorizationFlagDestroyRights;

        /// A flag that instructs the Security Server to preauthorize the rights
        /// requested.
        const PREAUTHORIZE = sys::kAuthorizationFlagPreAuthorize;
    }
}

impl Default for Flags {
    fn default() -> Flags {
        Flags::DEFAULTS
    }
}

/// Information about an authorization right or the environment.
#[repr(C)]
pub struct AuthorizationItem(sys::AuthorizationItem);

impl AuthorizationItem {
    /// The required name of the authorization right or environment data.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.name)
                .to_str()
                .expect("AuthorizationItem::name failed to convert &str to CStr")
        }
    }

    /// The information pertaining to the name field. Do not rely on NULL
    /// termination of string data.
    pub fn value(&self) -> Option<&[u8]> {
        if self.0.value.is_null() {
            return None;
        }

        let value =
            unsafe { std::slice::from_raw_parts(self.0.value as *const u8, self.0.valueLength) };

        Some(value)
    }
}

/// A set of authorization items returned and owned by the Security Server.
#[derive(Debug)]
#[repr(C)]
pub struct AuthorizationItemSet<'a> {
    inner: *const sys::AuthorizationItemSet,
    phantom: PhantomData<&'a sys::AuthorizationItemSet>,
}

impl<'a> Drop for AuthorizationItemSet<'a> {
    fn drop(&mut self) {
        unsafe {
            sys::AuthorizationFreeItemSet(self.inner as *mut sys::AuthorizationItemSet);
        }
    }
}

/// Used by `AuthorizationItemSetBuilder` to store data pointed to by
/// `sys::AuthorizationItemSet`.
#[derive(Debug)]
pub struct AuthorizationItemSetStorage {
    /// The layout of this is a little awkward because of the requirements of
    /// Apple's APIs. `items` contains pointers to data owned by `names` and
    /// `values`, so we must not modify them once `items` has been set up.
    names: Vec<CString>,
    values: Vec<Option<Vec<u8>>>,
    items: Vec<sys::AuthorizationItem>,

    /// Must not be given to APIs which would attempt to modify it.
    ///
    /// See `AuthorizationItemSet` for sets owned by the Security Server which
    /// are writable.
    pub set: sys::AuthorizationItemSet,
}

impl Default for AuthorizationItemSetStorage {
    fn default() -> Self {
        AuthorizationItemSetStorage {
            names: Vec::new(),
            values: Vec::new(),
            items: Vec::new(),
            set: sys::AuthorizationItemSet {
                count: 0,
                items: std::ptr::null_mut(),
            },
        }
    }
}

/// A convenience `AuthorizationItemSetBuilder` builder which enabled you to use
/// rust types. All names and values passed in will be copied.
#[derive(Debug, Default)]
pub struct AuthorizationItemSetBuilder {
    storage: AuthorizationItemSetStorage,
}

// Stores AuthorizationItems contiguously, and their items separately
impl AuthorizationItemSetBuilder {
    /// Creates a new `AuthorizationItemSetStore`, which simplifies creating
    /// owned vectors of `AuthorizationItem`s.
    pub fn new() -> AuthorizationItemSetBuilder {
        Default::default()
    }

    /// Adds an AuthorizationItem with the name set to a right and an empty
    /// value.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn add_right<N: Into<Vec<u8>>>(mut self, name: N) -> Result<Self> {
        self.storage.names.push(cstring_or_err!(name)?);
        self.storage.values.push(None);
        Ok(self)
    }

    /// Adds an AuthorizationItem with arbitrary data.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn add_data<N, V>(mut self, name: N, value: V) -> Result<Self>
    where
        N: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        self.storage.names.push(cstring_or_err!(name)?);
        self.storage.values.push(Some(value.into()));
        Ok(self)
    }

    /// Adds an AuthorizationItem with NULL terminated string data.
    ///
    /// If `name` or `value` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn add_string<N, V>(mut self, name: N, value: V) -> Result<Self>
    where
        N: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        self.storage.names.push(cstring_or_err!(name)?);
        self.storage
            .values
            .push(Some(cstring_or_err!(value)?.to_bytes().to_vec()));
        Ok(self)
    }

    /// Creates the `sys::AuthorizationItemSet`, and gives you ownership of the
    /// data it points to.
    pub fn build(mut self) -> AuthorizationItemSetStorage {
        self.storage.items = self
            .storage
            .names
            .iter()
            .zip(self.storage.values.iter())
            .map(|(n, v)| sys::AuthorizationItem {
                name: n.as_ptr(),
                value: v
                    .as_ref()
                    .map_or(std::ptr::null_mut(), |v| v.as_ptr() as *mut c_void),
                valueLength: v.as_ref().map_or(0, |v| v.len()),
                flags: 0,
            })
            .collect();

        self.storage.set = sys::AuthorizationItemSet {
            count: self.storage.items.len() as u32,
            items: self.storage.items.as_ptr() as *mut sys::AuthorizationItem,
        };

        self.storage
    }
}

/// Used by `Authorization::set_item` to define the rules of he right.
pub enum RightDefinition<'a> {
    /// The dictionary will contain the keys and values that define the rules.
    FromDictionary(&'a CFDictionary<CFStringRef, CFTypeRef>),

    /// The specified right's rules will be duplicated.
    FromExistingRight(&'a str),
}

/// A wrapper around AuthorizationCreate and functions which operate on an
/// AuthorizationRef.
#[derive(Debug)]
pub struct Authorization {
    handle: sys::AuthorizationRef,
    free_flags: Flags,
}

impl<'a> Authorization {
    /// Creates an authorization object which has no environment or associated
    /// rights.
    pub fn default() -> Result<Self> {
        Self::new(None, None, Default::default())
    }

    /// Creates an authorization reference and provides an option to authorize
    /// or preauthorize rights.
    ///
    /// `rights` should be the names of the rights you want to create.
    ///
    /// `environment` is used when authorizing or preauthorizing rights. Not
    /// used in OS X v10.2 and earlier. In macOS 10.3 and later, you can pass
    /// icon or prompt data to be used in the authentication dialog box. In
    /// macOS 10.4 and later, you can also pass a user name and password in
    /// order to authorize a user without user interaction.
    pub fn new(
        rights: Option<AuthorizationItemSetStorage>,
        environment: Option<AuthorizationItemSetStorage>,
        flags: Flags,
    ) -> Result<Self> {
        let rights_ptr = rights.as_ref().map_or(std::ptr::null(), |r| {
            &r.set as *const sys::AuthorizationItemSet
        });

        let env_ptr = environment.as_ref().map_or(std::ptr::null(), |e| {
            &e.set as *const sys::AuthorizationItemSet
        });

        let mut handle = MaybeUninit::<sys::AuthorizationRef>::uninit();

        let status = unsafe {
            sys::AuthorizationCreate(rights_ptr, env_ptr, flags.bits(), handle.as_mut_ptr())
        };

        if status != sys::errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        Ok(Authorization {
            handle: unsafe { handle.assume_init() },
            free_flags: Default::default(),
        })
    }

    /// Internalizes the external representation of an authorization reference.
    ///
    /// TODO: TryFrom when security-framework stops supporting rust versions
    /// which don't have it.
    pub fn from_external_form(external_form: sys::AuthorizationExternalForm) -> Result<Self> {
        let mut handle = MaybeUninit::<sys::AuthorizationRef>::uninit();

        let status = unsafe {
            sys::AuthorizationCreateFromExternalForm(&external_form, handle.as_mut_ptr())
        };

        if status != sys::errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        let auth = Authorization {
            handle: unsafe { handle.assume_init() },
            free_flags: Default::default(),
        };

        Ok(auth)
    }

    /// By default the rights acquired will be retained by the Security Server.
    /// Use this to ensure they are destroyed and to prevent shared rights'
    /// continued used by other processes.
    #[inline]
    pub fn destroy_rights(mut self) {
        self.free_flags = Flags::DESTROY_RIGHTS;
    }

    /// Retrieve's the right's definition as a dictionary. Use `right_exists`
    /// if you want to avoid retrieving the dictionary.
    ///
    /// `name` can be a wildcard right name.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn get_right<T: Into<Vec<u8>>>(name: T) -> Result<CFDictionary<CFString, CFTypeRef>> {
        let name = cstring_or_err!(name)?;
        let mut dict = MaybeUninit::<CFDictionaryRef>::uninit();

        let status = unsafe { sys::AuthorizationRightGet(name.as_ptr(), dict.as_mut_ptr()) };

        if status != sys::errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        let dict = unsafe { CFDictionary::wrap_under_create_rule(dict.assume_init()) };

        Ok(dict)
    }

    /// Checks if a right exists within the policy database. This is the same as
    /// `get_right`, but avoids a dictionary allocation.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn right_exists<T: Into<Vec<u8>>>(name: T) -> Result<bool> {
        let name = cstring_or_err!(name)?;

        let status = unsafe { sys::AuthorizationRightGet(name.as_ptr(), std::ptr::null_mut()) };

        Ok(status == sys::errAuthorizationSuccess)
    }

    /// Removes a right from the policy database.
    ///
    /// `name` cannot be a wildcard right name.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn remove_right<T: Into<Vec<u8>>>(&self, name: T) -> Result<()> {
        let name = cstring_or_err!(name)?;

        let status = unsafe { sys::AuthorizationRightRemove(self.handle, name.as_ptr()) };

        if status != sys::errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        Ok(())
    }

    /// Creates or updates a right entry in the policy database. Your process
    /// must have a code signature in order to be able to add rights to the
    /// authorization database.
    ///
    /// `name` cannot be a wildcard right.
    ///
    /// `definition` can be either a `CFDictionaryRef` containing keys defining
    /// the rules or a `CFStringRef` representing the name of another right
    /// whose rules you wish to duplicaate.
    ///
    /// `description` is a key which can be used to look up localized
    /// descriptions.
    ///
    /// `bundle` will be used to get localizations from if not the main bundle.
    ///
    /// `localeTableName` will be used to get localizations if provided.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn set_right<T: Into<Vec<u8>>>(
        &self,
        name: T,
        definition: RightDefinition<'_>,
        description: Option<&str>,
        bundle: Option<CFBundleRef>,
        locale: Option<&str>,
    ) -> Result<()> {
        let name = cstring_or_err!(name)?;

        let definition_cfstring: CFString;
        let definition_ref = match definition {
            RightDefinition::FromDictionary(def) => def.as_CFTypeRef(),
            RightDefinition::FromExistingRight(def) => {
                definition_cfstring = CFString::new(def);
                definition_cfstring.as_CFTypeRef()
            }
        };

        let status = unsafe {
            sys::AuthorizationRightSet(
                self.handle,
                name.as_ptr(),
                definition_ref,
                optional_str_to_cfref!(description),
                bundle.unwrap_or(std::ptr::null_mut()),
                optional_str_to_cfref!(locale),
            )
        };

        if status != sys::errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        Ok(())
    }

    /// An authorization plugin can store the results of an authentication
    /// operation by calling the `SetContextValue` function. You can then
    /// retrieve this supporting data, such as the user name.
    ///
    /// `tag` should specify the type of data the Security Server should return.
    /// If `None`, all available information is retreieved.
    ///
    /// If `tag` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn copy_info<T: Into<Vec<u8>>>(&self, tag: Option<T>) -> Result<AuthorizationItemSet<'_>> {
        let tag_with_nul: CString;

        let tag_ptr = match tag {
            Some(tag) => {
                tag_with_nul = cstring_or_err!(tag)?;
                tag_with_nul.as_ptr()
            }
            None => std::ptr::null(),
        };

        let mut inner = MaybeUninit::<*mut sys::AuthorizationItemSet>::uninit();

        let status =
            unsafe { sys::AuthorizationCopyInfo(self.handle, tag_ptr, inner.as_mut_ptr()) };

        if status != sys::errAuthorizationSuccess {
            return Err(Error::from(status));
        }

        let set = AuthorizationItemSet {
            inner: unsafe { inner.assume_init() },
            phantom: PhantomData,
        };

        Ok(set)
    }

    /// Creates an external representation of an authorization reference so that
    /// you can transmit it between processes.
    pub fn make_external_form(&self) -> Result<sys::AuthorizationExternalForm> {
        let mut external_form = MaybeUninit::<sys::AuthorizationExternalForm>::uninit();

        let status =
            unsafe { sys::AuthorizationMakeExternalForm(self.handle, external_form.as_mut_ptr()) };

        if status != sys::errAuthorizationSuccess {
            return Err(Error::from(status));
        }

        Ok(unsafe { external_form.assume_init() })
    }
}

impl Drop for Authorization {
    fn drop(&mut self) {
        unsafe {
            sys::AuthorizationFree(self.handle, self.free_flags.bits());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::string::CFString;

    #[test]
    fn test_create_default_authorization() {
        Authorization::default().unwrap();
    }

    #[test]
    fn test_create_allowed_authorization() -> Result<()> {
        let rights = AuthorizationItemSetBuilder::new()
            .add_right("system.hdd.smart")?
            .add_right("system.login.done")?
            .build();

        Authorization::new(Some(rights), None, Flags::EXTEND_RIGHTS).unwrap();

        Ok(())
    }

    #[test]
    fn test_create_then_destroy_allowed_authorization() -> Result<()> {
        let rights = AuthorizationItemSetBuilder::new()
            .add_right("system.hdd.smart")?
            .add_right("system.login.done")?
            .build();

        let auth = Authorization::new(Some(rights), None, Flags::EXTEND_RIGHTS).unwrap();
        auth.destroy_rights();

        Ok(())
    }

    #[test]
    fn test_create_authorization_requiring_interaction() -> Result<()> {
        let rights = AuthorizationItemSetBuilder::new()
            .add_right("system.privilege.admin")?
            .build();

        let error = Authorization::new(Some(rights), None, Flags::EXTEND_RIGHTS).unwrap_err();

        assert_eq!(error.code(), sys::errAuthorizationInteractionNotAllowed);

        Ok(())
    }

    fn create_credentials_env() -> Result<AuthorizationItemSetStorage> {
        let set = AuthorizationItemSetBuilder::new()
            .add_string(
                "username",
                option_env!("USER").expect("You must set the USER environment variable"),
            )?
            .add_string(
                "password",
                option_env!("PASSWORD").expect("You must set the PASSWORD environment varible"),
            )?
            .build();

        Ok(set)
    }

    #[test]
    fn test_create_authorization_with_bad_credentials() -> Result<()> {
        let rights = AuthorizationItemSetBuilder::new()
            .add_right("system.privilege.admin")?
            .build();

        let env = AuthorizationItemSetBuilder::new()
            .add_string("username", "Tim Apple")?
            .add_string("password", "butterfly")?
            .build();

        let error =
            Authorization::new(Some(rights), Some(env), Flags::INTERACTION_ALLOWED).unwrap_err();

        assert_eq!(error.code(), sys::errAuthorizationDenied);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_create_authorization_with_credentials() -> Result<()> {
        let rights = AuthorizationItemSetBuilder::new()
            .add_right("system.privilege.admin")?
            .build();

        let env = create_credentials_env()?;

        Authorization::new(Some(rights), Some(env), Flags::EXTEND_RIGHTS).unwrap();

        Ok(())
    }

    #[test]
    fn test_query_authorization_database() -> Result<()> {
        assert!(Authorization::right_exists("system.hdd.smart")?);
        assert!(!Authorization::right_exists("EMPTY")?);

        let dict = Authorization::get_right("system.hdd.smart").unwrap();

        let key = CFString::from_static_string("class");
        assert!(dict.contains_key(&key));

        let invalid_key = CFString::from_static_string("EMPTY");
        assert!(!dict.contains_key(&invalid_key));

        Ok(())
    }

    /// This test will only pass if its process has a valid code signature.
    #[test]
    #[ignore]
    fn test_modify_authorization_database() -> Result<()> {
        let rights = AuthorizationItemSetBuilder::new()
            .add_right("config.modify.")?
            .build();

        let env = create_credentials_env()?;

        let auth = Authorization::new(Some(rights), Some(env), Flags::EXTEND_RIGHTS).unwrap();

        assert!(!Authorization::right_exists("TEST_RIGHT")?);

        auth.set_right(
            "TEST_RIGHT",
            RightDefinition::FromExistingRight("system.hdd.smart"),
            None,
            None,
            None,
        )
        .unwrap();

        assert!(Authorization::right_exists("TEST_RIGHT")?);

        auth.remove_right("TEST_RIGHT").unwrap();

        assert!(!Authorization::right_exists("TEST_RIGHT")?);

        Ok(())
    }
}
