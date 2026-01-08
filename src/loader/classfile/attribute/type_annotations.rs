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

use crate::loader::classfile::attribute::annotations;
use crate::loader::classfile::constantpool::Index;

pub struct RuntimeVisible {
    annotations: Vec<TypeAnnotation>,
}

pub struct RuntimeInvisible {
    annotations: Vec<TypeAnnotation>,
}

mod _attr_name {
    use super::*;
    use crate::loader::classfile::attribute::names::{Nameable, Names, impl_attr_name};

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
    CastExpression = 0x47,
    GenericConstructorNewOrExplicitConstructorInvoke = 0x48,
    GenericMethodInvoke = 0x49,
    GenericConstructorMethodReferenceExpression = 0x4A,
    GenericMethodReferenceExpression = 0x4B,
}

pub enum TargetInfo {
    TypeParameter { index: u8 },
    Supertype { index: u16 },
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

mod _parse {
    use num_traits::FromPrimitive;
    use crate::buf_read_named_type_vec;
    use crate::loader::{BinaryReader, Parse, ParserError};
    use crate::loader::classfile::attribute::annotations::Element;
    use super::*;

    macro_rules! impl_type_annotation_attr {
        ($name: ident, $err_msg: expr, $err_msg_idx: expr) => {
            impl Parse<$name> for $name {
                fn parse(buf: &mut BinaryReader) -> Result<$name, ParserError> {
                    buf_read_named_type_vec!(TypeAnnotation, annotations, buf, $err_msg, $err_msg_idx);
                    Ok($name { annotations })
                }
            }
        };
    }
    impl_type_annotation_attr!(RuntimeVisible,
        "runtime visible type annotations",
        "runtime visible type annotations - idx {}");
    impl_type_annotation_attr!(RuntimeInvisible,
        "runtime invisible type annotations",
        "runtime invisible type annotations - idx {}");

    impl Parse<TypeAnnotation> for TypeAnnotation {
        fn parse(buf: &mut BinaryReader) -> Result<TypeAnnotation, ParserError> {
            buf.check_bytes(1, "type annotation - target type")?;

            // SAFETY: Guaranteed by check_bytes
            let raw_type = unsafe { buf.unsafe_read_u8() };

            let target_type = TargetType::from_u8(raw_type)
                .ok_or_else(|| {
                    let msg = format!("type annotation - invalid target type {raw_type}");
                    return ParserError::new(msg)
                })?;
            let target_info = parse_target_info(buf, target_type)?;

            buf_read_named_type_vec!(PathPart, target_path, buf,
                "type annotation - path", "type annotation - path - idx {}");

            buf.check_bytes(2, "type annotation - type index")?;
            // SAFETY: Guaranteed by check_bytes
            let type_index = unsafe { buf.unsafe_read_u16() };

            buf_read_named_type_vec!(Element, elements, buf,
                "type annotation - elements", "type annotation - elements - idx {}");

            Ok(TypeAnnotation {
                target_type,
                target_info,
                target_path,
                type_index,
                elements
            })
        }
    }

    fn parse_target_info(buf: &mut BinaryReader, target_type: TargetType) -> Result<TargetInfo, ParserError> {
        let r = match target_type {
            TargetType::Class | TargetType::Method => parse_info_type_param(buf),
            TargetType::Supertype => parse_info_supertype(buf),
            TargetType::ClassTypeBound |
                TargetType::MethodTypeBound => parse_info_type_param_bound(buf),
            TargetType::FieldOrRecord |
                TargetType::ReturnType |
                TargetType::Receiver => Ok(TargetInfo::Empty),
            TargetType::FormalParameter => parse_info_formal_param(buf),
            TargetType::Throws => parse_info_throws(buf),
            TargetType::LocalVar | TargetType::LocalResource =>  parse_info_local_var(buf),
            TargetType::CatchTarget => parse_info_catch(buf),
            TargetType::InstanceofExpression |
                TargetType::NewExpression |
                TargetType::MethodReferenceNewExpression |
                TargetType::MethodReferenceIdentifierExpression => parse_info_offset(buf),
            TargetType::CastExpression |
                TargetType::GenericConstructorNewOrExplicitConstructorInvoke |
                TargetType::GenericMethodInvoke |
                TargetType::GenericConstructorMethodReferenceExpression |
                TargetType::GenericMethodReferenceExpression => parse_info_type_argument(buf)
        };
        r.map_err(ParserError::wrap("type annotation"))
    }

    fn parse_info_type_param(buf: &mut BinaryReader) -> Result<TargetInfo, ParserError> {
        buf.check_bytes(1, "type parameter target - index")?;

        // SAFETY: Guaranteed by check_bytes
        let index = unsafe { buf.unsafe_read_u8() };
        Ok(TargetInfo::TypeParameter { index })
    }

    fn parse_info_supertype(buf: &mut BinaryReader) -> Result<TargetInfo, ParserError> {
        buf.check_bytes(2, "type parameter target - supertype")?;

        // SAFETY: Guaranteed by check_bytes
        let index = unsafe { buf.unsafe_read_u16() };
        Ok(TargetInfo::Supertype { index })
    }

    fn parse_info_type_param_bound(buf: &mut BinaryReader) -> Result<TargetInfo, ParserError> {
        buf.check_bytes(1 + 1, "type parameter bound target - type parameter index, bound index")?;

        // SAFETY: Guaranteed by check_bytes
        let type_parameter_index = unsafe { buf.unsafe_read_u8() };
        let bound_index = unsafe { buf.unsafe_read_u8() };

        Ok(TargetInfo::ParameterBound { type_parameter_index, bound_index })
    }

    fn parse_info_formal_param(buf: &mut BinaryReader) -> Result<TargetInfo, ParserError> {
        buf.check_bytes(1, "formal parameter target - index")?;

        // SAFETY: Guaranteed by check_bytes
        let index = unsafe { buf.unsafe_read_u8() };
        Ok(TargetInfo::FormalParameter { index })
    }

    fn parse_info_throws(buf: &mut BinaryReader) -> Result<TargetInfo, ParserError> {
        buf.check_bytes(2, "throws target - index")?;

        // SAFETY: Guaranteed by check_bytes
        let index = unsafe { buf.unsafe_read_u16() };
        Ok(TargetInfo::Throws { type_index: index })
    }

    fn parse_info_local_var(buf: &mut BinaryReader) -> Result<TargetInfo, ParserError> {
        buf_read_named_type_vec!(LocalVarInfoEntry, table, buf,
            "local var target - table", "local var target - table idx {}");
        Ok(TargetInfo::LocalVar { table })
    }

    fn parse_info_catch(buf: &mut BinaryReader) -> Result<TargetInfo, ParserError> {
        buf.check_bytes(2, "catch target - exception table index")?;

        // SAFETY: Guaranteed by check_bytes
        let exception_table_index = unsafe { buf.unsafe_read_u16() };
        Ok(TargetInfo::Catch { exception_table_index })
    }

    fn parse_info_offset(buf: &mut BinaryReader) -> Result<TargetInfo, ParserError> {
        buf.check_bytes(2, "offset target - offset")?;

        // SAFETY: Guaranteed by check_bytes
        let offset = unsafe { buf.unsafe_read_u16() };
        Ok(TargetInfo::Offset(offset))
    }

    fn parse_info_type_argument(buf: &mut BinaryReader) -> Result<TargetInfo, ParserError> {
        buf.check_bytes(2 + 1, "type argument target - offset, index")?;

        // SAFETY: Guaranteed by check_bytes
        let offset = unsafe { buf.unsafe_read_u16() };
        let index = unsafe { buf.unsafe_read_u8() };

        Ok(TargetInfo::TypeArgument { offset, index })
    }

    impl Parse<PathPart> for PathPart {
        fn parse(buf: &mut BinaryReader) -> Result<PathPart, ParserError> {
            buf.check_bytes(1 + 1, "type path kind, type argument index")?;

            // SAFETY: Guaranteed by check_bytes
            let type_path_kind = unsafe { buf.unsafe_read_u8() };
            let type_argument_index = unsafe { buf.unsafe_read_u8() };

            Ok(PathPart { type_path_kind, type_argument_index })
        }
    }

    impl Parse<LocalVarInfoEntry> for LocalVarInfoEntry {
        fn parse(buf: &mut BinaryReader) -> Result<LocalVarInfoEntry, ParserError> {
            buf.check_bytes(2 + 2 + 2, "local var - start pc, length, index")?;

            // SAFETY: Guaranteed by check_bytes
            let start_pc = unsafe { buf.unsafe_read_u16() };
            let length = unsafe { buf.unsafe_read_u16() };
            let index = unsafe { buf.unsafe_read_u16() };

            Ok(LocalVarInfoEntry { start_pc, length, index })
        }
    }
}
