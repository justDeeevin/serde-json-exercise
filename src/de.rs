use serde::de::DeserializeOwned;

use crate::Result;

pub struct Deserializer;

pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T> {
    todo!()
}

pub fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    todo!()
}

pub fn from_reader<T: DeserializeOwned>(reader: &mut impl std::io::Read) -> Result<T> {
    todo!()
}
