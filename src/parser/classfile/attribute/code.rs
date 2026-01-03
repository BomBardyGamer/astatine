use crate::parser::classfile::attribute::CodeAttribute;
use crate::parser::classfile::attribute::stackmap;

pub struct Code {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exceptions: Vec<Exception>,
    attributes: Vec<CodeAttribute>,
}

impl Code {
    pub fn max_stack(&self) -> u16 {
        self.max_stack
    }

    pub fn max_locals(&self) -> u16 {
        self.max_locals
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn exceptions(&self) -> &[Exception] {
        &self.exceptions
    }

    pub fn attributes(&self) -> &[CodeAttribute] {
        &self.attributes
    }
}

pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl Exception {
    pub fn start_pc(&self) -> u16 {
        self.start_pc
    }

    pub fn end_pc(&self) -> u16 {
        self.end_pc
    }

    pub fn handler_pc(&self) -> u16 {
        self.handler_pc
    }

    pub fn catch_type(&self) -> u16 {
        self.catch_type
    }
}

pub struct StackMapTable {
    entries: Vec<stackmap::Frame>,
}

impl StackMapTable {
    pub fn entries(&self) -> &[stackmap::Frame] {
        &self.entries
    }
}

mod _attr_name {
    use super::*;
    use crate::parser::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(Code, CODE);
    impl_attr_name!(StackMapTable, STACK_MAP_TABLE);
}
