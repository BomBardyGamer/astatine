use crate::parser::classfile::attribute::annotations;
use crate::parser::classfile::constantpool::Index;

pub struct RuntimeVisible {
    annotations: Vec<TypeAnnotation>,
}

pub struct RuntimeInvisible {
    annotations: Vec<TypeAnnotation>,
}

mod _attr_name {
    use super::*;
    use crate::parser::classfile::attribute::names::{Nameable, Names, impl_attr_name};

    impl_attr_name!(RuntimeVisible, RUNTIME_VISIBLE_TYPE_ANNOTATIONS);
    impl_attr_name!(RuntimeInvisible, RUNTIME_INVISIBLE_TYPE_ANNOTATIONS);
}

pub struct TypeAnnotation {
    target_type: TargetType,
    target_info: TargetInfo,
    target_path: Path,
    type_index: Index,
    elements: Vec<annotations::Element>,
}

pub type Path = Vec<PathPart>;

pub struct PathPart {
    type_path_kind: u8,
    type_argument_index: u8
}

#[repr(u8)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum TargetType {
    Class = 0x00, // Only a ClassFile attribute
    Method = 0x01, // Only a Method attribute

    Supertype = 0x10, // Only a ClassFile attribute
    ClassTypeBound = 0x11, // Only a ClassFile attribute
    MethodTypeBound = 0x12, // Only a Method attribute
    FieldOrRecord = 0x13, // Only a Field or RecordComponent attribute
    ReturnType = 0x14, // Only a Method attribute
    Receiver = 0x15, //  Only a Method attribute
    FormalParameter = 0x16, // Only a Method attribute
    Throws = 0x17, // Only a Method attribute

    // All the following only appear in Code attributes
    LocalVar = 0x40,
    LocalResource = 0x41,
    CatchTarget = 0x42,
    InstanceofExpression = 0x43,
    NewExpression = 0x44,
    MethodReferenceNewExpression = 0x45,
    MethodReferenceIdentifierExpression = 0x46,
    CastExpr = 0x47,
    GenericConstructorNewOrExplicitConstructorInvoke = 0x48,
    GenericMethodInvoke = 0x49,
    GenericConstructorMethodReferenceExpression = 0x4A,
    GenericMethodReferenceExpression = 0x4B,
}

pub enum TargetInfo {
    TypeParameter { index: u8 },
    Supertype { index: u8 },
    ParameterBound { type_parameter_index: u8, bound_index: u8 },
    Empty,
    FormalParameter { index: u8 },
    Throws { type_index: u16 },
    LocalVar { table: Vec<LocalVarInfoEntry> },
    Catch { exception_table_index: u16 },
    Offset(u16),
    TypeArgument { offset: u16, index: u8 },
}

pub struct LocalVarInfoEntry {
    start_pc: u16,
    length: u16,
    index: u16
}
