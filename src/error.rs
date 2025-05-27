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
