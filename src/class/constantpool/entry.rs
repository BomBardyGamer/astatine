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

pub use _parse::parse_entry;

use crate::class::constantpool::Pool;
use crate::types::{methodhandle, Array, Jdouble, Jfloat, Jint, Jlong};

macro_rules! tag {
    ($name: ident, $tag: ident) => {
        impl $name {
            pub const fn tag() -> super::Tag {
                super::Tag::$tag
            }
        }
    };
}

macro_rules! name_info {
    ($name: ident, $tag: ident) => {
        #[derive(Copy, Clone)]
        pub struct $name {
            name_index: super::Index,
        }
        tag!($name, $tag);

        impl $name {
            pub(super) fn new(name_index: super::Index) -> Self {
                Self { name_index }
            }
        }
    };
}

name_info!(UnresolvedClassInfo, Class);

impl UnresolvedClassInfo {
    pub fn resolve(&self, pool: &Pool) -> ClassInfo {
        // TODO: Better handle this error
        let utf8_name = pool.resolve_utf8(self.name_index)
            .expect("name index for unresolved class info not in constant pool!");
        ClassInfo { name: utf8_name.as_string() }
    }
}

pub struct ClassInfo {
    name: String
}

impl ClassInfo {
    pub fn name(&self) -> String {
        // TODO: This isn't ideal as it clones every time. We need to pool strings in
        //  the future, so will sort out when we do that
        self.name.clone()
    }

    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }
}

macro_rules! ref_entry {
    ($name: ident, $tag: ident) => {
        #[derive(Copy, Clone)]
        pub struct $name {
            class_index: super::Index,
            name_and_type_index: super::Index,
        }
        tag!($name, $tag);

        impl $name {
            pub fn new(class_index: super::Index, name_and_type_index: super::Index) -> Self {
                Self { class_index, name_and_type_index }
            }
        }
    };
}

ref_entry!(FieldrefInfo, Fieldref);
ref_entry!(MethodrefInfo, Methodref);
ref_entry!(InterfaceMethodrefInfo, InterfaceMethodref);

pub struct UnresolvedStringInfo {
    string_index: super::Index,
}
tag!(UnresolvedStringInfo, String);

impl UnresolvedStringInfo {
    pub fn new(string_index: super::Index) -> Self {
        Self { string_index }
    }

    pub fn resolve(&self, pool: &Pool) -> StringInfo {
        // TODO: Better handle this error
        let utf8 = pool.resolve_utf8(self.string_index)
            .expect("string index for unresolved string info not in constant pool!");
        StringInfo { value: utf8.as_string() }
    }
}

pub struct StringInfo {
    value: String
}

impl StringInfo {
    pub fn as_string(&self) -> String {
        // TODO: This isn't ideal as it clones every time. We need to pool strings in
        //  the future, so will sort out when we do that
        self.value.clone()
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

pub struct IntegerInfo {
    value: i32,
}
tag!(IntegerInfo, Integer);

impl IntegerInfo {
    pub(super) const fn from_bytes(v: u32) -> Self {
        Self { value: v as i32 }
    }

    pub const fn value(&self) -> Jint {
        self.value
    }
}

impl Into<Jint> for IntegerInfo {
    fn into(self) -> Jint {
        self.value
    }
}

pub struct FloatInfo {
    value: f32,
}

impl FloatInfo {
    pub(super) const fn from_bytes(v: u32) -> Self {
        Self { value: f32::from_bits(v) }
    }

    pub const fn value(&self) -> Jfloat {
        self.value
    }
}

impl Into<Jfloat> for FloatInfo {
    fn into(self) -> Jfloat {
        self.value
    }
}

pub struct LongInfo {
    value: i64,
}

impl LongInfo {
    pub(super) const fn from_bytes(low: u32, high: u32) -> Self {
        let v = (high as u64) << 32 | (low as u64);
        Self { value: v as i64 }
    }

    pub const fn value(&self) -> Jlong {
        self.value
    }
}

impl Into<Jlong> for LongInfo {
    fn into(self) -> Jlong {
        self.value
    }
}

pub struct DoubleInfo {
    value: f64,
}

impl DoubleInfo {
    pub(super) const fn from_bytes(low: u32, high: u32) -> Self {
        let v = (high as u64) << 32 | (low as u64);
        Self { value: f64::from_bits(v) }
    }

    pub const fn value(&self) -> Jdouble {
        self.value
    }
}

impl Into<Jdouble> for DoubleInfo {
    fn into(self) -> Jdouble {
        self.value
    }
}

pub struct UnresolvedNameAndTypeInfo {
    name_index: super::Index,
    descriptor_index: super::Index,
}
tag!(UnresolvedNameAndTypeInfo, NameAndType);

impl UnresolvedNameAndTypeInfo {
    pub(super) fn new(name_index: super::Index, descriptor_index: super::Index) -> Self {
        Self { name_index, descriptor_index }
    }

    pub fn resolve(&self, pool: &Pool) -> NameAndTypeInfo {
        // TODO: Better handle this error
        let utf8_name = pool.resolve_utf8(self.name_index)
            .expect("name index for unresolved name and type not in constant pool!");
        let name = utf8_name.as_string();

        // TODO: Better handle this error
        let utf8_descriptor = pool.resolve_utf8(self.descriptor_index)
            .expect("descriptor index for unresolved name and type not in constant pool!");
        let descriptor = utf8_descriptor.as_string();

        NameAndTypeInfo { name, descriptor }
    }
}

pub struct NameAndTypeInfo {
    name: String,
    descriptor: String
}

impl NameAndTypeInfo {
    pub fn name(&self) -> String {
        // TODO: This isn't ideal as it clones every time. We need to pool strings in
        //  the future, so will sort out when we do that
        self.name.clone()
    }

    pub fn descriptor(&self) -> String {
        // TODO: This isn't ideal as it clones every time. We need to pool strings in
        //  the future, so will sort out when we do that
        self.descriptor.clone()
    }
}

pub struct UnresolvedUtf8Info {
    bytes: Array<u8>
}
tag!(UnresolvedUtf8Info, Utf8);

impl UnresolvedUtf8Info {
    pub(super) fn new(bytes: Array<u8>) -> Self {
        Self { bytes }
    }

    fn as_string(&self) -> String {
        // SAFETY: The bytes Array is definitely initialized
        let vec = unsafe { self.bytes.to_vec() };

        // SAFETY: This is safe for 99% of cases.
        // TODO: must figure out what to do about nulls being 2 bytes and any
        //  chars more than 3 bytes
        unsafe { String::from_utf8_unchecked(vec) }
    }

    pub(super) fn resolve(&self) -> Utf8Info {
        let string = self.as_string();
        Utf8Info { value: string }
    }
}

pub struct Utf8Info {
    value: String
}

impl Utf8Info {
    pub fn as_string(&self) -> String {
        // TODO: This isn't ideal as it clones every time. We need to pool strings in
        //  the future, so will sort out when we do that
        self.value.clone()
    }
}

pub struct MethodHandleInfo {
    reference_kind: methodhandle::Ref,
    reference_index: super::Index
}
tag!(MethodHandleInfo, MethodHandle);

impl MethodHandleInfo {
    pub(super) fn new(reference_kind: methodhandle::Ref, reference_index: super::Index) -> Self {
        Self { reference_kind, reference_index }
    }
}

pub struct MethodTypeInfo {
    descriptor_index: super::Index,
}
tag!(MethodTypeInfo, MethodType);

impl MethodTypeInfo {
    pub(super) fn new(descriptor_index: super::Index) -> Self {
        Self { descriptor_index }
    }
}

macro_rules! dynamic {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            bootstrap_method_attr_index: super::Index,
            name_and_type_index: super::Index,
        }
        tag!($name, $tag);

        impl $name {
            pub fn new(bootstrap_method_attr_index: super::Index,
                name_and_type_index: super::Index) -> Self {
                Self { bootstrap_method_attr_index, name_and_type_index }
            }
        }
    };
}

dynamic!(DynamicInfo, Dynamic);
dynamic!(InvokeDynamicInfo, InvokeDynamic);

name_info!(ModuleInfo, Module);
name_info!(PackageInfo, Package);

mod _parse {
    use num_traits::FromPrimitive;
    use crate::class::constantpool::{Entry, Tag};
    use crate::class::parse::{BinaryReader, ParseError};
    use crate::types::{methodhandle, Array};
    use super::*;

    pub fn parse_entry(buf: &mut BinaryReader, tag: u8) -> Result<Entry, ParseError> {
        match tag {
            Tag::UTF8 => UnresolvedUtf8Info::parse(buf).map(Entry::Utf8),
            Tag::INTEGER => IntegerInfo::parse(buf).map(Entry::Integer),
            Tag::FLOAT => FloatInfo::parse(buf).map(Entry::Float),
            Tag::LONG => LongInfo::parse(buf).map(Entry::Long),
            Tag::DOUBLE => DoubleInfo::parse(buf).map(Entry::Double),
            Tag::CLASS => UnresolvedClassInfo::parse(buf).map(Entry::Class),
            Tag::STRING => UnresolvedStringInfo::parse(buf).map(Entry::String),
            Tag::FIELDREF => FieldrefInfo::parse(buf).map(Entry::Fieldref),
            Tag::METHODREF => MethodrefInfo::parse(buf).map(Entry::Methodref),
            Tag::INTERFACE_METHODREF => InterfaceMethodrefInfo::parse(buf).map(Entry::InterfaceMethodref),
            Tag::NAME_AND_TYPE => UnresolvedNameAndTypeInfo::parse(buf).map(Entry::NameAndType),
            Tag::METHOD_HANDLE => MethodHandleInfo::parse(buf).map(Entry::MethodHandle),
            Tag::METHOD_TYPE => MethodTypeInfo::parse(buf).map(Entry::MethodType),
            Tag::DYNAMIC => DynamicInfo::parse(buf).map(Entry::Dynamic),
            Tag::INVOKE_DYNAMIC => InvokeDynamicInfo::parse(buf).map(Entry::InvokeDynamic),
            Tag::MODULE => ModuleInfo::parse(buf).map(Entry::Module),
            Tag::PACKAGE => PackageInfo::parse(buf).map(Entry::Package),
            _ => ParseError::new("invalid entry tag {tag}").into(),
        }
    }

    impl UnresolvedUtf8Info {
        fn parse(buf: &mut BinaryReader) -> Result<UnresolvedUtf8Info, ParseError> {
            buf.check_bytes(2, "utf8")?;

            // SAFETY: Guaranteed by check_bytes
            let len = unsafe { buf.unsafe_read_u16() } as usize;
            buf.check_bytes(len, "utf8")?;

            // TODO: We shouldn't wrap this. When we have proper error handling,
            //  propagate it.
            let mut bytes = Array::new(len)
                .map_err(|_| ParseError::new("cannot allocate array"))?;

            // SAFETY: read only writes to the slice, doesn't read from it, so
            // it being full of uninitialized memory is not a problem
            let slice = unsafe { bytes.as_slice_mut() };
            buf.read(slice);

            Ok(UnresolvedUtf8Info::new(bytes))
        }
    }

    macro_rules! parse_num32 {
    ($name: ident) => {
        impl $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParseError> {
                buf.check_bytes(4, "integer/float")?;

                // SAFETY: Guaranteed by check_bytes
                let bytes = unsafe { buf.unsafe_read_u32() };
                Ok($name::from_bytes(bytes))
            }
        }
    };
}
    parse_num32!(IntegerInfo);
    parse_num32!(FloatInfo);

    macro_rules! parse_num64 {
    ($name: ident) => {
        impl $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParseError> {
                // 4 for high bytes, 4 for low bytes
                buf.check_bytes(4 + 4, "integer/float")?;

                // SAFETY: Guaranteed by check_bytes
                let high_bytes = unsafe { buf.unsafe_read_u32() };
                let low_bytes = unsafe { buf.unsafe_read_u32() };

                Ok($name::from_bytes(low_bytes, high_bytes))
            }
        }
    };
}
    parse_num64!(LongInfo);
    parse_num64!(DoubleInfo);

    impl UnresolvedStringInfo {
        fn parse(buf: &mut BinaryReader) -> Result<UnresolvedStringInfo, ParseError> {
            buf.check_bytes(2, "string")?;

            // SAFETY: Guaranteed by check_bytes
            let string_index = unsafe { buf.unsafe_read_u16() };
            Ok(UnresolvedStringInfo::new(string_index))
        }
    }

    macro_rules! parse_ref {
    ($name: ident, $typ: expr) => {
        impl $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParseError> {
                // 2 for class index, 2 for name and type index
                buf.check_bytes(2 + 2, $typ)?;

                // SAFETY: Guaranteed by check_bytes
                let class_index = unsafe { buf.unsafe_read_u16() };
                let name_and_type_index = unsafe { buf.unsafe_read_u16() };

                Ok($name::new(class_index, name_and_type_index))
            }
        }
    };
}
    parse_ref!(FieldrefInfo, "fieldref");
    parse_ref!(MethodrefInfo, "methodref");
    parse_ref!(InterfaceMethodrefInfo, "interface methodref");

    impl UnresolvedNameAndTypeInfo {
        fn parse(buf: &mut BinaryReader) -> Result<UnresolvedNameAndTypeInfo, ParseError> {
            // 2 for name index, 2 for descriptor index
            buf.check_bytes(2 + 2, "name and type")?;

            // SAFETY: Guaranteed by check_bytes
            let name_index = unsafe { buf.unsafe_read_u16() };
            let descriptor_index = unsafe { buf.unsafe_read_u16() };

            Ok(UnresolvedNameAndTypeInfo::new(name_index, descriptor_index))
        }
    }

    impl MethodHandleInfo {
        fn parse(buf: &mut BinaryReader) -> Result<MethodHandleInfo, ParseError> {
            // 1 for reference kind, 2 for referenxe index
            buf.check_bytes(1 + 2, "method handle")?;

            // SAFETY: Guaranteed by check_bytes
            let ref_kind = unsafe { buf.unsafe_read_u8() };
            let reference_kind = methodhandle::Ref::from_u8(ref_kind)
                .ok_or_else(|| ParseError::new("method handle - reference kind invalid"))?;
            let reference_index = unsafe { buf.unsafe_read_u16() };

            Ok(MethodHandleInfo::new(reference_kind, reference_index))
        }
    }

    impl MethodTypeInfo {
        fn parse(buf: &mut BinaryReader) -> Result<MethodTypeInfo, ParseError> {
            buf.check_bytes(2, "method type")?;

            // SAFETY: Guaranteed by check_bytes
            let descriptor_index = unsafe { buf.unsafe_read_u16() };
            Ok(MethodTypeInfo::new(descriptor_index))
        }
    }

    macro_rules! parse_dynamic {
    ($name: ident, $typ: expr) => {
        impl $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParseError> {
                // 2 for bootstrap method attribute index, 2 for name and type index
                buf.check_bytes(2 + 2, "dynamic")?;

                // SAFETY: Guaranteed by check_bytes
                let bootstrap_method_attr_index = unsafe { buf.unsafe_read_u16() };
                let name_and_type_index = unsafe { buf.unsafe_read_u16() };

                Ok($name::new(bootstrap_method_attr_index, name_and_type_index))
            }
        }
    };
}
    parse_dynamic!(DynamicInfo, "dynamic");
    parse_dynamic!(InvokeDynamicInfo, "invokedynamic");

    macro_rules! parse_nameable {
    ($name: ident, $typ: expr) => {
        impl $name {
            fn parse(buf: &mut BinaryReader) -> Result<$name, ParseError> {
                buf.check_bytes(2, $typ)?;

                // SAFETY: Guaranteed by check_bytes
                let name_index = unsafe { buf.unsafe_read_u16() };
                Ok($name::new(name_index))
            }
        }
    };
}
    parse_nameable!(UnresolvedClassInfo, "class");
    parse_nameable!(ModuleInfo, "module");
    parse_nameable!(PackageInfo, "package");
}
