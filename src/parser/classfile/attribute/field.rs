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
