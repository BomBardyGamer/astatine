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
