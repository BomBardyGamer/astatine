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

pub struct AnnotationDefault {
    value: super::annotations::ElementValue
}

impl AnnotationDefault {
    pub fn value(&self) -> &super::annotations::ElementValue {
        &self.value
    }
}

mod _attr_name {
    use super::*;
    use crate::parser::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(Exceptions, EXCEPTIONS);
    impl_attr_name!(MethodParameters, METHOD_PARAMETERS);
}

mod _parse {
    use crate::{buf_read_named_type_vec, buf_read_u16_vec};
    use crate::parser::{BinaryReader, Parse, ParserError};
    use super::*;
    use super::super::annotations;

    impl Parse<Exceptions> for Exceptions {
        fn parse(buf: &mut BinaryReader) -> Result<Exceptions, ParserError> {
            buf_read_u16_vec!(exception_indexes, buf, "exceptions");
            Ok(Exceptions { exception_indexes })
        }
    }

    impl Parse<MethodParameters> for MethodParameters {
        fn parse(buf: &mut BinaryReader) -> Result<MethodParameters, ParserError> {
            buf_read_named_type_vec!(MethodParameter, parameters, buf,
                "method parameters", "method parameters - idx {}");
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

    impl Parse<AnnotationDefault> for AnnotationDefault {
        fn parse(buf: &mut BinaryReader) -> Result<AnnotationDefault, ParserError> {
            let value = annotations::ElementValue::parse(buf)
                .map_err(ParserError::wrap("annotation default - default value"))?;
            Ok(AnnotationDefault { value })
        }
    }
}
