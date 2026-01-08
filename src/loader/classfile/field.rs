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

use crate::loader::{Parse, ParserError};
use crate::loader::reader::BinaryReader;
use super::{attribute, constantpool};

pub struct Field {
    access_flags: u16,
    name_index: constantpool::Index,
    descriptor_index: constantpool::Index,
    attributes: Vec<attribute::FieldAttribute>
}

#[repr(u16)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum AccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000, // Doesn't show up in source code
    Enum = 0x4000 // Represents enum constant
}

impl Parse<Field> for Field {
    fn parse(buf: &mut BinaryReader) -> Result<Field, ParserError> {
        // 2 access flags, 2 name index, 2 descriptor index
        buf.check_bytes(2 + 2 + 2, "access flags, name index, descriptor index").map_err(field_err)?;

        // SAFETY: Guaranteed by check_bytes
        let access_flags = unsafe { buf.unsafe_read_u16() };
        let name_index = unsafe { buf.unsafe_read_u16() };
        let descriptor_index = unsafe { buf.unsafe_read_u16() };

        Ok(Field {
            access_flags,
            name_index,
            descriptor_index,
            attributes: vec![] // TODO: Attributes
        })
    }
}

fn field_err(err: ParserError) -> ParserError {
    ParserError::new(format!("bad field: {err:?}"))
}
