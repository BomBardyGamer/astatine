use crate::parser::classfile::attribute::names::{
    impl_attr_name, AttributeNames, NameableAttribute,
};
use crate::parser::classfile::constantpool::PoolIndex;

pub struct Exceptions {
    exception_indexes: Vec<PoolIndex>,
}
impl_attr_name!(Exceptions, EXCEPTIONS);

impl Exceptions {
    pub fn indexes(&self) -> &[PoolIndex] {
        &self.exception_indexes
    }
}

pub struct MethodParameters {
    parameters: Vec<MethodParameter>,
}
impl_attr_name!(MethodParameters, METHOD_PARAMETERS);

impl MethodParameters {
    pub fn parameters(&self) -> &[MethodParameter] {
        &self.parameters
    }
}

pub struct MethodParameter {
    name_index: PoolIndex,
    access_flags: u16,
}

impl MethodParameter {
    pub fn name_index(&self) -> PoolIndex {
        self.name_index
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }
}
