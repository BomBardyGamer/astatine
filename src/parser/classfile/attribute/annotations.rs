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

const TAG_BYTE: u8 = 'B' as u8;
const TAG_CHAR: u8 = 'C' as u8;
const TAG_DOUBLE: u8 = 'D' as u8;
const TAG_FLOAT: u8 = 'F' as u8;
const TAG_INT: u8 = 'I' as u8;
const TAG_LONG: u8 = 'J' as u8;
const TAG_SHORT: u8 = 'S' as u8;
const TAG_BOOLEAN: u8 = 'Z' as u8;
const TAG_STRING: u8 = 's' as u8;
const TAG_ENUM: u8 = 'e' as u8;
const TAG_CLASS: u8 = 'c' as u8;
const TAG_ANNOTATION: u8 = '@' as u8;
const TAG_ARRAY: u8 = '[' as u8;
#[repr(u8)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum Tag {
    Byte = TAG_BYTE,
    Char = TAG_CHAR,
    Double = TAG_DOUBLE,
    Float = TAG_FLOAT,
    Int = TAG_INT,
    Long = TAG_LONG,
    Short = TAG_SHORT,
    Boolean = TAG_BOOLEAN,
    String = TAG_STRING,
    Enum = TAG_ENUM,
    Class = TAG_CLASS,
    Annotation = TAG_ANNOTATION,
    Array = TAG_ARRAY,
}

mod _parse {
    use crate::buf_read_named_type_vec;
    use crate::parser::{BinaryReader, Parse, ParserError};
    use super::*;

    macro_rules! impl_annotation_attr {
        ($name: ident, $err_msg: expr, $err_msg_idx: expr) => {
            impl Parse<$name> for $name {
                fn parse(buf: &mut BinaryReader) -> Result<$name, ParserError> {
                    buf_read_named_type_vec!(Annotation, annotations, buf, $err_msg, $err_msg_idx);
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
                fn parse(buf: &mut BinaryReader) -> Result<$name, ParserError> {
                    // copy and paste of buf_read_named_type_vec for type Vec<Annotation>
                    // as that macro does not support this properly
                    buf.check_bytes(2, $err_msg)?;

                    let mut annotations: Vec<Vec<Annotation>>;
                    {
                        // SAFETY: Guaranteed by check_bytes
                        let len = unsafe { buf.unsafe_read_u16() };
                        annotations = Vec::with_capacity(len as usize);

                        for i in 0..len {
                            annotations.push(parse_param_vec(buf).map_err(ParserError::wrap(format!($err_msg_idx, i)))?);
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

    fn parse_param_vec(buf: &mut BinaryReader) -> Result<Vec<Annotation>, ParserError> {
        buf_read_named_type_vec!(Annotation, result, buf, "annotations", "annotations - idx {}");
        Ok(result)
    }

    impl Parse<Annotation> for Annotation {
        fn parse(buf: &mut BinaryReader) -> Result<Annotation, ParserError> {
            buf.check_bytes(2, "type index")?;

            // SAFETY: Guaranteed by check_bytes
            let type_index = unsafe { buf.unsafe_read_u16() };
            buf_read_named_type_vec!(Element, elements, buf, "element", "element - idx {}");

            Ok(Annotation { type_index, elements })
        }
    }

    impl Parse<Element> for Element {
        fn parse(buf: &mut BinaryReader) -> Result<Element, ParserError> {
            buf.check_bytes(2, "name index")?;

            // SAFETY: Guaranteed by check_bytes
            let name_index = unsafe { buf.unsafe_read_u16() };
            let value = ElementValue::parse(buf)?;

            Ok(Element { name_index, value })
        }
    }

    impl Parse<ElementValue> for ElementValue {
        fn parse(buf: &mut BinaryReader) -> Result<ElementValue, ParserError> {
            buf.check_bytes(1, "value - tag")?;

            let tag = unsafe { buf.unsafe_read_u8() };
            match tag {
                TAG_BYTE => Ok(ElementValue::Byte(parse_const(buf)?)),
                TAG_CHAR => Ok(ElementValue::Char(parse_const(buf)?)),
                TAG_DOUBLE => Ok(ElementValue::Double(parse_const(buf)?)),
                TAG_FLOAT => Ok(ElementValue::Float(parse_const(buf)?)),
                TAG_INT => Ok(ElementValue::Int(parse_const(buf)?)),
                TAG_LONG => Ok(ElementValue::Long(parse_const(buf)?)),
                TAG_SHORT => Ok(ElementValue::Short(parse_const(buf)?)),
                TAG_BOOLEAN => Ok(ElementValue::Boolean(parse_const(buf)?)),
                TAG_STRING => Ok(ElementValue::String(parse_const(buf)?)),
                TAG_ENUM => Ok(ElementValue::EnumConst(parse_enum_const(buf)?)),
                TAG_CLASS => Ok(ElementValue::Class(parse_class(buf)?)),
                TAG_ANNOTATION => Ok(ElementValue::Annotation(parse_annotation(buf)?)),
                TAG_ARRAY => Ok(ElementValue::Array(parse_array(buf)?)),
                _ => ParserError::new(format!("value - invalid tag {tag}")).into()
            }
        }
    }

    fn parse_const(buf: &mut BinaryReader) -> Result<ConstValue, ParserError> {
        buf.check_bytes(2, "value - const - value index")?;

        let value_index = unsafe { buf.unsafe_read_u16() };
        Ok(ConstValue { value_index })
    }

    fn parse_enum_const(buf: &mut BinaryReader) -> Result<EnumConstValue, ParserError> {
        buf.check_bytes(2 + 2, "value - enum - type name index, const name index")?;

        let type_name_index = unsafe { buf.unsafe_read_u16() };
        let const_name_index = unsafe { buf.unsafe_read_u16() };

        Ok(EnumConstValue { type_name_index, const_name_index })
    }

    fn parse_class(buf: &mut BinaryReader) -> Result<ClassValue, ParserError> {
        buf.check_bytes(2, "value - class - info index")?;

        let info_index = unsafe { buf.unsafe_read_u16() };
        Ok(ClassValue { info_index })
    }

    fn parse_annotation(buf: &mut BinaryReader) -> Result<AnnotationValue, ParserError> {
        buf.check_bytes(2, "value - annotation")?;

        let value = Annotation::parse(buf)
            .map_err(ParserError::wrap("value - annotation"))?;

        Ok(AnnotationValue { value })
    }

    fn parse_array(buf: &mut BinaryReader) -> Result<ArrayValue, ParserError> {
        buf_read_named_type_vec!(ElementValue, values, buf,
            "value - array", "value - array - idx {}");
        Ok(ArrayValue { values })
    }
}
