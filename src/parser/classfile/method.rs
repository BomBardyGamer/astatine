use crate::parser::{Parse, ParserError};
use crate::parser::reader::BinaryReader;
use super::{attribute, constantpool};

pub struct Method {
    access_flags: u16,
    name_index: constantpool::Index,
    descriptor_index: constantpool::Index,
    attributes: Vec<attribute::MethodAttribute>,
}

#[repr(u16)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum AccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040, // Prefixed 'bridge$' and used to access inner class privates
    Varargs = 0x0080, // Takes variable arguments in source code
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800, // `strictfp`
    Synthetic = 0x1000, // Doesn't show up in source code
}

impl Parse<Method> for Method {
    fn parse(buf: &mut BinaryReader) -> Result<Method, ParserError> {
        // 2 access flags, 2 name index, 2 descriptor index
        buf.check_bytes(2 + 2 + 2, "access flags, name index, descriptor index").map_err(method_err)?;

        // SAFETY: Guaranteed by check_bytes
        let access_flags = unsafe { buf.unsafe_read_u16() };
        let name_index = unsafe { buf.unsafe_read_u16() };
        let descriptor_index = unsafe { buf.unsafe_read_u16() };

        Ok(Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes: vec![] // TODO: Attributes
        })
    }
}

fn method_err(err: ParserError) -> ParserError {
    ParserError::new(format!("bad method: {err:?}"))
}
