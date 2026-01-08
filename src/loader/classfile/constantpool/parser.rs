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

use num_traits::FromPrimitive;
use super::*;
use crate::loader::{Parse, ParserError};
use crate::loader::reader::BinaryReader;
use crate::types::methodhandle;

impl Parse<Pool> for Pool {
    fn parse(buf: &mut BinaryReader) -> Result<Pool, ParserError> {
        buf.check_bytes(2, "constant pool")?;

        // SAFETY: Guaranteed by check_bytes
        let len = unsafe { buf.unsafe_read_u16() } as usize;
        let mut pool = Pool { constants: Vec::with_capacity(len) };

        let mut idx = 0;
        while idx < len {
            buf.check_bytes(1, format!("constant pool tag at {idx}"))?;

            let tag = unsafe { buf.unsafe_read_u8() };
            let r = parse_entry(buf, tag);

            let entry = r.map_err(|err| {
                let msg = format!("bad constant pool entry {idx}: {err}");
                return ParserError::new(msg);
            })?;

            pool.put(idx as u16, entry);
            idx += 1;

            if tag == TAG_LONG || tag == TAG_DOUBLE {
                // Long and double take up 2 entries in constant pool
                // Ref: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5
                pool.put((idx + 1) as u16, Entry::Unusable);

                // Add another 1 to the index for the unusable entry
                idx += 1;
            }
        }

        Ok(pool)
    }
}

fn parse_entry(buf: &mut BinaryReader, tag: u8) -> Result<Entry, ParserError> {
    match tag {
        TAG_UTF8 => Utf8Info::parse(buf).map(Entry::Utf8),
        TAG_INTEGER => IntegerInfo::parse(buf).map(Entry::Integer),
        TAG_FLOAT => FloatInfo::parse(buf).map(Entry::Float),
        TAG_LONG => LongInfo::parse(buf).map(Entry::Long),
        TAG_DOUBLE => DoubleInfo::parse(buf).map(Entry::Double),
        TAG_CLASS => ClassInfo::parse(buf).map(Entry::Class),
        TAG_STRING => StringInfo::parse(buf).map(Entry::String),
        TAG_FIELDREF => FieldrefInfo::parse(buf).map(Entry::Fieldref),
        TAG_METHODREF => MethodrefInfo::parse(buf).map(Entry::Methodref),
        TAG_INTERFACE_METHODREF => InterfaceMethodrefInfo::parse(buf).map(Entry::InterfaceMethodref),
        TAG_NAME_AND_TYPE => NameAndTypeInfo::parse(buf).map(Entry::NameAndType),
        TAG_METHOD_HANDLE => MethodHandleInfo::parse(buf).map(Entry::MethodHandle),
        TAG_METHOD_TYPE => MethodTypeInfo::parse(buf).map(Entry::MethodType),
        TAG_DYNAMIC => DynamicInfo::parse(buf).map(Entry::Dynamic),
        TAG_INVOKE_DYNAMIC => InvokeDynamicInfo::parse(buf).map(Entry::InvokeDynamic),
        TAG_MODULE => ModuleInfo::parse(buf).map(Entry::Module),
        TAG_PACKAGE => PackageInfo::parse(buf).map(Entry::Package),
        _ => ParserError::new("invalid entry tag {tag}").into(),
    }
}

impl Parse<Utf8Info> for Utf8Info {
    fn parse(buf: &mut BinaryReader) -> Result<Utf8Info, ParserError> {
        buf.check_bytes(2, "utf8")?;

        // SAFETY: Guaranteed by check_bytes
        let len = unsafe { buf.unsafe_read_u16() } as usize;
        buf.check_bytes(len, "utf8")?;

        let mut bytes = Vec::with_capacity(len);
        buf.read(&mut bytes);

        Ok(Utf8Info { bytes })
    }
}

macro_rules! parse_num32 {
    ($name: ident) => {
        impl Parse<$name> for $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParserError> {
                buf.check_bytes(4, "integer/float")?;

                // SAFETY: Guaranteed by check_bytes
                let bytes = unsafe { buf.unsafe_read_u32() };
                Ok($name { bytes })
            }
        }
    };
}
parse_num32!(IntegerInfo);
parse_num32!(FloatInfo);

macro_rules! parse_num64 {
    ($name: ident) => {
        impl Parse<$name> for $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParserError> {
                // 4 for high bytes, 4 for low bytes
                buf.check_bytes(4 + 4, "integer/float")?;

                // SAFETY: Guaranteed by check_bytes
                let high_bytes = unsafe { buf.unsafe_read_u32() };
                let low_bytes = unsafe { buf.unsafe_read_u32() };

                Ok($name { bytes: ((high_bytes as u64) << 32) | (low_bytes as u64) })
            }
        }
    };
}
parse_num64!(LongInfo);
parse_num64!(DoubleInfo);

impl Parse<StringInfo> for StringInfo {
    fn parse(buf: &mut BinaryReader) -> Result<StringInfo, ParserError> {
        buf.check_bytes(2, "string")?;

        // SAFETY: Guaranteed by check_bytes
        let string_index = unsafe { buf.unsafe_read_u16() };
        Ok(StringInfo { string_index })
    }
}

macro_rules! parse_ref {
    ($name: ident, $typ: expr) => {
        impl Parse<$name> for $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParserError> {
                // 2 for class index, 2 for name and type index
                buf.check_bytes(2 + 2, $typ)?;

                // SAFETY: Guaranteed by check_bytes
                let class_index = unsafe { buf.unsafe_read_u16() };
                let name_and_type_index = unsafe { buf.unsafe_read_u16() };

                Ok($name { class_index, name_and_type_index })
            }
        }
    };
}
parse_ref!(FieldrefInfo, "fieldref");
parse_ref!(MethodrefInfo, "methodref");
parse_ref!(InterfaceMethodrefInfo, "interface methodref");

impl Parse<NameAndTypeInfo> for NameAndTypeInfo {
    fn parse(buf: &mut BinaryReader) -> Result<NameAndTypeInfo, ParserError> {
        // 2 for name index, 2 for descriptor index
        buf.check_bytes(2 + 2, "name and type")?;

        // SAFETY: Guaranteed by check_bytes
        let name_index = unsafe { buf.unsafe_read_u16() };
        let descriptor_index = unsafe { buf.unsafe_read_u16() };

        Ok(NameAndTypeInfo { name_index, descriptor_index })
    }
}

impl Parse<MethodHandleInfo> for MethodHandleInfo {
    fn parse(buf: &mut BinaryReader) -> Result<MethodHandleInfo, ParserError> {
        // 1 for reference kind, 2 for referenxe index
        buf.check_bytes(1 + 2, "method handle")?;

        // SAFETY: Guaranteed by check_bytes
        let ref_kind = unsafe { buf.unsafe_read_u8() };
        let reference_kind = methodhandle::Ref::from_u8(ref_kind)
            .ok_or_else(|| ParserError::new("method handle - reference kind invalid"))?;
        let reference_index = unsafe { buf.unsafe_read_u16() };

        Ok(MethodHandleInfo { reference_kind, reference_index })
    }
}

impl Parse<MethodTypeInfo> for MethodTypeInfo {
    fn parse(buf: &mut BinaryReader) -> Result<MethodTypeInfo, ParserError> {
        buf.check_bytes(2, "method type")?;

        // SAFETY: Guaranteed by check_bytes
        let descriptor_index = unsafe { buf.unsafe_read_u16() };
        Ok(MethodTypeInfo { descriptor_index })
    }
}

macro_rules! parse_dynamic {
    ($name: ident, $typ: expr) => {
        impl Parse<$name> for $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParserError> {
                // 2 for bootstrap method attribute index, 2 for name and type index
                buf.check_bytes(2 + 2, "dynamic")?;

                // SAFETY: Guaranteed by check_bytes
                let bootstrap_method_attr_index = unsafe { buf.unsafe_read_u16() };
                let name_and_type_index = unsafe { buf.unsafe_read_u16() };

                Ok($name { bootstrap_method_attr_index, name_and_type_index })
            }
        }
    };
}
parse_dynamic!(DynamicInfo, "dynamic");
parse_dynamic!(InvokeDynamicInfo, "invokedynamic");

macro_rules! parse_nameable {
    ($name: ident, $typ: expr) => {
        impl Parse<$name> for $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParserError> {
                buf.check_bytes(2, $typ)?;

                // SAFETY: Guaranteed by check_bytes
                let name_index = unsafe { buf.unsafe_read_u16() };
                Ok($name { name_index })
            }
        }
    };
}
parse_nameable!(ClassInfo, "class");
parse_nameable!(ModuleInfo, "module");
parse_nameable!(PackageInfo, "package");
