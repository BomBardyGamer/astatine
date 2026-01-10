mod reader;
mod class;

pub use reader::BinaryReader;

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParseError {
    msg: String
}

impl ParseError {
    pub fn new(msg: impl Into<String>) -> ParseError {
        Self { msg: msg.into() }
    }

    pub fn wrap(wrapping_msg: impl Into<String>) -> impl Fn(ParseError) -> ParseError {
        let msg = wrapping_msg.into();
        move |err| ParseError::new(format!("{msg}: {err:?}"))
    }

    pub fn not_enough_bytes<T>(msg: impl Into<String>) -> Result<T, ParseError> {
        let msg: String = msg.into();
        let full_msg = format!("Not enough bytes for {msg}");
        Err(Self { msg: full_msg })
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl<T> Into<Result<T, ParseError>> for ParseError {
    fn into(self) -> Result<T, ParseError> {
        Err(self)
    }
}
