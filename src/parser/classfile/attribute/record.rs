use crate::parser::classfile::attribute::RecordAttribute;
use crate::parser::classfile::constantpool;

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
    use crate::parser::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(Record, RECORD);
}

mod _parse {
    use crate::buf_read_named_type_vec;
    use crate::parser::{BinaryReader, Parse, ParserError};
    use super::*;

    impl Parse<Record> for Record {
        fn parse(buf: &mut BinaryReader) -> Result<Record, ParserError> {
            buf_read_named_type_vec!(Component, components, buf,
                "record - components", "record - components - idx {}");
            Ok(Record { components })
        }
    }

    impl Parse<Component> for Component {
        fn parse(buf: &mut BinaryReader) -> Result<Component, ParserError> {
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
