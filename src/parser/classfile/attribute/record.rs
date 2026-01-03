use crate::parser::classfile::attribute::RecordAttribute;
use crate::parser::classfile::attribute::names::{
    impl_attr_name, AttributeNames, NameableAttribute,
};
use crate::parser::classfile::constantpool::PoolIndex;

pub struct Record {
    components: Vec<RecordComponent>,
}
impl_attr_name!(Record, RECORD);

pub struct RecordComponent {
    name_index: PoolIndex,
    descriptor_index: PoolIndex,
    attributes: Vec<RecordAttribute>,
}
