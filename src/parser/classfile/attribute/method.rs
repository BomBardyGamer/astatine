use crate::parser::classfile::constantpool;

pub struct Exceptions {
    exception_indexes: Vec<constantpool::Index>,
}

impl Exceptions {
    pub fn indexes(&self) -> &[constantpool::Index] {
        &self.exception_indexes
    }
}

pub struct MethodParameters {
    parameters: Vec<MethodParameter>,
}

impl MethodParameters {
    pub fn parameters(&self) -> &[MethodParameter] {
        &self.parameters
    }
}

pub struct MethodParameter {
    name_index: constantpool::Index,
    access_flags: u16,
}

impl MethodParameter {
    pub fn name_index(&self) -> constantpool::Index {
        self.name_index
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }
}

mod _attr_name {
    use super::*;
    use crate::parser::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(Exceptions, EXCEPTIONS);
    impl_attr_name!(MethodParameters, METHOD_PARAMETERS);
}

mod _parse {
    use crate::parser::{BinaryReader, Parse, ParserError};
    use super::*;

    impl Parse<Exceptions> for Exceptions {
        fn parse(buf: &mut BinaryReader) -> Result<Exceptions, ParserError> {
            buf.check_bytes(2, "exceptions")?;

            // SAFETY: Guaranteed by check_bytes
            let len = unsafe { buf.unsafe_read_u16() };
            buf.check_bytes((len * 2) as usize, "exceptions")?;

            let mut exception_indexes = Vec::with_capacity(len as usize);
            // SAFETY: Guaranteed by check_bytes
            unsafe { buf.unsafe_read_u16_slice(&mut exception_indexes) }

            Ok(Exceptions { exception_indexes })
        }
    }

    impl Parse<MethodParameters> for MethodParameters {
        fn parse(buf: &mut BinaryReader) -> Result<MethodParameters, ParserError> {
            buf.check_bytes(2, "method parameters")?;

            // SAFETY: Guaranteed by check_bytes
            let count = unsafe { buf.unsafe_read_u16() };

            let mut parameters = Vec::with_capacity(count as usize);
            for _ in 0..count {
                parameters.push(MethodParameter::parse(buf)?);
            }

            Ok(MethodParameters { parameters })
        }
    }

    impl Parse<MethodParameter> for MethodParameter {
        fn parse(buf: &mut BinaryReader) -> Result<MethodParameter, ParserError> {
            // 2 name index, 2 access flags
            buf.check_bytes(2 + 2, "method parameter")?;

            // SAFETY: Guaranteed by check_bytes
            let name_index = unsafe { buf.unsafe_read_u16() };
            let access_flags = unsafe { buf.unsafe_read_u16() };
            Ok(MethodParameter { name_index, access_flags })
        }
    }
}
