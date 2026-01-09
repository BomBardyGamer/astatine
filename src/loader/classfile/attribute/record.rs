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

use crate::loader::classfile::attribute::RecordAttribute;
use crate::loader::classfile::constantpool;

pub struct Record {
    components: Vec<Component>,
}

pub struct Component {
    name_index: constantpool::Index,
    descriptor_index: constantpool::Index,
    attributes: Vec<RecordAttribute>,
}

mod _attr_name {
    use super::*;
    use crate::loader::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(Record, RECORD);
}

mod _parse {
    use crate::buf_read_named_type_vec;
    use crate::loader::{BinaryReader, Parse, ParseError};
    use super::*;

    impl Parse<Record> for Record {
        fn parse(buf: &mut BinaryReader) -> Result<Record, ParseError> {
            buf_read_named_type_vec!(Component, components, buf,
                "record - components", "record - components - idx {}");
            Ok(Record { components })
        }
    }

    impl Parse<Component> for Component {
        fn parse(buf: &mut BinaryReader) -> Result<Component, ParseError> {
            buf.check_bytes(2 + 2, "name index, descriptor index")?;

            // SAFETY: Guaranteed by check_bytes
            let name_index = unsafe { buf.unsafe_read_u16() };
            let descriptor_index = unsafe { buf.unsafe_read_u16() };

            Ok(Component {
                name_index,
                descriptor_index,
                attributes: vec![], // TODO: Attributes
            })
        }
    }
}
