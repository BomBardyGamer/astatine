use crate::parser::classfile::constantpool;

pub struct RuntimeVisible {
    annotations: Vec<Annotation>,
}

pub struct RuntimeInvisible {
    annotations: Vec<Annotation>,
}

pub struct ParameterRuntimeVisible {
    annotations: Vec<Vec<Annotation>>, // Annotations by parameter
}

pub struct ParameterRuntimeInvisible {
    annotations: Vec<Vec<Annotation>>, // Annotations by parameter
}

mod _attr_name {
    use super::*;
    use crate::parser::classfile::attribute::names::{Nameable, Names, impl_attr_name};

    impl_attr_name!(RuntimeVisible, RUNTIME_VISIBLE_ANNOTATIONS);
    impl_attr_name!(RuntimeInvisible, RUNTIME_INVISIBLE_ANNOTATIONS);
    impl_attr_name!(ParameterRuntimeVisible, RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS);
    impl_attr_name!(ParameterRuntimeInvisible, RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS);
}

pub struct Annotation {
    type_index: constantpool::Index,
    elements: Vec<Element>,
}

pub struct Element {
    name_index: constantpool::Index,
    value: ElementValue,
}

pub enum ElementValue {
    Byte(ConstValue),
    Char(ConstValue),
    Double(ConstValue),
    Float(ConstValue),
    Int(ConstValue),
    Long(ConstValue),
    Short(ConstValue),
    Boolean(ConstValue),
    String(ConstValue),
    EnumConst(EnumConstValue),
    Class(ClassValue),
    Annotation(AnnotationValue),
    Array(ArrayValue),
}

impl ElementValue {
    pub fn tag(self) -> Tag {
        match self {
            ElementValue::Byte(_) => Tag::Byte,
            ElementValue::Char(_) => Tag::Char,
            ElementValue::Double(_) => Tag::Double,
            ElementValue::Float(_) => Tag::Float,
            ElementValue::Int(_) => Tag::Int,
            ElementValue::Long(_) => Tag::Long,
            ElementValue::Short(_) => Tag::Short,
            ElementValue::Boolean(_) => Tag::Boolean,
            ElementValue::String(_) => Tag::String,
            ElementValue::EnumConst(_) => Tag::Enum,
            ElementValue::Class(_) => Tag::Class,
            ElementValue::Annotation(_) => Tag::Annotation,
            ElementValue::Array(_) => Tag::Array,
        }
    }
}

pub struct ConstValue {
    value_index: constantpool::Index
}

pub struct EnumConstValue {
    type_name_index: constantpool::Index,
    const_name_index: constantpool::Index,
}

pub struct ClassValue {
    info_index: constantpool::Index,
}

pub struct AnnotationValue {
    value: Annotation,
}

pub struct ArrayValue {
    values: Vec<ElementValue>,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Tag {
    Byte = 'B' as u8,
    Char = 'C' as u8,
    Double = 'D' as u8,
    Float = 'F' as u8,
    Int = 'I' as u8,
    Long = 'J' as u8,
    Short = 'S' as u8,
    Boolean = 'Z' as u8,
    String = 's' as u8,
    Enum = 'e' as u8,
    Class = 'c' as u8,
    Annotation = '@' as u8,
    Array = '[' as u8,
}
