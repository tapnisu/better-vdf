use std::fmt::Display;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Eof,
    UnsupportedType,
    TrailingCharacters,
    ExpectedBoolean,
    ExpectedString,
    ExpectedInteger,
    NonSelfDescribing,
    ExpectedArray,
    ExpectedArrayEnd,
    ArrayIndex,
    ExpectedMap,
    ExpectedMapEnd,
    MapSyntax,
    SeqSyntax,
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::Eof => f.write_str("unexpected end of string"),
            Error::UnsupportedType => f.write_str("unsupported data type"),
            _ => todo!(),
        }
    }
}

impl std::error::Error for Error {}
