use std::{
    io::{BufRead, BufReader, Read},
    num::ParseIntError,
};

use serde::{
    de::{DeserializeOwned, MapAccess, SeqAccess},
    forward_to_deserialize_any,
};

use crate::{Error, Result};

pub struct Deserializer<R: Read> {
    input: BufReader<R>,
    hold: Option<u8>,
}

impl<R: Read> Deserializer<R> {
    pub fn new(input: R) -> Self {
        Self {
            input: BufReader::new(input),
            hold: None,
        }
    }

    fn next_char(&mut self) -> Result<char> {
        if let Some(hold) = self.hold.take() {
            return Ok(hold as char);
        }
        let mut buf = [0];
        self.input.read_exact(&mut buf)?;
        Ok(buf[0] as char)
    }

    fn expect_next(&mut self, c: char) -> Result<()> {
        let next = self.next_char()?;
        if next == c {
            Ok(())
        } else {
            Err(Error::Unexpected {
                expected: Some(c.to_string()),
                found: next.to_string(),
            })
        }
    }

    fn peek_char(&mut self) -> Result<char> {
        if let Some(hold) = self.hold {
            return Ok(hold as char);
        }
        let mut buf = [0];
        self.input.read_exact(&mut buf)?;
        self.hold = Some(buf[0]);
        Ok(buf[0] as char)
    }

    fn get_integer(&mut self, mut buf: Vec<char>) -> Result<String> {
        loop {
            let next = self.peek_char();
            if let Err(Error::Io(e)) = next {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(Error::Io(e));
                }
            } else if !(next?).is_ascii_digit() {
                break;
            }
            buf.push(self.next_char()?);
        }
        Ok(buf.into_iter().collect::<String>())
    }

    fn parse_int<V: std::str::FromStr<Err = ParseIntError>>(&mut self) -> Result<V> {
        let mut buf = Vec::new();
        if self.peek_char()? == '-' {
            buf.push(self.next_char()?);
        }
        let string = self.get_integer(buf)?;
        if string == "-" {
            return Err(Error::Unexpected {
                found: "-".to_string(),
                expected: Some("number".to_string()),
            });
        }

        // Previous checks should guarentee that the string is parseable
        Ok(string.parse()?)
    }

    fn parse_uint<V: std::str::FromStr<Err = ParseIntError>>(&mut self) -> Result<V> {
        let string = self.get_integer(Vec::new())?;
        Ok(string.parse()?)
    }
}

impl<'de, R: Read> serde::Deserializer<'de> for &mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        loop {
            let out = match self.peek_char()? {
                '"' => self.deserialize_str(visitor),
                '[' => self.deserialize_seq(visitor),
                '{' => self.deserialize_map(visitor),
                'n' => self.deserialize_unit(visitor),
                't' | 'f' => self.deserialize_bool(visitor),
                '-' | '0'..='9' => todo!("Parse numbers"),
                ' ' => {
                    self.next_char()?;
                    continue;
                }
                c => Err(Error::Unexpected {
                    found: c.to_string(),
                    expected: None,
                }),
            };
            return out;
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.expect_next('"')?;

        let mut buf = Vec::new();
        loop {
            self.input.read_until(b'"', &mut buf)?;
            if buf.last() != Some(&b'"') {
                return Err(Error::Unclosed('"'));
            }
            let len = buf.len();
            let check_index = if len < 3 { len - 1 } else { len - 2 };
            if buf[check_index] != b'\\' {
                break;
            }
        }
        let s = String::from_utf8(buf)?;
        visitor.visit_str(&unescape(&s[..(s.len() - 1)])?)
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.expect_next('[')?;
        visitor.visit_seq(CommaSeparated::new(self))
    }

    fn deserialize_map<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.expect_next('{')?;
        visitor.visit_map(CommaSeparated::new(self))
    }

    fn deserialize_unit<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0; 4];
        self.input.read_exact(&mut buf)?;
        if buf.as_slice() != b"null" {
            Err(Error::Unexpected {
                found: String::from_utf8(buf.to_vec())?,
                expected: Some("null".to_string()),
            })
        } else {
            visitor.visit_unit()
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek_char()? {
            't' => {
                let mut buf = [0; 4];
                self.input.read_exact(&mut buf)?;
                if buf.as_slice() == b"true" {
                    visitor.visit_bool(true)
                } else {
                    Err(Error::Unexpected {
                        found: String::from_utf8(buf.to_vec())?,
                        expected: Some("true".to_string()),
                    })
                }
            }
            'f' => {
                let mut buf = [0; 5];
                self.input.read_exact(&mut buf)?;
                if buf.as_slice() == b"false" {
                    visitor.visit_bool(false)
                } else {
                    Err(Error::Unexpected {
                        found: String::from_utf8(buf.to_vec())?,
                        expected: Some("false".to_string()),
                    })
                }
            }
            c => Err(Error::Unexpected {
                found: c.to_string(),
                expected: Some("true or false".to_string()),
            }),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_int()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_int()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_int()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_int()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_uint()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_uint()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_uint()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_uint()?)
    }

    forward_to_deserialize_any! {char string bytes byte_buf option unit_struct newtype_struct tuple tuple_struct struct enum identifier ignored_any
    f32 f64}
}

struct CommaSeparated<'a, R: Read> {
    de: &'a mut Deserializer<R>,
    start: bool,
}

impl<'a, R: Read> CommaSeparated<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        Self { de, start: true }
    }
}

impl<'a, 'de, R: Read> SeqAccess<'de> for CommaSeparated<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::result::Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.de.peek_char()? == ']' {
            self.de.next_char()?;
            return Ok(None);
        }

        if !self.start {
            self.de.expect_next(',')?;
        } else {
            self.start = false;
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'a, 'de, R: Read> MapAccess<'de> for CommaSeparated<'a, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> std::result::Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.de.peek_char()? == '}' {
            self.de.next_char()?;
            return Ok(None);
        }

        if !self.start {
            self.de.expect_next(',')?;
        } else {
            self.start = false;
        }
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.de.expect_next(':')?;
        seed.deserialize(&mut *self.de)
    }
}

pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T> {
    let mut de = Deserializer::new(s.as_bytes());
    let t = T::deserialize(&mut de)?;
    Ok(t)
}

pub fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    let mut de = Deserializer::new(bytes);
    let t = T::deserialize(&mut de)?;
    Ok(t)
}

pub fn from_reader<T: DeserializeOwned>(reader: &mut impl Read) -> Result<T> {
    let mut de = Deserializer::new(reader);
    let t = T::deserialize(&mut de)?;
    Ok(t)
}

fn unescape(s: &str) -> Result<String> {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            let c = chars.next().unwrap();
            match c {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                'b' => out.push('\x08'),
                'f' => out.push('\x0c'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'u' => {
                    let mut buf = ['\0'; 2];
                    chars.next();
                    chars.next();
                    for item in &mut buf {
                        *item = chars.next().ok_or(Error::InvalidEscape)?;
                    }
                    out.push(
                        char::from_u32(u32::from_str_radix(&buf.iter().collect::<String>(), 16)?)
                            .ok_or(Error::InvalidEscape)?,
                    );
                }
                _ => out.push(c),
            }
        } else {
            out.push(c);
        }
    }
    Ok(out)
}
