use serde::{
    Serialize,
    ser::{
        Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
        SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    },
};
use std::io::Write;

use crate::{Error, Result};

pub struct Serializer<'a, W: Write> {
    output: &'a mut W,
    start: bool,
}

impl<'a, W: Write> Serializer<'a, W> {
    pub fn new(output: &'a mut W) -> Self {
        Self {
            output,
            start: false,
        }
    }
}

impl<W: Write> serde::Serializer for &mut Serializer<'_, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.write_all(if v { b"true" } else { b"false" })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i16(self, v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i32(self, v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i64(self, v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output
            .write_all(itoa::Buffer::new().format(v).as_bytes())?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u16(self, v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u32(self, v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u64(self, v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output
            .write_all(itoa::Buffer::new().format(v).as_bytes())?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_f64(self, v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.write_all(v.to_string().as_bytes())?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.write_all(b"\"")?;
        let escaped = v.chars().map(escape).collect::<String>();
        self.output.write_all(escaped.as_bytes())?;
        self.output.write_all(b"\"")?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            SerializeSeq::serialize_element(&mut seq, byte)?;
        }
        SerializeSeq::end(seq)
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.write_all(b"null")?;
        Ok(())
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.output.write_all(b"{")?;
        variant.serialize(&mut *self)?;
        self.output.write_all(b":")?;
        value.serialize(&mut *self)?;
        self.output.write_all(b"}")?;
        Ok(())
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        self.start = true;
        self.output.write_all(b"[")?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        self.output.write_all(b"{")?;
        variant.serialize(&mut *self)?;
        self.output.write_all(b":")?;
        self.serialize_seq(Some(len))
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        self.start = true;
        self.output.write_all(b"{")?;
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        self.output.write_all(b"{")?;
        variant.serialize(&mut *self)?;
        self.output.write_all(b":")?;
        self.serialize_map(Some(len))
    }
}

impl<W: Write> SerializeSeq for &mut Serializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.start {
            self.output.write_all(b",")?;
        } else {
            self.start = false;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.write_all(b"]")?;
        Ok(())
    }
}

impl<W: Write> SerializeTuple for &mut Serializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

impl<W: Write> SerializeTupleStruct for &mut Serializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

impl<W: Write> SerializeTupleVariant for &mut Serializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.write_all(b"]}")?;
        Ok(())
    }
}

impl<W: Write> SerializeMap for &mut Serializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.start {
            self.output.write_all(b",")?;
        } else {
            self.start = false;
        }
        key.serialize(&mut KeySerializer {
            output: &mut *self.output,
        })?;
        self.output.write_all(b":")?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.write_all(b"}")?;
        Ok(())
    }
}

impl<W: Write> SerializeStruct for &mut Serializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_entry(key, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        SerializeMap::end(self)
    }
}

impl<W: Write> SerializeStructVariant for &mut Serializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_entry(key, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.write_all(b"}}")?;
        Ok(())
    }
}

struct KeySerializer<'a, W: Write> {
    output: &'a mut W,
}

impl<W: Write> serde::Serializer for &mut KeySerializer<'_, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }
    fn serialize_i8(self, _v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }
    fn serialize_i16(self, _v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }
    fn serialize_i32(self, _v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }
    fn serialize_i64(self, _v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_u8(self, _v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }
    fn serialize_u16(self, _v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }
    fn serialize_u32(self, _v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }
    fn serialize_u64(self, _v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_f32(self, _v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }
    fn serialize_f64(self, _v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        Serializer {
            start: false,
            output: &mut *self.output,
        }
        .serialize_str(v)
    }

    fn serialize_bytes(self, _v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_some<T>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::KeyNotString)
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::KeyNotString)
    }
}

pub fn to_string(value: &impl Serialize) -> Result<String> {
    let mut out = Vec::new();
    let mut serializer = Serializer::new(&mut out);
    value.serialize(&mut serializer)?;
    // SAFETY: The serializer implementation only ever writes valid UTF-8.
    Ok(unsafe { String::from_utf8_unchecked(out) })
}

pub fn to_bytes(value: &impl Serialize) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut serializer = Serializer::new(&mut out);
    value.serialize(&mut serializer)?;
    Ok(out)
}

pub fn to_writer(value: &impl Serialize, writer: &mut impl Write) -> Result<()> {
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(())
}

pub fn escape(c: char) -> String {
    match c {
        '"' => "\\\"".to_string(),
        '\\' => "\\\\".to_string(),
        '\x08' => "\\b".to_string(),
        '\x0c' => "\\f".to_string(),
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        '\x00'..='\x1F' => format!("\\u{:04x}", c as u8),
        _ => c.to_string(),
    }
}
