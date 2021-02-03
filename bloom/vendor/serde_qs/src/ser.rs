//! Serialization support for querystrings.

use percent_encoding::{percent_encode, AsciiSet, NON_ALPHANUMERIC};
use serde::ser;

use crate::error::*;

use std::borrow::Cow;
use std::fmt::Display;
use std::io::Write;
use std::str;

const QS_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b' ')
    .remove(b'*')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_');

/// Serializes a value into a querystring.
///
/// ```
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_qs;
/// #[derive(Deserialize, Serialize)]
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
///
/// assert_eq!(
///     serde_qs::to_string(&q).unwrap(),
///     "name=Alice&age=24&occupation=Student");
/// # }
/// ```
pub fn to_string<T: ser::Serialize>(input: &T) -> Result<String> {
    let mut buffer = Vec::new();
    let mut first = true;
    input.serialize(&mut QsSerializer {
        writer: &mut buffer,
        key: None,
        first: &mut first,
    })?;
    String::from_utf8(buffer).map_err(Error::from)
}

/// Serializes a value into a generic writer object.
///
/// ```
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_qs;
/// #[derive(Deserialize, Serialize)]
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
/// let mut buffer = Vec::new();
/// serde_qs::to_writer(&q, &mut buffer).unwrap();
/// assert_eq!(
///     String::from_utf8(buffer).unwrap(),
///     "name=Alice&age=24&occupation=Student");
/// # }
/// ```
pub fn to_writer<T: ser::Serialize, W: Write>(input: &T, writer: &mut W) -> Result<()> {
    let mut first = true;
    input.serialize(&mut QsSerializer {
        writer,
        key: None,
        first: &mut first,
    })
}

/// A serializer for the querystring format.
///
/// * Supported top-level inputs are structs and maps.
///
/// * Supported values are currently most primitive types, structs, maps and
///   sequences. Sequences are serialized with an incrementing key index.
///
/// * Newtype structs defer to their inner values.
pub struct QsSerializer<'a, W: 'a + Write> {
    key: Option<Cow<'static, str>>,
    writer: &'a mut W,
    first: &'a mut bool,
}

fn replace_space(input: &str) -> Cow<str> {
    match input.as_bytes().iter().position(|&b| b == b' ') {
        None => Cow::Borrowed(input),
        Some(first_position) => {
            let mut replaced = input.as_bytes().to_owned();
            replaced[first_position] = b'+';
            for byte in &mut replaced[first_position + 1..] {
                if *byte == b' ' {
                    *byte = b'+';
                }
            }
            Cow::Owned(String::from_utf8(replaced).expect("replacing ' ' with '+' cannot panic"))
        }
    }
}

impl<'a, W: 'a + Write> QsSerializer<'a, W> {
    fn extend_key(&mut self, newkey: &str) {
        let newkey = percent_encode(newkey.as_bytes(), QS_ENCODE_SET).collect::<Cow<str>>();
        let key = if let Some(ref key) = self.key {
            format!("{}[{}]", key, newkey).into()
        } else {
            newkey.to_owned()
        };
        self.key = Some(key)
    }

    fn write_value(&mut self, value: &[u8]) -> Result<()> {
        if let Some(ref key) = self.key {
            write!(
                self.writer,
                "{}{}={}",
                if *self.first {
                    *self.first = false;
                    ""
                } else {
                    "&"
                },
                key,
                percent_encode(value, QS_ENCODE_SET)
                    .map(replace_space)
                    .collect::<String>()
            )
            .map_err(Error::from)
        } else {
            Err(Error::no_key())
        }
    }

    /// Creates a new `QsSerializer` with a distinct key, but `writer` and
    ///`first` referring to the original data.
    fn new_from_ref<'b: 'a>(other: &'a mut QsSerializer<'b, W>) -> QsSerializer<'a, W> {
        Self {
            key: other.key.clone(),
            writer: other.writer,
            first: other.first,
        }
    }
}

impl Error {
    fn no_key() -> Self {
        let msg = "tried to serialize a value before serializing key";
        Error::Custom(msg.into())
    }
}

macro_rules! serialize_as_string {
    (Qs $($ty:ty => $meth:ident,)*) => {
        $(
            fn $meth(self, v: $ty) -> Result<Self::Ok> {
                self.write_value(&v.to_string().as_bytes())
            }
        )*
    };
    ($($ty:ty => $meth:ident,)*) => {
        $(
            fn $meth(self, v: $ty) -> Result<Self::Ok> {
                Ok(v.to_string())
            }
        )*
    };
}

impl<'a, W: Write> ser::Serializer for &'a mut QsSerializer<'a, W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = QsSeq<'a, W>;
    type SerializeTuple = QsSeq<'a, W>;
    type SerializeTupleStruct = QsSeq<'a, W>;
    type SerializeTupleVariant = QsSeq<'a, W>;
    type SerializeMap = QsMap<'a, W>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    serialize_as_string! {
        Qs
        bool => serialize_bool,
        u8  => serialize_u8,
        u16 => serialize_u16,
        u32 => serialize_u32,
        u64 => serialize_u64,
        i8  => serialize_i8,
        i16 => serialize_i16,
        i32 => serialize_i32,
        i64 => serialize_i64,
        f32 => serialize_f32,
        f64 => serialize_f64,
        char => serialize_char,
        &str => serialize_str,
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        self.write_value(&value)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.write_value(&[])
    }

    /// Returns an error.
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        self.write_value(name.as_bytes())
    }

    /// Returns an error.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.write_value(variant.as_bytes())
    }

    /// Returns an error.
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        value.serialize(self)
    }

    /// Returns an error.
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        self.extend_key(variant);
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<Self::Ok> {
        // Err(Error::Unsupported)
        value.serialize(self)
    }

    /// Returns an error.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(QsSeq(self, 0))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(QsSeq(self, 0))
    }

    /// Returns an error.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(QsSeq(self, 0))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        // self.write(variant)?;
        self.extend_key(variant);
        Ok(QsSeq(self, 0))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(QsMap(self, None))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.extend_key(variant);
        Ok(self)
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

pub struct QsSeq<'a, W: 'a + Write>(&'a mut QsSerializer<'a, W>, usize);
pub struct QsMap<'a, W: 'a + Write>(&'a mut QsSerializer<'a, W>, Option<Cow<'a, str>>);

impl<'a, W: Write> ser::SerializeTuple for QsSeq<'a, W> {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let mut serializer = QsSerializer::new_from_ref(self.0);
        serializer.extend_key(&self.1.to_string());
        self.1 += 1;
        value.serialize(&mut serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeSeq for QsSeq<'a, W> {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let mut serializer = QsSerializer::new_from_ref(self.0);
        serializer.extend_key(&self.1.to_string());
        self.1 += 1;
        value.serialize(&mut serializer)
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut QsSerializer<'a, W> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let mut serializer = QsSerializer::new_from_ref(self);
        serializer.extend_key(key);
        value.serialize(&mut serializer)
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut QsSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let mut serializer = QsSerializer::new_from_ref(self);
        serializer.extend_key(key);
        value.serialize(&mut serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for QsSeq<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let mut serializer = QsSerializer::new_from_ref(self.0);
        serializer.extend_key(&self.1.to_string());
        self.1 += 1;
        value.serialize(&mut serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for QsSeq<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let mut serializer = QsSerializer::new_from_ref(self.0);
        serializer.extend_key(&self.1.to_string());
        self.1 += 1;
        value.serialize(&mut serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeMap for QsMap<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.1 = Some(Cow::from(key.serialize(StringSerializer)?));
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let mut serializer = QsSerializer::new_from_ref(self.0);
        if let Some(ref key) = self.1 {
            serializer.extend_key(key);
        } else {
            return Err(Error::no_key());
        }
        self.1 = None;
        value.serialize(&mut serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: ser::Serialize,
        V: ser::Serialize,
    {
        let mut serializer = QsSerializer::new_from_ref(self.0);
        serializer.extend_key(&key.serialize(StringSerializer)?);
        value.serialize(&mut serializer)
    }
}

struct StringSerializer;

impl ser::Serializer for StringSerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = ser::Impossible<String, Error>;
    type SerializeTuple = ser::Impossible<String, Error>;
    type SerializeTupleStruct = ser::Impossible<String, Error>;
    type SerializeTupleVariant = ser::Impossible<String, Error>;
    type SerializeMap = ser::Impossible<String, Error>;
    type SerializeStruct = ser::Impossible<String, Error>;
    type SerializeStructVariant = ser::Impossible<String, Error>;

    serialize_as_string! {
        bool => serialize_bool,
        u8  => serialize_u8,
        u16 => serialize_u16,
        u32 => serialize_u32,
        u64 => serialize_u64,
        i8  => serialize_i8,
        i16 => serialize_i16,
        i32 => serialize_i32,
        i64 => serialize_i64,
        f32 => serialize_f32,
        f64 => serialize_f64,
        char => serialize_char,
        &str => serialize_str,
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        Ok(String::from_utf8_lossy(value).to_string())
    }

    /// Returns an error.
    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    /// Returns an error.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(variant.to_string())
    }

    /// Returns an error.
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    /// Returns an error.
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    /// Returns an error.
    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    /// Returns an error.
    fn serialize_some<T: ?Sized + ser::Serialize>(self, _value: &T) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    /// Returns an error.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::Unsupported)
    }

    /// Returns an error.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Unsupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::Unsupported)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::Unsupported)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Unsupported)
    }
}
