use std::num::ParseIntError;

use serde::{de, ser};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error")]
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("Key is not a string")]
    KeyNotString,
    #[error("Unclosed delimiter {0}")]
    Unclosed(char),
    #[error("Failed to read UTF-8")]
    Utf8(
        #[from]
        #[source]
        std::string::FromUtf8Error,
    ),
    #[error("Unexpected character {found}{}", if let Some(expected) = expected {format!(" (expected `{expected}`)")} else {"".to_string()})]
    Unexpected {
        found: String,
        expected: Option<String>,
    },
    #[error("Trailing comma")]
    TrailingComma,
    #[error("Invalid escape sequence")]
    InvalidEscape,
    #[error("Failed to parse integer")]
    ParseInt(
        #[from]
        #[source]
        ParseIntError,
    ),

    #[error("{0}")]
    Message(String),
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
