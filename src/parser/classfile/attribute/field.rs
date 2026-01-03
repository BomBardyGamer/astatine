use crate::parser::classfile::attribute::names::{
    impl_attr_name, AttributeNames, NameableAttribute,
};
use crate::parser::classfile::constantpool::pool::PoolIndex;

pub struct ConstantValue {
    value_index: PoolIndex,
}
impl_attr_name!(ConstantValue, CONSTANT_VALUE);

impl ConstantValue {
    pub fn value_index(&self) -> PoolIndex {
        self.value_index
    }
}
