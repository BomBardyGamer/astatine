// Copyright (C) 2026 Callum Jay Seabrook Hefford (BomBardyGamer)
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, see <https://www.gnu.org/licenses/>.

pub mod classfile;
mod reader;

pub use reader::BinaryReader;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub trait Parse<T> {
    fn parse(buf: &mut BinaryReader) -> Result<T, ParseError>;
}

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
