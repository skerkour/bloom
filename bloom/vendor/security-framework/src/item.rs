//! Support to search for items in a keychain.

use core_foundation::array::CFArray;
use core_foundation::base::{CFType, TCFType};
use core_foundation::boolean::CFBoolean;
use core_foundation::data::CFData;
use core_foundation::date::CFDate;
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;
use core_foundation_sys::base::{CFCopyDescription, CFGetTypeID, CFRelease, CFTypeRef};
use core_foundation_sys::string::CFStringRef;
use security_framework_sys::item::*;
use std::collections::HashMap;
use std::fmt;
use std::ptr;

use crate::base::Result;
use crate::certificate::SecCertificate;
use crate::cvt;
use crate::identity::SecIdentity;
use crate::key::SecKey;
#[cfg(target_os = "macos")]
use crate::os::macos::keychain::SecKeychain;

/// Specifies the type of items to search for.
#[derive(Debug, Copy, Clone)]
pub struct ItemClass(CFStringRef);

impl ItemClass {
    /// Look for `SecKeychainItem`s corresponding to generic passwords.
    pub fn generic_password() -> Self {
        unsafe { Self(kSecClassGenericPassword) }
    }

    /// Look for `SecKeychainItem`s corresponding to internet passwords.
    pub fn internet_password() -> Self {
        unsafe { Self(kSecClassInternetPassword) }
    }

    /// Look for `SecCertificate`s.
    pub fn certificate() -> Self {
        unsafe { Self(kSecClassCertificate) }
    }

    /// Look for `SecKey`s.
    pub fn key() -> Self {
        unsafe { Self(kSecClassKey) }
    }

    /// Look for `SecIdentity`s.
    pub fn identity() -> Self {
        unsafe { Self(kSecClassIdentity) }
    }

    fn to_value(self) -> CFType {
        unsafe { CFType::wrap_under_get_rule(self.0 as *const _) }
    }
}

/// A builder type to search for items in keychains.
#[derive(Default)]
pub struct ItemSearchOptions {
    #[cfg(target_os = "macos")]
    keychains: Option<CFArray<SecKeychain>>,
    #[cfg(not(target_os = "macos"))]
    keychains: Option<CFArray<CFType>>,
    class: Option<ItemClass>,
    load_refs: bool,
    load_attributes: bool,
    load_data: bool,
    limit: Option<i64>,
    label: Option<CFString>,
}

#[cfg(target_os = "macos")]
impl crate::ItemSearchOptionsInternals for ItemSearchOptions {
    fn keychains(&mut self, keychains: &[SecKeychain]) -> &mut Self {
        self.keychains = Some(CFArray::from_CFTypes(keychains));
        self
    }
}

impl ItemSearchOptions {
    /// Creates a new builder with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Search only for items of the specified class.
    pub fn class(&mut self, class: ItemClass) -> &mut Self {
        self.class = Some(class);
        self
    }


    /// Load Security Framework objects (`SecCertificate`, `SecKey`, etc) for
    /// the results.
    pub fn load_refs(&mut self, load_refs: bool) -> &mut Self {
        self.load_refs = load_refs;
        self
    }

    /// Load Security Framework object attributes for
    /// the results.
    pub fn load_attributes(&mut self, load_attributes: bool) -> &mut Self {
        self.load_attributes = load_attributes;
        self
    }

    /// Load Security Framework objects data for
    /// the results.
    pub fn load_data(&mut self, load_data: bool) -> &mut Self {
        self.load_data = load_data;
        self
    }

    /// Limit the number of search results.
    ///
    /// If this is not called, the default limit is 1.
    pub fn limit(&mut self, limit: i64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Search for an item with the given label.
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = Some(CFString::new(label));
        self
    }

    /// Search for objects.
    pub fn search(&self) -> Result<Vec<SearchResult>> {
        unsafe {
            let mut params = vec![];

            if let Some(ref keychains) = self.keychains {
                params.push((
                    CFString::wrap_under_get_rule(kSecMatchSearchList),
                    keychains.as_CFType(),
                ));
            }

            if let Some(class) = self.class {
                params.push((CFString::wrap_under_get_rule(kSecClass), class.to_value()));
            }

            if self.load_refs {
                params.push((
                    CFString::wrap_under_get_rule(kSecReturnRef),
                    CFBoolean::true_value().as_CFType(),
                ));
            }

            if self.load_attributes {
                params.push((
                    CFString::wrap_under_get_rule(kSecReturnAttributes),
                    CFBoolean::true_value().as_CFType(),
                ));
            }

            if self.load_data {
                params.push((
                    CFString::wrap_under_get_rule(kSecReturnData),
                    CFBoolean::true_value().as_CFType(),
                ));
            }

            if let Some(limit) = self.limit {
                params.push((
                    CFString::wrap_under_get_rule(kSecMatchLimit),
                    CFNumber::from(limit).as_CFType(),
                ));
            }

            if let Some(ref label) = self.label {
                params.push((
                    CFString::wrap_under_get_rule(kSecAttrLabel),
                    label.as_CFType(),
                ));
            }

            let params = CFDictionary::from_CFType_pairs(&params);

            let mut ret = ptr::null();
            cvt(SecItemCopyMatching(params.as_concrete_TypeRef(), &mut ret))?;
            let type_id = CFGetTypeID(ret);

            let mut items = vec![];

            if type_id == CFArray::<CFType>::type_id() {
                let array: CFArray<CFType> = CFArray::wrap_under_create_rule(ret as *mut _);
                for item in array.iter() {
                    items.push(get_item(item.as_CFTypeRef()));
                }
            } else {
                items.push(get_item(ret));
                // This is a bit janky, but get_item uses wrap_under_get_rule
                // which bumps the refcount but we want create semantics
                CFRelease(ret);
            }

            Ok(items)
        }
    }
}

#[cfg(target_os = "macos")]
unsafe fn get_item(item: CFTypeRef) -> SearchResult {
    use crate::os::macos::keychain_item::SecKeychainItem;

    let type_id = CFGetTypeID(item);

    if type_id == CFData::type_id() {
        let data = CFData::wrap_under_get_rule(item as *mut _);
        let mut buf = Vec::new();
        buf.extend_from_slice(data.bytes());
        return SearchResult::Data(buf);
    }

    if type_id == CFDictionary::<*const u8, *const u8>::type_id() {
        return SearchResult::Dict(CFDictionary::wrap_under_get_rule(item as *mut _));
    }

    let reference = if type_id == SecCertificate::type_id() {
        Reference::Certificate(SecCertificate::wrap_under_get_rule(item as *mut _))
    } else if type_id == SecKey::type_id() {
        Reference::Key(SecKey::wrap_under_get_rule(item as *mut _))
    } else if type_id == SecIdentity::type_id() {
        Reference::Identity(SecIdentity::wrap_under_get_rule(item as *mut _))
    } else if type_id == SecKeychainItem::type_id() {
        Reference::KeychainItem(SecKeychainItem::wrap_under_get_rule(item as *mut _))
    } else {
        panic!("Got bad type from SecItemCopyMatching: {}", type_id);
    };

    SearchResult::Ref(reference)
}

#[cfg(not(target_os = "macos"))]
unsafe fn get_item(item: CFTypeRef) -> SearchResult {
    let type_id = CFGetTypeID(item);

    let reference = if type_id == SecCertificate::type_id() {
        Reference::Certificate(SecCertificate::wrap_under_get_rule(item as *mut _))
    } else if type_id == SecKey::type_id() {
        Reference::Key(SecKey::wrap_under_get_rule(item as *mut _))
    } else if type_id == SecIdentity::type_id() {
        Reference::Identity(SecIdentity::wrap_under_get_rule(item as *mut _))
    } else {
        panic!("Got bad type from SecItemCopyMatching: {}", type_id);
    };

    SearchResult::Ref(reference)
}

/// An enum including all objects which can be found by `ItemSearchOptions`.
#[derive(Debug)]
pub enum Reference {
    /// A `SecIdentity`.
    Identity(SecIdentity),
    /// A `SecCertificate`.
    Certificate(SecCertificate),
    /// A `SecKey`.
    Key(SecKey),
    /// A `SecKeychainItem`.
    ///
    /// Only defined on OSX
    #[cfg(target_os = "macos")]
    KeychainItem(crate::os::macos::keychain_item::SecKeychainItem),
    #[doc(hidden)]
    __NonExhaustive,
}

/// An individual search result.
pub enum SearchResult {
    /// A reference to the Security Framework object, if asked for.
    Ref(Reference),
    /// A dictionary of data about the Security Framework object, if asked for.
    Dict(CFDictionary),
    /// The Security Framework object as bytes, if asked for.
    Data(Vec<u8>),
    /// An unknown representation of the Security Framework object.
    Other,
}

impl fmt::Debug for SearchResult {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Ref(ref reference) => fmt
                .debug_struct("SearchResult::Ref")
                .field("reference", reference)
                .finish(),
            Self::Data(ref buf) => fmt
                .debug_struct("SearchResult::Data")
                .field("data", buf)
                .finish(),
            Self::Dict(_) => {
                let mut debug = fmt.debug_struct("SearchResult::Dict");
                for (k, v) in self.simplify_dict().unwrap() {
                    debug.field(&k, &v);
                }
                debug.finish()
            }
            Self::Other => write!(fmt, "SearchResult::Other"),
        }
    }
}

impl SearchResult {
    /// If the search result is a `CFDict`, simplify that to a
    /// `HashMap<String, String>`. This transformation isn't
    /// comprehensive, it only supports CFString, CFDate, and CFData
    /// value types.
    pub fn simplify_dict(&self) -> Option<HashMap<String, String>> {
        match *self {
            Self::Dict(ref d) => unsafe {
                let mut retmap = HashMap::new();
                let (keys, values) = d.get_keys_and_values();
                for (k, v) in keys.iter().zip(values.iter()) {
                    let keycfstr = CFString::wrap_under_get_rule(*k as *const _);
                    let val: String = match CFGetTypeID(*v) {
                        cfstring if cfstring == CFString::type_id() => {
                            format!("{}", CFString::wrap_under_get_rule(*v as *const _))
                        }
                        cfdata if cfdata == CFData::type_id() => {
                            let buf = CFData::wrap_under_get_rule(*v as *const _);
                            let mut vec = Vec::new();
                            vec.extend_from_slice(buf.bytes());
                            format!("{}", String::from_utf8_lossy(&vec))
                        }
                        cfdate if cfdate == CFDate::type_id() => format!(
                            "{}",
                            CFString::wrap_under_create_rule(CFCopyDescription(*v))
                        ),
                        _ => String::from("unknown"),
                    };
                    retmap.insert(format!("{}", keycfstr), val);
                }
                Some(retmap)
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_nothing() {
        assert!(ItemSearchOptions::new().search().is_err());
    }

    #[test]
    fn limit_two() {
        let results = ItemSearchOptions::new()
            .class(ItemClass::certificate())
            .limit(2)
            .search()
            .unwrap();
        assert_eq!(results.len(), 2);
    }
}
