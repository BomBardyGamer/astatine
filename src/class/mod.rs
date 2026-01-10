mod constantpool;
mod field;
mod method;
mod parse;

use std::cell::{Ref, RefCell};
use crate::types::{AccessFlags, Array};

pub struct Class {
    info: ClassInfo,
    constant_pool: constantpool::Pool,
    fields: Array<field::Field>,
    methods: Array<method::Method>,
}

pub struct ClassInfo {
    minor_version: u16,
    major_version: u16,
    access_flags: AccessFlags,
    descriptor: ClassDescriptor,
    super_class: constantpool::Index,
    interfaces: Array<constantpool::Index>,
}

pub struct ClassDescriptor {
    name: String,
    signature: String
}
