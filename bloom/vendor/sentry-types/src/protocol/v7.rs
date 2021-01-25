//! The current latest sentry protocol version.
//!
//! Most constructs in the protocol map directly to types here but some
//! cleanup by renaming attributes has been applied.  The idea here is that
//! a future sentry protocol will be a cleanup of the old one and is mapped
//! to similar values on the rust side.
#![allow(clippy::trivially_copy_pass_by_ref)]

use std::borrow::Cow;
use std::cmp;
use std::fmt;
use std::iter::FromIterator;
use std::net::{AddrParseError, IpAddr};
use std::ops;
use std::str;

use ::debugid::DebugId;
use chrono::{DateTime, Utc};
use serde::Serializer;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

use crate::utils::ts_seconds_float;

pub use super::envelope::*;
pub use super::session::*;

/// An arbitrary (JSON) value.
pub mod value {
    pub use serde_json::value::{from_value, to_value, Index, Map, Number, Value};
}

/// The internally used arbitrary data map type.
pub mod map {
    pub use std::collections::btree_map::{BTreeMap as Map, *};
}

/// Represents a debug ID.
pub mod debugid {
    pub use debugid::{BreakpadFormat, DebugId, ParseDebugIdError};
}

/// An arbitrary (JSON) value.
pub use self::value::Value;

/// The internally useed map type.
pub use self::map::Map;

/// A wrapper type for collections with attached meta data.
///
/// The JSON payload can either directly be an array or an object containing a `values` field and
/// arbitrary other fields. All other fields will be collected into `Values::data` when
/// deserializing and re-serialized in the same place. The shorthand array notation is always
/// reserialized as object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Values<T> {
    /// The values of the collection.
    pub values: Vec<T>,
}

impl<T> Values<T> {
    /// Creates an empty values struct.
    pub fn new() -> Values<T> {
        Values { values: Vec::new() }
    }

    /// Checks whether this struct is empty in both values and data.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<T> Default for Values<T> {
    fn default() -> Self {
        // Default implemented manually even if <T> does not impl Default.
        Values::new()
    }
}

impl<T> From<Vec<T>> for Values<T> {
    fn from(values: Vec<T>) -> Self {
        Values { values }
    }
}

impl<T> AsRef<[T]> for Values<T> {
    fn as_ref(&self) -> &[T] {
        &self.values
    }
}

impl<T> AsMut<Vec<T>> for Values<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.values
    }
}

impl<T> ops::Deref for Values<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<T> ops::DerefMut for Values<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl<T> FromIterator<T> for Values<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Vec::<T>::from_iter(iter).into()
    }
}

impl<T> Extend<T> for Values<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.values.extend(iter)
    }
}

impl<'a, T> IntoIterator for &'a mut Values<T> {
    type Item = <&'a mut Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}

impl<'a, T> IntoIterator for &'a Values<T> {
    type Item = <&'a Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<T> IntoIterator for Values<T> {
    type Item = <Vec<T> as IntoIterator>::Item;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

/// Represents a log entry message.
///
/// A log message is similar to the `message` attribute on the event itself but
/// can additionally hold optional parameters.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LogEntry {
    /// The log message with parameters replaced by `%s`
    pub message: String,
    /// Positional parameters to be inserted into the log entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<Value>,
}

/// Represents a frame.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct Frame {
    /// The name of the function is known.
    ///
    /// Note that this might include the name of a class as well if that makes
    /// sense for the language.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// The potentially mangled name of the symbol as it appears in an executable.
    ///
    /// This is different from a function name by generally being the mangled
    /// name that appears natively in the binary.  This is relevant for languages
    /// like Swift, C++ or Rust.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// The name of the module the frame is contained in.
    ///
    /// Note that this might also include a class name if that is something the
    /// language natively considers to be part of the stack (for instance in Java).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// The name of the package that contains the frame.
    ///
    /// For instance this can be a dylib for native languages, the name of the jar
    /// or .NET assembly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    /// The filename (basename only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// If known the absolute path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abs_path: Option<String>,
    /// The line number if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineno: Option<u64>,
    /// The column number if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colno: Option<u64>,
    /// The sources of the lines leading up to the current line.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_context: Vec<String>,
    /// The current line as source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_line: Option<String>,
    /// The sources of the lines after the current line.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub post_context: Vec<String>,
    /// In-app indicator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_app: Option<bool>,
    /// Optional local variables.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub vars: Map<String, Value>,
    /// If known the location of the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_addr: Option<Addr>,
    /// If known the location of the instruction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instruction_addr: Option<Addr>,
    /// If known the location of symbol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_addr: Option<Addr>,
}

/// Represents template debug info.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct TemplateInfo {
    /// The filename (basename only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// If known the absolute path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abs_path: Option<String>,
    /// The line number if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineno: Option<u64>,
    /// The column number if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colno: Option<u64>,
    /// The sources of the lines leading up to the current line.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_context: Vec<String>,
    /// The current line as source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_line: Option<String>,
    /// The sources of the lines after the current line.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub post_context: Vec<String>,
}

/// Represents a stacktrace.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Stacktrace {
    /// The list of frames in the stacktrace.
    #[serde(default)]
    pub frames: Vec<Frame>,
    /// Optionally a segment of frames removed (`start`, `end`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frames_omitted: Option<(u64, u64)>,
    /// Optional register values of the thread.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub registers: Map<String, RegVal>,
}

impl Stacktrace {
    /// Optionally creates a stacktrace from a list of stack frames.
    pub fn from_frames_reversed(mut frames: Vec<Frame>) -> Option<Stacktrace> {
        if frames.is_empty() {
            None
        } else {
            frames.reverse();
            Some(Stacktrace {
                frames,
                ..Default::default()
            })
        }
    }
}

/// Represents a thread id.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[serde(untagged)]
pub enum ThreadId {
    /// Integer representation for the thread id
    Int(u64),
    /// String representation for the thread id
    String(String),
}

impl Default for ThreadId {
    fn default() -> ThreadId {
        ThreadId::Int(0)
    }
}

impl<'a> From<&'a str> for ThreadId {
    fn from(id: &'a str) -> ThreadId {
        ThreadId::String(id.to_string())
    }
}

impl From<String> for ThreadId {
    fn from(id: String) -> ThreadId {
        ThreadId::String(id)
    }
}

impl From<i64> for ThreadId {
    fn from(id: i64) -> ThreadId {
        ThreadId::Int(id as u64)
    }
}

impl From<i32> for ThreadId {
    fn from(id: i32) -> ThreadId {
        ThreadId::Int(id as u64)
    }
}

impl From<u32> for ThreadId {
    fn from(id: u32) -> ThreadId {
        ThreadId::Int(id as u64)
    }
}

impl From<u16> for ThreadId {
    fn from(id: u16) -> ThreadId {
        ThreadId::Int(id as u64)
    }
}

impl fmt::Display for ThreadId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ThreadId::Int(i) => write!(f, "{}", i),
            ThreadId::String(ref s) => write!(f, "{}", s),
        }
    }
}

/// Represents an address.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Addr(pub u64);

impl Addr {
    /// Returns `true` if this address is the null pointer.
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

impl_hex_serde!(Addr, u64);

impl From<u64> for Addr {
    fn from(addr: u64) -> Addr {
        Addr(addr)
    }
}

impl From<i32> for Addr {
    fn from(addr: i32) -> Addr {
        Addr(addr as u64)
    }
}

impl From<u32> for Addr {
    fn from(addr: u32) -> Addr {
        Addr(addr as u64)
    }
}

impl From<usize> for Addr {
    fn from(addr: usize) -> Addr {
        Addr(addr as u64)
    }
}

impl<T> From<*const T> for Addr {
    fn from(addr: *const T) -> Addr {
        Addr(addr as u64)
    }
}

impl<T> From<*mut T> for Addr {
    fn from(addr: *mut T) -> Addr {
        Addr(addr as u64)
    }
}

impl Into<u64> for Addr {
    fn into(self) -> u64 {
        self.0
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

/// Represents a register value.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct RegVal(pub u64);

impl_hex_serde!(RegVal, u64);

impl From<u64> for RegVal {
    fn from(addr: u64) -> RegVal {
        RegVal(addr)
    }
}

impl From<i32> for RegVal {
    fn from(addr: i32) -> RegVal {
        RegVal(addr as u64)
    }
}

impl From<u32> for RegVal {
    fn from(addr: u32) -> RegVal {
        RegVal(addr as u64)
    }
}

impl From<usize> for RegVal {
    fn from(addr: usize) -> RegVal {
        RegVal(addr as u64)
    }
}

impl<T> From<*const T> for RegVal {
    fn from(addr: *const T) -> RegVal {
        RegVal(addr as u64)
    }
}

impl<T> From<*mut T> for RegVal {
    fn from(addr: *mut T) -> RegVal {
        RegVal(addr as u64)
    }
}

impl Into<u64> for RegVal {
    fn into(self) -> u64 {
        self.0
    }
}

/// Represents a single thread.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Thread {
    /// The optional ID of the thread (usually an integer)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ThreadId>,
    /// The optional name of the thread.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If the thread suspended or crashed a stacktrace can be
    /// attached here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stacktrace: Option<Stacktrace>,
    /// Optional raw stacktrace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_stacktrace: Option<Stacktrace>,
    /// True if this is the crashed thread.
    #[serde(default, skip_serializing_if = "is_false")]
    pub crashed: bool,
    /// Indicates that the thread was not suspended when the
    /// event was created.
    #[serde(default, skip_serializing_if = "is_false")]
    pub current: bool,
}

/// POSIX signal with optional extended data.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct CError {
    /// The error code as specified by ISO C99, POSIX.1-2001 or POSIX.1-2008.
    pub number: i32,
    /// Optional name of the errno constant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<i32> for CError {
    fn from(number: i32) -> CError {
        CError { number, name: None }
    }
}

impl Into<i32> for CError {
    fn into(self) -> i32 {
        self.number
    }
}

/// Mach exception information.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct MachException {
    /// The mach exception type.
    pub exception: i32,
    /// The mach exception code.
    pub code: u64,
    /// The mach exception subcode.
    pub subcode: u64,
    /// Optional name of the mach exception.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// POSIX signal with optional extended data.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct PosixSignal {
    /// The POSIX signal number.
    pub number: i32,
    /// An optional signal code present on Apple systems.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Optional name of the errno constant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional name of the errno constant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_name: Option<String>,
}

impl From<i32> for PosixSignal {
    fn from(number: i32) -> PosixSignal {
        PosixSignal {
            number,
            code: None,
            name: None,
            code_name: None,
        }
    }
}

impl From<(i32, i32)> for PosixSignal {
    fn from(tuple: (i32, i32)) -> PosixSignal {
        let (number, code) = tuple;
        PosixSignal {
            number,
            code: Some(code),
            name: None,
            code_name: None,
        }
    }
}

impl Into<i32> for PosixSignal {
    fn into(self) -> i32 {
        self.number
    }
}

/// Operating system or runtime meta information to an exception mechanism.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct MechanismMeta {
    /// Optional ISO C standard error code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errno: Option<CError>,
    /// Optional POSIX signal number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal: Option<PosixSignal>,
    /// Optional mach exception information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mach_exception: Option<MachException>,
}

impl MechanismMeta {
    fn is_empty(&self) -> bool {
        self.errno.is_none() && self.signal.is_none() && self.mach_exception.is_none()
    }
}

/// Represents a single exception.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Mechanism {
    /// The mechanism type identifier.
    #[serde(rename = "type")]
    pub ty: String,
    /// Human readable detail description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An optional link to online resources describing this error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<Url>,
    /// An optional flag indicating whether this exception was handled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handled: Option<bool>,
    /// An optional flag indicating a synthetic exception.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synthetic: Option<bool>,
    /// Additional attributes depending on the mechanism type.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub data: Map<String, Value>,
    /// Operating system or runtime meta information.
    #[serde(default, skip_serializing_if = "MechanismMeta::is_empty")]
    pub meta: MechanismMeta,
}

/// Represents a single exception.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Exception {
    /// The type of the exception.
    #[serde(rename = "type")]
    pub ty: String,
    /// The optional value of the exception.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// An optional module for this exception.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Optionally the stacktrace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stacktrace: Option<Stacktrace>,
    /// An optional raw stacktrace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_stacktrace: Option<Stacktrace>,
    /// Optional identifier referring to a thread.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<ThreadId>,
    /// The mechanism of the exception including OS specific exception values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<Mechanism>,
}

/// An error used when parsing `Level`.
#[derive(Debug, Error)]
#[error("invalid level")]
pub struct ParseLevelError;

/// Represents the level of severity of an event or breadcrumb.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    /// Indicates very spammy debug information.
    Debug,
    /// Informational messages.
    Info,
    /// A warning.
    Warning,
    /// An error.
    Error,
    /// Similar to error but indicates a critical event that usually causes a shutdown.
    Fatal,
}

impl Default for Level {
    fn default() -> Level {
        Level::Info
    }
}

impl str::FromStr for Level {
    type Err = ParseLevelError;

    fn from_str(string: &str) -> Result<Level, Self::Err> {
        Ok(match string {
            "debug" => Level::Debug,
            "info" | "log" => Level::Info,
            "warning" => Level::Warning,
            "error" => Level::Error,
            "fatal" => Level::Fatal,
            _ => return Err(ParseLevelError),
        })
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Level::Debug => write!(f, "debug"),
            Level::Info => write!(f, "info"),
            Level::Warning => write!(f, "warning"),
            Level::Error => write!(f, "error"),
            Level::Fatal => write!(f, "fatal"),
        }
    }
}

impl Level {
    /// A quick way to check if the level is `debug`.
    pub fn is_debug(&self) -> bool {
        *self == Level::Debug
    }

    /// A quick way to check if the level is `info`.
    pub fn is_info(&self) -> bool {
        *self == Level::Info
    }

    /// A quick way to check if the level is `warning`.
    pub fn is_warning(&self) -> bool {
        *self == Level::Warning
    }

    /// A quick way to check if the level is `error`.
    pub fn is_error(&self) -> bool {
        *self == Level::Error
    }

    /// A quick way to check if the level is `fatal`.
    pub fn is_fatal(&self) -> bool {
        *self == Level::Fatal
    }
}

impl_str_serde!(Level);

mod breadcrumb {
    use super::*;

    pub fn default_timestamp() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn default_type() -> String {
        "default".to_string()
    }

    pub fn is_default_type(ty: &str) -> bool {
        ty == "default"
    }

    pub fn default_level() -> Level {
        Level::Info
    }
}

/// Represents a single breadcrumb.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Breadcrumb {
    /// The timestamp of the breadcrumb.  This is required.
    #[serde(default = "breadcrumb::default_timestamp", with = "ts_seconds_float")]
    pub timestamp: DateTime<Utc>,
    /// The type of the breadcrumb.
    #[serde(
        rename = "type",
        default = "breadcrumb::default_type",
        skip_serializing_if = "breadcrumb::is_default_type"
    )]
    pub ty: String,
    /// The optional category of the breadcrumb.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The non optional level of the breadcrumb.  It
    /// defaults to info.
    #[serde(
        default = "breadcrumb::default_level",
        skip_serializing_if = "Level::is_info"
    )]
    pub level: Level,
    /// An optional human readbale message for the breadcrumb.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Arbitrary breadcrumb data that should be send along.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub data: Map<String, Value>,
}

impl Default for Breadcrumb {
    fn default() -> Breadcrumb {
        Breadcrumb {
            timestamp: breadcrumb::default_timestamp(),
            ty: breadcrumb::default_type(),
            category: Default::default(),
            level: breadcrumb::default_level(),
            message: Default::default(),
            data: Default::default(),
        }
    }
}

/// An IP address, either IPv4, IPv6 or Auto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum IpAddress {
    /// The IP address needs to be infered from the user's context.
    Auto,
    /// The exact given IP address (v4 or v6).
    Exact(IpAddr),
}

impl PartialEq<IpAddr> for IpAddress {
    fn eq(&self, other: &IpAddr) -> bool {
        match *self {
            IpAddress::Auto => false,
            IpAddress::Exact(ref addr) => addr == other,
        }
    }
}

impl cmp::PartialOrd<IpAddr> for IpAddress {
    fn partial_cmp(&self, other: &IpAddr) -> Option<cmp::Ordering> {
        match *self {
            IpAddress::Auto => None,
            IpAddress::Exact(ref addr) => addr.partial_cmp(other),
        }
    }
}

impl Default for IpAddress {
    fn default() -> IpAddress {
        IpAddress::Auto
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpAddress::Auto => write!(f, "{{{{auto}}}}"),
            IpAddress::Exact(ref addr) => write!(f, "{}", addr),
        }
    }
}

impl From<IpAddr> for IpAddress {
    fn from(addr: IpAddr) -> IpAddress {
        IpAddress::Exact(addr)
    }
}

impl str::FromStr for IpAddress {
    type Err = AddrParseError;

    fn from_str(string: &str) -> Result<IpAddress, AddrParseError> {
        match string {
            "{{auto}}" => Ok(IpAddress::Auto),
            other => other.parse().map(IpAddress::Exact),
        }
    }
}

impl_str_serde!(IpAddress);

/// Represents user info.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct User {
    /// The ID of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The email address of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The remote ip address of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<IpAddress>,
    /// A human readable username of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Additional arbitrary fields for forwards compatibility.
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

/// Represents http request data.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Request {
    /// The current URL of the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    /// The HTTP request method.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Optionally some associated request data (human readable)
    // XXX: this makes absolutely no sense because of unicode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Optionally the encoded query string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    /// An encoded cookie string if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    /// HTTP request headers.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub headers: Map<String, String>,
    /// Optionally a CGI/WSGI etc. environment dictionary.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub env: Map<String, String>,
}

/// Holds information about the system SDK.
///
/// This is relevant for iOS and other platforms that have a system
/// SDK.  Not to be confused with the client SDK.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SystemSdkInfo {
    /// The internal name of the SDK
    pub sdk_name: String,
    /// the major version of the SDK as integer or 0
    pub version_major: u32,
    /// the minor version of the SDK as integer or 0
    pub version_minor: u32,
    /// the patch version of the SDK as integer or 0
    pub version_patchlevel: u32,
}

/// Represents a debug image.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum DebugImage {
    /// Apple debug images (machos).  This is currently also used for
    /// non apple platforms with similar debug setups.
    Apple(AppleDebugImage),
    /// Symbolic (new style) debug infos.
    Symbolic(SymbolicDebugImage),
    /// A reference to a proguard debug file.
    Proguard(ProguardDebugImage),
}

impl DebugImage {
    /// Returns the name of the type on sentry.
    pub fn type_name(&self) -> &str {
        match *self {
            DebugImage::Apple(..) => "apple",
            DebugImage::Symbolic(..) => "symbolic",
            DebugImage::Proguard(..) => "proguard",
        }
    }
}

macro_rules! into_debug_image {
    ($kind:ident, $ty:ty) => {
        impl From<$ty> for DebugImage {
            fn from(data: $ty) -> DebugImage {
                DebugImage::$kind(data)
            }
        }
    };
}

/// Represents an apple debug image in the debug meta.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppleDebugImage {
    /// The name of the debug image (usually filename)
    pub name: String,
    /// The optional CPU architecture of the debug image.
    pub arch: Option<String>,
    /// Alternatively a macho cpu type.
    pub cpu_type: Option<u32>,
    /// Alternatively a macho cpu subtype.
    pub cpu_subtype: Option<u32>,
    /// The starting address of the image.
    pub image_addr: Addr,
    /// The size of the image in bytes.
    pub image_size: u64,
    /// The address where the image is loaded at runtime.
    #[serde(default, skip_serializing_if = "Addr::is_null")]
    pub image_vmaddr: Addr,
    /// The unique UUID of the image.
    pub uuid: Uuid,
}

/// Represents a symbolic debug image.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SymbolicDebugImage {
    /// The name of the debug image (usually filename)
    pub name: String,
    /// The optional CPU architecture of the debug image.
    pub arch: Option<String>,
    /// The starting address of the image.
    pub image_addr: Addr,
    /// The size of the image in bytes.
    pub image_size: u64,
    /// The address where the image is loaded at runtime.
    #[serde(default, skip_serializing_if = "Addr::is_null")]
    pub image_vmaddr: Addr,
    /// The unique debug id of the image.
    pub id: DebugId,
}

/// Represents a proguard mapping file reference.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProguardDebugImage {
    /// The UUID of the associated proguard file.
    pub uuid: Uuid,
}

into_debug_image!(Apple, AppleDebugImage);
into_debug_image!(Symbolic, SymbolicDebugImage);
into_debug_image!(Proguard, ProguardDebugImage);

/// Represents debug meta information.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct DebugMeta {
    /// Optional system SDK information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sdk_info: Option<SystemSdkInfo>,
    /// A list of debug information files.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<DebugImage>,
}

impl DebugMeta {
    /// Returns true if the debug meta is empty.
    ///
    /// This is used by the serializer to entirely skip the section.
    pub fn is_empty(&self) -> bool {
        self.sdk_info.is_none() && self.images.is_empty()
    }
}

/// Information on the SDK client.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClientSdkInfo {
    /// The name of the SDK.
    pub name: String,
    /// The version of the SDK.
    pub version: String,
    /// An optional list of integrations that are enabled in this SDK.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub integrations: Vec<String>,
    /// An optional list of packages that are installed in the SDK's environment.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<ClientSdkPackage>,
}

/// Represents an installed package relevant to the SDK.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClientSdkPackage {
    /// The name of the package installed.
    pub name: String,
    /// The version of the package.
    pub version: String,
}

/// Typed contextual data.
///
/// Types like `OsContext` can be directly converted with `.into()`
/// to `Context`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
#[non_exhaustive]
pub enum Context {
    /// Device data.
    Device(Box<DeviceContext>),
    /// Operating system data.
    Os(Box<OsContext>),
    /// Runtime data.
    Runtime(Box<RuntimeContext>),
    /// Application data.
    App(Box<AppContext>),
    /// Web browser data.
    Browser(Box<BrowserContext>),
    /// Tracing data.
    Trace(Box<TraceContext>),
    /// Generic other context data.
    #[serde(rename = "unknown")]
    Other(Map<String, Value>),
}

impl Context {
    /// Returns the name of the type for sentry.
    pub fn type_name(&self) -> &str {
        match *self {
            Context::Device(..) => "device",
            Context::Os(..) => "os",
            Context::Runtime(..) => "runtime",
            Context::App(..) => "app",
            Context::Browser(..) => "browser",
            Context::Trace(..) => "trace",
            Context::Other(..) => "unknown",
        }
    }
}

/// Optional device screen orientation
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    /// Portrait device orientation.
    Portrait,
    /// Landscape device orientation.
    Landscape,
}

/// Holds device information.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DeviceContext {
    /// The name of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The family of the device model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// The device model (human readable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The device model (internal identifier).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// The native cpu architecture of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// The current battery level (0-100).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battery_level: Option<f32>,
    /// The current screen orientation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
    /// Simulator/prod indicator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulator: Option<bool>,
    /// Total memory available in byts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<u64>,
    /// How much memory is still available in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_memory: Option<u64>,
    /// How much memory is usable for the app in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usable_memory: Option<u64>,
    /// Total storage size of the device in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<u64>,
    /// How much storage is free in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_storage: Option<u64>,
    /// Total size of the attached external storage in bytes (eg: android SDK card).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_storage_size: Option<u64>,
    /// Free size of the attached external storage in bytes (eg: android SDK card).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_free_storage: Option<u64>,
    /// Optionally an indicator when the device was booted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boot_time: Option<DateTime<Utc>>,
    /// The timezone of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Additional arbitrary fields for forwards compatibility.
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

/// Holds operating system information.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct OsContext {
    /// The name of the operating system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The version of the operating system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The internal build number of the operating system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    /// The current kernel version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// An indicator if the os is rooted (mobile mostly).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rooted: Option<bool>,
    /// Additional arbitrary fields for forwards compatibility.
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

/// Holds information about the runtime.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RuntimeContext {
    /// The name of the runtime (for instance JVM).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The version of the runtime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Additional arbitrary fields for forwards compatibility.
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

/// Holds app information.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AppContext {
    /// Optional start time of the app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_start_time: Option<DateTime<Utc>>,
    /// Optional device app hash (app specific device ID)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_app_hash: Option<String>,
    /// Optional build identicator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_type: Option<String>,
    /// Optional app identifier (dotted bundle id).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_identifier: Option<String>,
    /// Application name as it appears on the platform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// Application version as it appears on the platform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    /// Internal build ID as it appears on the platform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_build: Option<String>,
    /// Additional arbitrary fields for forwards compatibility.
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

/// Holds information about the web browser.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct BrowserContext {
    /// The name of the browser (for instance "Chrome").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The version of the browser.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Additional arbitrary fields for forwards compatibility.
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

/// Holds information about a tracing event.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TraceContext {
    /// The ID of the trace event
    #[serde(default = "event::default_id", serialize_with = "event::serialize_id")]
    pub span_id: Uuid,
    /// Determines which trace the transaction belongs to.
    #[serde(default = "event::default_id", serialize_with = "event::serialize_id")]
    pub trace_id: Uuid,
    /// Determines the parent of this transaction if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
    /// Short code identifying the type of operation the transaction is measuring.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// Human readable detail description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Describes the status of the span (e.g. `ok`, `cancelled`, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

macro_rules! into_context {
    ($kind:ident, $ty:ty) => {
        impl From<$ty> for Context {
            fn from(data: $ty) -> Self {
                Context::$kind(Box::new(data))
            }
        }
    };
}

into_context!(App, AppContext);
into_context!(Device, DeviceContext);
into_context!(Os, OsContext);
into_context!(Runtime, RuntimeContext);
into_context!(Browser, BrowserContext);
into_context!(Trace, TraceContext);

mod event {
    use super::*;

    pub fn default_id() -> Uuid {
        Uuid::new_v4()
    }

    pub fn serialize_id<S: Serializer>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_some(&uuid.to_simple_ref().to_string())
    }

    pub fn default_level() -> Level {
        Level::Error
    }

    pub fn default_platform() -> Cow<'static, str> {
        Cow::Borrowed("other")
    }

    pub fn is_default_platform(value: &str) -> bool {
        value == "other"
    }

    static DEFAULT_FINGERPRINT: &[Cow<'static, str>] = &[Cow::Borrowed("{{ default }}")];

    pub fn default_fingerprint<'a>() -> Cow<'a, [Cow<'a, str>]> {
        Cow::Borrowed(DEFAULT_FINGERPRINT)
    }

    #[allow(clippy::ptr_arg)]
    pub fn is_default_fingerprint<'a>(fp: &[Cow<'a, str>]) -> bool {
        fp.len() == 1 && ((&fp)[0] == "{{ default }}" || (&fp)[0] == "{{default}}")
    }

    pub fn default_timestamp() -> DateTime<Utc> {
        Utc::now()
    }
}

/// Represents a full event for Sentry.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event<'a> {
    /// The ID of the event
    #[serde(default = "event::default_id", serialize_with = "event::serialize_id")]
    pub event_id: Uuid,
    /// The level of the event (defaults to error)
    #[serde(
        default = "event::default_level",
        skip_serializing_if = "Level::is_error"
    )]
    pub level: Level,
    /// An optional fingerprint configuration to override the default.
    #[serde(
        default = "event::default_fingerprint",
        skip_serializing_if = "event::is_default_fingerprint"
    )]
    pub fingerprint: Cow<'a, [Cow<'a, str>]>,
    /// The culprit of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub culprit: Option<String>,
    /// The transaction name of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
    /// A message to be sent with the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optionally a log entry that can be used instead of the message for
    /// more complex cases.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logentry: Option<LogEntry>,
    /// Optionally the name of the logger that created this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
    /// Optionally a name to version mapping of installed modules.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub modules: Map<String, String>,
    /// A platform identifier for this event.
    #[serde(
        default = "event::default_platform",
        skip_serializing_if = "event::is_default_platform"
    )]
    pub platform: Cow<'a, str>,
    /// The timestamp of when the event was created.
    ///
    /// This can be set to `None` in which case the server will set a timestamp.
    #[serde(default = "event::default_timestamp", with = "ts_seconds_float")]
    pub timestamp: DateTime<Utc>,
    /// Optionally the server (or device) name of this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<Cow<'a, str>>,
    /// A release identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Cow<'a, str>>,
    /// An optional distribution identifer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dist: Option<Cow<'a, str>>,
    /// An optional environment identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<Cow<'a, str>>,
    /// Optionally user data to be sent along.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Optionally HTTP request data to be sent along.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    /// Optional contexts.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub contexts: Map<String, Context>,
    /// List of breadcrumbs to send along.
    #[serde(default, skip_serializing_if = "Values::is_empty")]
    pub breadcrumbs: Values<Breadcrumb>,
    /// Exceptions to be attached (one or multiple if chained).
    #[serde(default, skip_serializing_if = "Values::is_empty")]
    pub exception: Values<Exception>,
    /// A single stacktrace (deprecated)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stacktrace: Option<Stacktrace>,
    /// Simplified template error location info
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplateInfo>,
    /// A list of threads.
    #[serde(default, skip_serializing_if = "Values::is_empty")]
    pub threads: Values<Thread>,
    /// Optional tags to be attached to the event.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub tags: Map<String, String>,
    /// Optional extra information to be sent with the event.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    /// Debug meta information.
    #[serde(default, skip_serializing_if = "DebugMeta::is_empty")]
    pub debug_meta: Cow<'a, DebugMeta>,
    /// SDK metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sdk: Option<Cow<'a, ClientSdkInfo>>,
}

impl<'a> Default for Event<'a> {
    fn default() -> Self {
        Event {
            event_id: event::default_id(),
            level: event::default_level(),
            fingerprint: event::default_fingerprint(),
            culprit: Default::default(),
            transaction: Default::default(),
            message: Default::default(),
            logentry: Default::default(),
            logger: Default::default(),
            modules: Default::default(),
            platform: event::default_platform(),
            timestamp: event::default_timestamp(),
            server_name: Default::default(),
            release: Default::default(),
            dist: Default::default(),
            environment: Default::default(),
            user: Default::default(),
            request: Default::default(),
            contexts: Default::default(),
            breadcrumbs: Default::default(),
            exception: Default::default(),
            stacktrace: Default::default(),
            template: Default::default(),
            threads: Default::default(),
            tags: Default::default(),
            extra: Default::default(),
            debug_meta: Default::default(),
            sdk: Default::default(),
        }
    }
}

impl<'a> Event<'a> {
    /// Creates a new event with the current timestamp and random id.
    pub fn new() -> Event<'a> {
        Default::default()
    }

    /// Creates a fully owned version of the event.
    pub fn into_owned(self) -> Event<'static> {
        Event {
            event_id: self.event_id,
            level: self.level,
            fingerprint: Cow::Owned(
                self.fingerprint
                    .iter()
                    .map(|x| Cow::Owned(x.to_string()))
                    .collect(),
            ),
            culprit: self.culprit,
            transaction: self.transaction,
            message: self.message,
            logentry: self.logentry,
            logger: self.logger,
            modules: self.modules,
            platform: Cow::Owned(self.platform.into_owned()),
            timestamp: self.timestamp,
            server_name: self.server_name.map(|x| Cow::Owned(x.into_owned())),
            release: self.release.map(|x| Cow::Owned(x.into_owned())),
            dist: self.dist.map(|x| Cow::Owned(x.into_owned())),
            environment: self.environment.map(|x| Cow::Owned(x.into_owned())),
            user: self.user,
            request: self.request,
            contexts: self.contexts,
            breadcrumbs: self.breadcrumbs,
            exception: self.exception,
            stacktrace: self.stacktrace,
            template: self.template,
            threads: self.threads,
            tags: self.tags,
            extra: self.extra,
            debug_meta: Cow::Owned(self.debug_meta.into_owned()),
            sdk: self.sdk.map(|x| Cow::Owned(x.into_owned())),
        }
    }
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event(id: {}, ts: {})", self.event_id, self.timestamp)
    }
}

/// Represents a tracing span.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Span {
    /// The ID of the span
    #[serde(default = "event::default_id", serialize_with = "event::serialize_id")]
    pub span_id: Uuid,
    /// Determines which trace the span belongs to.
    #[serde(default = "event::default_id", serialize_with = "event::serialize_id")]
    pub trace_id: Uuid,
    /// Determines the parent of this span, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
    /// Determines whether this span is generated in the same process as its parent, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub same_process_as_parent: Option<bool>,
    /// Short code identifying the type of operation the span is measuring.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// Longer description of the span's operation, which uniquely identifies the span
    /// but is consistent across instances of the span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The timestamp at the measuring of the span finished.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    /// The timestamp at the measuring of the span started.
    #[serde(default = "event::default_timestamp", with = "ts_seconds_float")]
    pub start_timestamp: DateTime<Utc>,
    /// Describes the status of the span (e.g. `ok`, `cancelled`, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Optional tags to be attached to the span.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub tags: Map<String, String>,
    /// Optional extra information to be sent with the span.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub data: Map<String, Value>,
}

impl Default for Span {
    fn default() -> Self {
        Span {
            span_id: event::default_id(),
            trace_id: event::default_id(),
            timestamp: Default::default(),
            tags: Default::default(),
            start_timestamp: event::default_timestamp(),
            description: Default::default(),
            status: Default::default(),
            parent_span_id: Default::default(),
            same_process_as_parent: Default::default(),
            op: Default::default(),
            data: Default::default(),
        }
    }
}

impl Span {
    /// Creates a new span with the current timestamp and random id.
    pub fn new() -> Span {
        Default::default()
    }

    /// Finalizes the span.
    pub fn finish(&mut self) {
        self.timestamp = Some(Utc::now());
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Span(id: {}, ts: {})",
            self.span_id, self.start_timestamp
        )
    }
}

/// Represents a tracing transaction.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction<'a> {
    /// The ID of the event
    #[serde(default = "event::default_id", serialize_with = "event::serialize_id")]
    pub event_id: Uuid,
    /// The transaction name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional tags to be attached to the event.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub tags: Map<String, String>,
    /// SDK metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sdk: Option<Cow<'a, ClientSdkInfo>>,
    /// A platform identifier for this event.
    #[serde(
        default = "event::default_platform",
        skip_serializing_if = "event::is_default_platform"
    )]
    pub platform: Cow<'a, str>,
    /// The end time of the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    /// The start time of the transaction.
    #[serde(default = "event::default_timestamp", with = "ts_seconds_float")]
    pub start_timestamp: DateTime<Utc>,
    /// The collection of finished spans part of this transaction.
    pub spans: Vec<Span>,
    /// Optional contexts.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub contexts: Map<String, Context>,
}

impl<'a> Default for Transaction<'a> {
    fn default() -> Self {
        Transaction {
            event_id: event::default_id(),
            name: Default::default(),
            tags: Default::default(),
            sdk: Default::default(),
            platform: event::default_platform(),
            timestamp: Default::default(),
            start_timestamp: event::default_timestamp(),
            spans: Default::default(),
            contexts: Default::default(),
        }
    }
}

impl<'a> Transaction<'a> {
    /// Creates a new span transaction the current timestamp and random id.
    pub fn new() -> Transaction<'a> {
        Default::default()
    }

    /// Creates a fully owned version of the transaction.
    pub fn into_owned(self) -> Transaction<'static> {
        Transaction {
            event_id: self.event_id,
            name: self.name,
            tags: self.tags,
            sdk: self.sdk.map(|x| Cow::Owned(x.into_owned())),
            platform: Cow::Owned(self.platform.into_owned()),
            timestamp: self.timestamp,
            start_timestamp: self.start_timestamp,
            spans: self.spans,
            contexts: self.contexts,
        }
    }

    /// Finalizes the transaction to be dispatched.
    pub fn finish(&mut self) {
        self.timestamp = Some(Utc::now());
    }
}

impl<'a> fmt::Display for Transaction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Transaction(id: {}, ts: {})",
            self.event_id, self.start_timestamp
        )
    }
}
