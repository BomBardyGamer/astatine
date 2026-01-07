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

use crate::parser::classfile::constantpool;

pub struct ConstantValue {
    value_index: constantpool::Index,
}

impl ConstantValue {
    pub fn value_index(&self) -> constantpool::Index {
        self.value_index
    }
}

mod _attr_name {
    use super::*;
    use crate::parser::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(ConstantValue, CONSTANT_VALUE);
}

mod _parse {
    use crate::parser::{BinaryReader, Parse, ParserError};
    use super::*;

    impl Parse<ConstantValue> for ConstantValue {
        fn parse(buf: &mut BinaryReader) -> Result<ConstantValue, ParserError> {
            buf.check_bytes(2, "constant value")?;

            // SAFETY: Guaranteed by check_bytes
            let value_index = unsafe { buf.unsafe_read_u16() };
            Ok(ConstantValue { value_index })
        }
    }
}
