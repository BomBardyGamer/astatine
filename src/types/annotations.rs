// TODO: Figure out what to do about all the `index` fields in here.
//  Should we resolve them on resolution?
use crate::types::Array;

pub struct Annotation {
    type_index: u16,
    elements: Array<Element>,
}

pub struct Element {
    name_index: u16,
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
    pub fn tag(self) -> u8 {
        match self {
            ElementValue::Byte(_) => Tag::BYTE,
            ElementValue::Char(_) => Tag::CHAR,
            ElementValue::Double(_) => Tag::DOUBLE,
            ElementValue::Float(_) => Tag::FLOAT,
            ElementValue::Int(_) => Tag::INT,
            ElementValue::Long(_) => Tag::LONG,
            ElementValue::Short(_) => Tag::SHORT,
            ElementValue::Boolean(_) => Tag::BOOLEAN,
            ElementValue::String(_) => Tag::STRING,
            ElementValue::EnumConst(_) => Tag::ENUM,
            ElementValue::Class(_) => Tag::CLASS,
            ElementValue::Annotation(_) => Tag::ANNOTATION,
            ElementValue::Array(_) => Tag::ARRAY,
        }
    }
}

pub struct ConstValue {
    value_index: u16
}

pub struct EnumConstValue {
    type_name_index: u16,
    const_name_index: u16,
}

pub struct ClassValue {
    info_index: u16,
}

pub struct AnnotationValue {
    value: Annotation,
}

pub struct ArrayValue {
    values: Array<ElementValue>,
}

struct Tag;

impl Tag {
    pub const BYTE: u8 = 'B' as u8;
    pub const CHAR: u8 = 'C' as u8;
    pub const DOUBLE: u8 = 'D' as u8;
    pub const FLOAT: u8 = 'F' as u8;
    pub const INT: u8 = 'I' as u8;
    pub const LONG: u8 = 'J' as u8;
    pub const SHORT: u8 = 'S' as u8;
    pub const BOOLEAN: u8 = 'Z' as u8;
    pub const STRING: u8 = 's' as u8;
    pub const ENUM: u8 = 'e' as u8;
    pub const CLASS: u8 = 'c' as u8;
    pub const ANNOTATION: u8 = '@' as u8;
    pub const ARRAY: u8 = '[' as u8;
}
