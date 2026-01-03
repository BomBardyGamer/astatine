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
