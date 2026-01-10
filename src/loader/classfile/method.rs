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

use crate::loader::{Parse, ParseError};
use crate::loader::reader::BinaryReader;
use crate::types::{AccessFlags, Array};
use super::{attribute, constantpool};

pub struct Method {
    access_flags: AccessFlags,
    name_index: constantpool::Index,
    descriptor_index: constantpool::Index,
    attributes: Array<attribute::MethodAttribute>,
}

impl Parse<Method> for Method {
    fn parse(buf: &mut BinaryReader) -> Result<Method, ParseError> {
        // 2 access flags, 2 name index, 2 descriptor index
        buf.check_bytes(2 + 2 + 2, "access flags, name index, descriptor index").map_err(method_err)?;

        // SAFETY: Guaranteed by check_bytes
        let flags = unsafe { buf.unsafe_read_u16() };
        let name_index = unsafe { buf.unsafe_read_u16() };
        let descriptor_index = unsafe { buf.unsafe_read_u16() };

        Ok(Method {
            access_flags: AccessFlags::new(flags),
            name_index,
            descriptor_index,
            attributes: Array::empty() // TODO: Attributes
        })
    }
}

fn method_err(err: ParseError) -> ParseError {
    ParseError::new(format!("bad method: {err:?}"))
}
