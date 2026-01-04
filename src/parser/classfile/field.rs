use crate::parser::{Parse, ParserError};
use crate::parser::reader::BinaryReader;
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

fn field_err(err: ParserError) -> ParserError {
    ParserError::new(format!("bad field: {err:?}"))
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
