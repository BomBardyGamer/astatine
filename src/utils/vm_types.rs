use std::fmt::{Display, Formatter};

pub const T_BOOLEAN: u8 = 4;
pub const T_CHAR: u8 = 5;
pub const T_FLOAT: u8 = 6;
pub const T_DOUBLE: u8 = 7;
pub const T_BYTE: u8 = 8;
pub const T_SHORT: u8 = 9;
pub const T_INT: u8 = 10;
pub const T_LONG: u8 = 11;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum ArrayType {
    Boolean = T_BOOLEAN,
    Char = T_CHAR,
    Float = T_FLOAT,
    Double = T_DOUBLE,
    Byte = T_BYTE,
    Short = T_SHORT,
    Int = T_INT,
    Long = T_LONG
}

impl ArrayType {
    pub fn from(value: u8) -> ArrayType {
        match value {
            T_BOOLEAN => ArrayType::Boolean,
            T_CHAR => ArrayType::Char,
            T_FLOAT => ArrayType::Float,
            T_DOUBLE => ArrayType::Double,
            T_BYTE => ArrayType::Byte,
            T_SHORT => ArrayType::Short,
            T_INT => ArrayType::Int,
            T_LONG => ArrayType::Long,
            _ => panic!("Invalid array type {}!", value)
        }
    }
}

impl Display for ArrayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
