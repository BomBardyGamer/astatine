use super::{attribute, constantpool};

pub struct Field {
    access_flags: u16,
    name_index: constantpool::Index,
    descriptor_index: constantpool::Index,
    attributes: attribute::FieldAttribute
}

#[repr(u16)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum AccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000, // Doesn't show up in source code
    Enum = 0x4000 // Represents enum constant
}
