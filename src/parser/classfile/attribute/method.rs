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
