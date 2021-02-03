//! Deserialization support for querystrings.

//! ### An overview of the design of `QsDeserializer`
//!
//! This code is designed to handle non-ordered query parameters. For example,
//! `struct { a: Vec<u8>, b: String }` might be serialized as either
//! `a[0]=1&a[1]=2&b=Hello or a[1]=2&b=Hello&a[0]=1`.
//!
//! In order to cover the latter case, we have two options: scan through the
//! string each time we need to find a particular key - worst case O(n^2 )
//! running time; or pre-parse the list into a map structure, and then
//! deserialize the map.
//!
//! We opt for the latter. But a TODO is implement the first case, which could
//! potentially be more desirable, especially when the keys are known to be in
//! order.
//!
//! The `parse` module handles this step of deserializing a querystring into the
//! map structure. This uses `rust_url::percent_encoding` to handle
//! first converting the string.
//!
//! From here, there are two main `Deserializer` objects: `QsDeserializer` and
//! `LevelDeserializer`.
//!
//! The former is the top-level deserializer which is effectively only capable
//! of deserializing map-like objects (i.e. those with (key, value) pairs).
//! Hence, structs, maps, and enums are supported at this level.
//!
//! Each key is a `String`, and deserialized from a `String`. The values are
//! `Level` elements. This is a recursive structure which can either be a "flat
//! value", i.e. just a string, or a sequence or map of these elements. This can
//! be thought of as similar to the `serde_json::Value` enum.
//!
//! Each `Level` can be deserialized through `LevelDeserializer`. This will
//! recursively call back to the top level `QsDeserializer` for maps, or when
//! `Level` is a flat value it will attempt to deserialize it to a primitive via
//! `ParsableStringDeserializer`.

mod parse;

use crate::error::*;

use serde::de;
use serde::de::IntoDeserializer;

use std::borrow::Cow;
use std::collections::btree_map::{BTreeMap, Entry, IntoIter};

/// To override the default serialization parameters, first construct a new
/// Config.
///
/// The `strict` parameter controls whether the deserializer will tolerate
/// encoded brackets as part of the key. For example, serializing the field
/// `a = vec![12]` might give `a[0]=12`. In strict mode, the only string accepted
/// will be this string, whereas in non-strict mode, this can also be deserialized
/// from `a%5B0%5D=12`. Strict mode is more accurate for cases where it a field
/// may contain square brackets.
/// In non-strict mode, the deserializer will generally tolerate unexpected
/// characters.
///
/// A `max_depth` of 0 implies no nesting: the result will be a flat map.
/// This is mostly useful when the maximum nested depth is known beforehand,
/// to prevent denial of service attacks by providing incredibly deeply nested
/// inputs.
///
/// The default value for `max_depth` is 5, and the default mode is `strict=true`.
///
/// ```
/// use serde_qs::Config;
/// use std::collections::HashMap;
///
/// let config = Config::new(0, true);
/// let map: HashMap<String, String> = config.deserialize_str("a[b][c]=1")
///                                          .unwrap();
/// assert_eq!(map.get("a[b][c]").unwrap(), "1");
///
/// let config = Config::new(10, true);
/// let map: HashMap<String, HashMap<String, HashMap<String, String>>> =
///             config.deserialize_str("a[b][c]=1").unwrap();
/// assert_eq!(map.get("a").unwrap().get("b").unwrap().get("c").unwrap(), "1");
/// ```
///
pub struct Config {
    /// Specifies the maximum depth key that `serde_qs` will attempt to
    /// deserialize. Default is 5.
    max_depth: usize,
    /// Strict deserializing mode will not tolerate encoded brackets.
    strict: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new(5, true)
    }
}

impl Config {
    /// Create a new `Config` with the specified `max_depth` and `strict` mode.
    pub fn new(max_depth: usize, strict: bool) -> Self {
        Self { max_depth, strict }
    }

    /// Get maximum depth parameter.
    fn max_depth(&self) -> usize {
        self.max_depth
    }
}

impl Config {
    /// Deserializes a querystring from a `&[u8]` using this `Config`.
    pub fn deserialize_bytes<'de, T: de::Deserialize<'de>>(&self, input: &'de [u8]) -> Result<T> {
        T::deserialize(QsDeserializer::with_config(self, input)?)
    }

    // pub fn deserialize_bytes_sloppy<T: de::DeserializeOwned>(&self, input: &[u8])
    //     -> Result<T>
    // {
    //     let buf = String::from_utf8(input.to_vec())?;
    //     let buf = buf.replace("%5B", "[").replace("%5D", "]").into_bytes();
    //     let deser = QsDeserializer::with_config(self, &buf)?;
    //     T::deserialize(deser)
    // }

    /// Deserializes a querystring from a `&str` using this `Config`.
    pub fn deserialize_str<'de, T: de::Deserialize<'de>>(&self, input: &'de str) -> Result<T> {
        self.deserialize_bytes(input.as_bytes())
    }
}

/// Deserializes a querystring from a `&[u8]`.
///
/// ```
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_qs;
/// #[derive(Debug, Deserialize, PartialEq, Serialize)]
/// struct Query {
///     name: String,
///     age: u8,
///     occupation: String,
/// }
///
/// # fn main(){
/// let q =  Query {
///     name: "Alice".to_owned(),
///     age: 24,
///     occupation: "Student".to_owned(),
/// };
///
/// assert_eq!(
///     serde_qs::from_bytes::<Query>(
///         "name=Alice&age=24&occupation=Student".as_bytes()
///     ).unwrap(), q);
/// # }
/// ```
pub fn from_bytes<'de, T: de::Deserialize<'de>>(input: &'de [u8]) -> Result<T> {
    Config::default().deserialize_bytes(input)
}

/// Deserializes a querystring from a `&str`.
///
/// ```
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_qs;
/// #[derive(Debug, Deserialize, PartialEq, Serialize)]
/// struct Query {
///     name: String,
///     age: u8,
///     occupation: String,
/// }
///
/// # fn main(){
/// let q =  Query {
///     name: "Alice".to_owned(),
///     age: 24,
///     occupation: "Student".to_owned(),
/// };
///
/// assert_eq!(
///     serde_qs::from_str::<Query>("name=Alice&age=24&occupation=Student").unwrap(),
///     q);
/// # }
/// ```
pub fn from_str<'de, T: de::Deserialize<'de>>(input: &'de str) -> Result<T> {
    from_bytes(input.as_bytes())
}

/// A deserializer for the querystring format.
///
/// Supported top-level outputs are structs and maps.
pub(crate) struct QsDeserializer<'a> {
    iter: IntoIter<Cow<'a, str>, Level<'a>>,
    value: Option<Level<'a>>,
}

#[derive(Debug)]
enum Level<'a> {
    Nested(BTreeMap<Cow<'a, str>, Level<'a>>),
    OrderedSeq(BTreeMap<usize, Level<'a>>),
    Sequence(Vec<Level<'a>>),
    Flat(Cow<'a, str>),
    Invalid(&'static str),
    Uninitialised,
}

impl<'a> QsDeserializer<'a> {
    fn with_map(map: BTreeMap<Cow<'a, str>, Level<'a>>) -> Self {
        QsDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }

    /// Returns a new `QsDeserializer<'a>`.
    fn with_config(config: &Config, input: &'a [u8]) -> Result<Self> {
        parse::Parser::new(input, config.max_depth(), config.strict).as_deserializer()
    }
}

impl<'de> de::Deserializer<'de> for QsDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::top_level("primitive"))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    /// Throws an error.
    ///
    /// Sequences are not supported at the top level.
    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::top_level("sequence"))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    /// Throws an error.
    ///
    /// Tuples are not supported at the top level.
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::top_level("tuple"))
    }

    /// Throws an error.
    ///
    /// TupleStructs are not supported at the top level.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::top_level("tuple struct"))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    forward_to_deserialize_any! {
        bool
        u8
        u16
        u32
        u64
        i8
        i16
        i32
        i64
        f32
        f64
        char
        str
        string
        unit
        option
        bytes
        byte_buf
        unit_struct
        identifier
        ignored_any
    }
}

impl<'de> de::MapAccess<'de> for QsDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            return seed.deserialize(ParsableStringDeserializer(key)).map(Some);
        };
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        if let Some(v) = self.value.take() {
            seed.deserialize(LevelDeserializer(v))
        } else {
            Err(de::Error::custom(
                "Somehow the list was empty after a \
                 non-empty key was returned",
            ))
        }
    }
}

impl<'de> de::EnumAccess<'de> for QsDeserializer<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            Ok((seed.deserialize(ParsableStringDeserializer(key))?, self))
        } else {
            Err(de::Error::custom("No more values"))
        }
    }
}

impl<'de> de::VariantAccess<'de> for QsDeserializer<'de> {
    type Error = Error;
    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.value {
            seed.deserialize(LevelDeserializer(value))
        } else {
            Err(de::Error::custom("no value to deserialize"))
        }
    }
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.value {
            de::Deserializer::deserialize_seq(LevelDeserializer(value), visitor)
        } else {
            Err(de::Error::custom("no value to deserialize"))
        }
    }
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.value {
            de::Deserializer::deserialize_map(LevelDeserializer(value), visitor)
        } else {
            Err(de::Error::custom("no value to deserialize"))
        }
    }
}

impl<'de> de::EnumAccess<'de> for LevelDeserializer<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.0 {
            Level::Flat(x) => Ok((
                seed.deserialize(ParsableStringDeserializer(x))?,
                LevelDeserializer(Level::Invalid(
                    "this value can only \
                     deserialize to a \
                     UnitVariant",
                )),
            )),
            _ => Err(de::Error::custom(
                "this value can only deserialize to a \
                 UnitVariant",
            )),
        }
    }
}

impl<'de> de::VariantAccess<'de> for LevelDeserializer<'de> {
    type Error = Error;
    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self, visitor)
    }
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self, visitor)
    }
}

struct LevelSeq<'a, I: Iterator<Item = Level<'a>>>(I);

impl<'de, I: Iterator<Item = Level<'de>>> de::SeqAccess<'de> for LevelSeq<'de, I> {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(v) = self.0.next() {
            seed.deserialize(LevelDeserializer(v)).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct LevelDeserializer<'a>(Level<'a>);

macro_rules! deserialize_primitive {
    ($ty:ident, $method:ident, $visit_method:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            match self.0 {
                Level::Nested(_) => Err(de::Error::custom(format!(
                    "Expected: {:?}, got a Map",
                    stringify!($ty)
                ))),
                Level::OrderedSeq(_) => Err(de::Error::custom(format!(
                    "Expected: {:?}, got an OrderedSequence",
                    stringify!($ty)
                ))),
                Level::Sequence(_) => Err(de::Error::custom(format!(
                    "Expected: {:?}, got a Sequence",
                    stringify!($ty)
                ))),
                Level::Flat(x) => ParsableStringDeserializer(x).$method(visitor),
                Level::Invalid(e) => Err(de::Error::custom(e)),
                Level::Uninitialised => Err(de::Error::custom(
                    "attempted to deserialize unitialised value",
                )),
            }
        }
    };
}

impl<'a> LevelDeserializer<'a> {
    fn into_deserializer(self) -> Result<QsDeserializer<'a>> {
        match self.0 {
            Level::Nested(map) => Ok(QsDeserializer::with_map(map)),
            Level::Invalid(e) => Err(de::Error::custom(e)),
            l => Err(de::Error::custom(format!(
                "could not convert {:?} to \
                 QsDeserializer<'a>",
                l
            ))),
        }
    }
}

impl<'de> de::Deserializer<'de> for LevelDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.0 {
            Level::Nested(_) => self.into_deserializer()?.deserialize_map(visitor),
            Level::OrderedSeq(map) => visitor.visit_seq(LevelSeq(map.into_iter().map(|(_k, v)| v))),
            Level::Sequence(seq) => visitor.visit_seq(LevelSeq(seq.into_iter())),
            Level::Flat(x) => match x {
                Cow::Owned(s) => visitor.visit_string(s),
                Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
            },
            Level::Invalid(e) => Err(de::Error::custom(e)),
            Level::Uninitialised => Err(de::Error::custom(
                "attempted to deserialize unitialised \
                 value",
            )),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.0 {
            Level::Flat(ref x) if x == "" => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.0 {
            Level::Nested(map) => {
                QsDeserializer::with_map(map).deserialize_enum(name, variants, visitor)
            }
            Level::Flat(_) => visitor.visit_enum(self),
            x => Err(de::Error::custom(format!(
                "{:?} does not appear to be \
                 an enum",
                x
            ))),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.0 {
            Level::Nested(_) => self.into_deserializer()?.deserialize_map(visitor),
            Level::OrderedSeq(map) => visitor.visit_seq(LevelSeq(map.into_iter().map(|(_k, v)| v))),
            Level::Sequence(seq) => visitor.visit_seq(LevelSeq(seq.into_iter())),
            Level::Flat(_) => {
                // For a newtype_struct, attempt to deserialize a flat value as a
                // single element sequence.
                visitor.visit_seq(LevelSeq(vec![self.0].into_iter()))
            }
            Level::Invalid(e) => Err(de::Error::custom(e)),
            Level::Uninitialised => Err(de::Error::custom(
                "attempted to deserialize unitialised \
                 value",
            )),
        }
    }

    deserialize_primitive!(bool, deserialize_bool, visit_bool);
    deserialize_primitive!(i8, deserialize_i8, visit_i8);
    deserialize_primitive!(i16, deserialize_i16, visit_i16);
    deserialize_primitive!(i32, deserialize_i32, visit_i32);
    deserialize_primitive!(i64, deserialize_i64, visit_i64);
    deserialize_primitive!(u8, deserialize_u8, visit_u8);
    deserialize_primitive!(u16, deserialize_u16, visit_u16);
    deserialize_primitive!(u32, deserialize_u32, visit_u32);
    deserialize_primitive!(u64, deserialize_u64, visit_u64);
    deserialize_primitive!(f32, deserialize_f32, visit_f32);
    deserialize_primitive!(f64, deserialize_f64, visit_f64);

    forward_to_deserialize_any! {
        char
        str
        string
        bytes
        byte_buf
        unit
        unit_struct
        // newtype_struct
        tuple_struct
        struct
        identifier
        tuple
        ignored_any
        seq
        map
    }
}

macro_rules! forward_parsable_to_deserialize_any {
    ($($ty:ident => $meth:ident,)*) => {
        $(
            fn $meth<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
                match self.0.parse::<$ty>() {
                    Ok(val) => val.into_deserializer().$meth(visitor),
                    Err(e) => Err(de::Error::custom(e))
                }
            }
        )*
    }
}

struct ParsableStringDeserializer<'a>(Cow<'a, str>);

impl<'de> de::Deserializer<'de> for ParsableStringDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.0.into_deserializer().deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(LevelDeserializer(Level::Flat(self.0)))
    }

    forward_to_deserialize_any! {
        map
        struct
        seq
        option
        char
        str
        string
        unit
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        identifier
        tuple
        ignored_any
    }

    forward_parsable_to_deserialize_any! {
        bool => deserialize_bool,
        u8 => deserialize_u8,
        u16 => deserialize_u16,
        u32 => deserialize_u32,
        u64 => deserialize_u64,
        i8 => deserialize_i8,
        i16 => deserialize_i16,
        i32 => deserialize_i32,
        i64 => deserialize_i64,
        f32 => deserialize_f32,
        f64 => deserialize_f64,
    }
}
