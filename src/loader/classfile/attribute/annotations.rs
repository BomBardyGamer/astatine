// Copyright (C) 2026 Callum Jay Seabrook Hefford (BomBardyGamer)
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, see <https://www.gnu.org/licenses/>.

use crate::loader::classfile::constantpool;
use crate::types::Array;

pub struct RuntimeVisible {
    annotations: Array<Annotation>,
}

pub struct RuntimeInvisible {
    annotations: Array<Annotation>,
}

pub struct ParameterRuntimeVisible {
    annotations: Array<Array<Annotation>>, // Annotations by parameter
}

pub struct ParameterRuntimeInvisible {
    annotations: Array<Array<Annotation>>, // Annotations by parameter
}

mod _attr_name {
    use super::*;
    use crate::loader::classfile::attribute::names::{Nameable, Names, impl_attr_name};

    impl_attr_name!(RuntimeVisible, RUNTIME_VISIBLE_ANNOTATIONS);
    impl_attr_name!(RuntimeInvisible, RUNTIME_INVISIBLE_ANNOTATIONS);
    impl_attr_name!(ParameterRuntimeVisible, RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS);
    impl_attr_name!(ParameterRuntimeInvisible, RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS);
}

pub struct Annotation {
    type_index: constantpool::Index,
    elements: Array<Element>,
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
    values: Array<ElementValue>,
}

pub struct Tag;

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

mod _parse {
    use crate::buf_read_named_type_arr;
    use crate::loader::{BinaryReader, Parse, ParseError};
    use crate::types::Array;
    use super::*;

    macro_rules! impl_annotation_attr {
        ($name: ident, $err_msg: expr, $err_msg_idx: expr) => {
            impl Parse<$name> for $name {
                fn parse(buf: &mut BinaryReader) -> Result<$name, ParseError> {
                    buf_read_named_type_arr!(Annotation, annotations, buf, $err_msg, $err_msg_idx);
                    Ok($name { annotations })
                }
            }
        };
    }
    impl_annotation_attr!(RuntimeVisible,
        "runtime visible annotations",
        "runtime visible annotations - idx {}");
    impl_annotation_attr!(RuntimeInvisible,
        "runtime invisible annotations",
        "runtime invisible annotations - idx {}");

    macro_rules! impl_annotation_param_attr {
        ($name: ident, $err_msg: expr, $err_msg_idx: expr) => {
            impl Parse<$name> for $name {
                fn parse(buf: &mut BinaryReader) -> Result<$name, ParseError> {
                    // copy and paste of buf_read_named_type_arr for type Array<Array<Annotation>>
                    // as that macro does not support this properly
                    buf.check_bytes(2, $err_msg)?;

                    let mut annotations: Array<Array<Annotation>>;
                    {
                        // SAFETY: Guaranteed by check_bytes
                        let len = unsafe { buf.unsafe_read_u16() } as usize;
                        // TODO: We shouldn't wrap this. When we have proper error handling,
                        //  propagate it.
                        annotations = Array::new(len)
                            .map_err(|_| ParseError::new("cannot allocate array"))?;

                        for i in 0..len {
                            let v = parse_param_arr(buf).map_err(ParseError::wrap(format!($err_msg_idx, i)))?;
                            annotations.set(i, v).expect("array set was somehow out of bounds");
                        }
                    }
                    Ok($name { annotations })
                }
            }
        };
    }
    impl_annotation_param_attr!(ParameterRuntimeVisible,
        "runtime visible parameter annotations - parameters",
        "runtime visible parameter annotations - parameter idx {}");
    impl_annotation_param_attr!(ParameterRuntimeInvisible,
        "runtime invisible parameter annotations - parameters",
        "runtime invisible parameter annotations - parameter idx {}");

    fn parse_param_arr(buf: &mut BinaryReader) -> Result<Array<Annotation>, ParseError> {
        buf_read_named_type_arr!(Annotation, result, buf, "annotations", "annotations - idx {}");
        Ok(result)
    }

    impl Parse<Annotation> for Annotation {
        fn parse(buf: &mut BinaryReader) -> Result<Annotation, ParseError> {
            buf.check_bytes(2, "type index")?;

            // SAFETY: Guaranteed by check_bytes
            let type_index = unsafe { buf.unsafe_read_u16() };
            buf_read_named_type_arr!(Element, elements, buf, "element", "element - idx {}");

            Ok(Annotation { type_index, elements })
        }
    }

    impl Parse<Element> for Element {
        fn parse(buf: &mut BinaryReader) -> Result<Element, ParseError> {
            buf.check_bytes(2, "name index")?;

            // SAFETY: Guaranteed by check_bytes
            let name_index = unsafe { buf.unsafe_read_u16() };
            let value = ElementValue::parse(buf)?;

            Ok(Element { name_index, value })
        }
    }

    impl Parse<ElementValue> for ElementValue {
        fn parse(buf: &mut BinaryReader) -> Result<ElementValue, ParseError> {
            buf.check_bytes(1, "value - tag")?;

            let tag = unsafe { buf.unsafe_read_u8() };
            match tag {
                Tag::BYTE => Ok(ElementValue::Byte(parse_const(buf)?)),
                Tag::CHAR => Ok(ElementValue::Char(parse_const(buf)?)),
                Tag::DOUBLE => Ok(ElementValue::Double(parse_const(buf)?)),
                Tag::FLOAT => Ok(ElementValue::Float(parse_const(buf)?)),
                Tag::INT => Ok(ElementValue::Int(parse_const(buf)?)),
                Tag::LONG => Ok(ElementValue::Long(parse_const(buf)?)),
                Tag::SHORT => Ok(ElementValue::Short(parse_const(buf)?)),
                Tag::BOOLEAN => Ok(ElementValue::Boolean(parse_const(buf)?)),
                Tag::STRING => Ok(ElementValue::String(parse_const(buf)?)),
                Tag::ENUM => Ok(ElementValue::EnumConst(parse_enum_const(buf)?)),
                Tag::CLASS => Ok(ElementValue::Class(parse_class(buf)?)),
                Tag::ANNOTATION => Ok(ElementValue::Annotation(parse_annotation(buf)?)),
                Tag::ARRAY => Ok(ElementValue::Array(parse_array(buf)?)),
                _ => ParseError::new(format!("value - invalid tag {tag}")).into()
            }
        }
    }

    fn parse_const(buf: &mut BinaryReader) -> Result<ConstValue, ParseError> {
        buf.check_bytes(2, "value - const - value index")?;

        let value_index = unsafe { buf.unsafe_read_u16() };
        Ok(ConstValue { value_index })
    }

    fn parse_enum_const(buf: &mut BinaryReader) -> Result<EnumConstValue, ParseError> {
        buf.check_bytes(2 + 2, "value - enum - type name index, const name index")?;

        let type_name_index = unsafe { buf.unsafe_read_u16() };
        let const_name_index = unsafe { buf.unsafe_read_u16() };

        Ok(EnumConstValue { type_name_index, const_name_index })
    }

    fn parse_class(buf: &mut BinaryReader) -> Result<ClassValue, ParseError> {
        buf.check_bytes(2, "value - class - info index")?;

        let info_index = unsafe { buf.unsafe_read_u16() };
        Ok(ClassValue { info_index })
    }

    fn parse_annotation(buf: &mut BinaryReader) -> Result<AnnotationValue, ParseError> {
        buf.check_bytes(2, "value - annotation")?;

        let value = Annotation::parse(buf)
            .map_err(ParseError::wrap("value - annotation"))?;

        Ok(AnnotationValue { value })
    }

    fn parse_array(buf: &mut BinaryReader) -> Result<ArrayValue, ParseError> {
        buf_read_named_type_arr!(ElementValue, values, buf,
            "value - array", "value - array - idx {}");
        Ok(ArrayValue { values })
    }
}
