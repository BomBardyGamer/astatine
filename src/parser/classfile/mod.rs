mod constantpool;
mod attribute;
mod parser;
mod field;
mod method;

pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: constantpool::Pool,
    access_flags: u16,
    this_class: constantpool::Index,
    super_class: constantpool::Index,
    interfaces: Vec<constantpool::Index>,
    fields: Vec<field::Field>,
    methods: Vec<method::Method>,
    attributes: Vec<attribute::ClassFileAttribute>
}

#[repr(u16)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum AccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
    Module = 0x8000,
}
