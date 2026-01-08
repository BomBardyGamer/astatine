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

use crate::types::methodhandle;
use crate::types::primitives;

pub enum Entry {
    Utf8(Utf8Info),
    Integer(IntegerInfo),
    Float(FloatInfo),
    Long(LongInfo),
    Double(DoubleInfo),
    Class(ClassInfo),
    String(StringInfo),
    Fieldref(FieldrefInfo),
    Methodref(MethodrefInfo),
    InterfaceMethodref(InterfaceMethodrefInfo),
    NameAndType(NameAndTypeInfo),
    MethodHandle(MethodHandleInfo),
    MethodType(MethodTypeInfo),
    Dynamic(DynamicInfo),
    InvokeDynamic(InvokeDynamicInfo),
    Module(ModuleInfo),
    Package(PackageInfo),
    // Used for the 2nd part of Long and Double which take up 2 spaces
    // This is a bit of a hack to avoid some other solution of having the actual entry resolvable
    // from both index and index + 1
    Unusable
}

pub trait Info {
    fn tag() -> super::EntryTag;
}

macro_rules! impl_info {
    ($name: ident, $tag: ident) => {
        impl Info for $name {
            fn tag() -> super::EntryTag {
                super::EntryTag::$tag
            }
        }
    };
}

pub trait InfoNameable {
    fn name_index(&self) -> super::Index;
}

macro_rules! impl_nameable {
    ($name: ident) => {
        impl InfoNameable for $name {
            fn name_index(&self) -> super::Index {
                self.name_index
            }
        }
    };
}

macro_rules! simple_nameable {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            pub(super) name_index: super::Index,
        }
        impl_info!($name, $tag);
        impl_nameable!($name);
    };
}

simple_nameable!(ClassInfo, Class);

pub trait RefInfo {
    fn class_index(&self) -> super::Index;
    fn name_and_type_index(&self) -> super::Index;
}

macro_rules! ref_entry {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            pub(super) class_index: super::Index,
            pub(super) name_and_type_index: super::Index,
        }
        impl_info!($name, $tag);

        impl RefInfo for $name {
            fn class_index(&self) -> super::Index {
                self.class_index
            }

            fn name_and_type_index(&self) -> super::Index {
                self.name_and_type_index
            }
        }
    };
}

ref_entry!(FieldrefInfo, Fieldref);
ref_entry!(MethodrefInfo, Methodref);
ref_entry!(InterfaceMethodrefInfo, InterfaceMethodref);

pub struct StringInfo {
    pub(super) string_index: super::Index,
}
impl_info!(StringInfo, String);

pub trait Number32Info {
    fn int(&self) -> primitives::Int;
    fn float(&self) -> primitives::Float;
}

macro_rules! number32_info {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            pub(super) bytes: u32
        }
        impl_info!($name, $tag);

        impl Number32Info for $name {
            fn int(&self) -> primitives::Int {
                self.bytes as primitives::Int
            }

            fn float(&self) -> primitives::Float {
                f32::from_bits(self.bytes)
            }
        }
    };
}

number32_info!(IntegerInfo, Integer);
number32_info!(FloatInfo, Float);

pub trait Number64Info {
    fn long(&self) -> primitives::Long;
    fn double(&self) -> primitives::Double;
}

macro_rules! number64_info {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            pub(super) bytes: u64
        }
        impl_info!($name, $tag);

        impl Number64Info for $name {
            fn long(&self) -> primitives::Long {
                self.bytes as primitives::Long
            }

            fn double(&self) -> primitives::Double {
                f64::from_bits(self.bytes)
            }
        }
    };
}

number64_info!(LongInfo, Long);
number64_info!(DoubleInfo, Double);

pub struct NameAndTypeInfo {
    pub(super) name_index: super::Index,
    pub(super) descriptor_index: super::Index,
}
impl_info!(NameAndTypeInfo, NameAndType);
impl_nameable!(NameAndTypeInfo);

impl NameAndTypeInfo {
    pub fn descriptor_index(&self) -> super::Index {
        self.descriptor_index
    }
}

// TODO: Figure out about how to do string stuff with this
pub struct Utf8Info {
    pub(super) bytes: Vec<u8>
}
impl_info!(Utf8Info, Utf8);

pub struct MethodHandleInfo {
    pub(super) reference_kind: methodhandle::Ref,
    pub(super) reference_index: super::Index
}
impl_info!(MethodHandleInfo, MethodHandle);

pub struct MethodTypeInfo {
    pub(super) descriptor_index: super::Index,
}
impl_info!(MethodTypeInfo, MethodType);

macro_rules! dynamic {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            pub(super) bootstrap_method_attr_index: super::Index,
            pub(super) name_and_type_index: super::Index,
        }
        impl_info!($name, $tag);

        impl $name {
            pub fn bootstrap_method_attr_index(&self) -> super::Index {
                self.bootstrap_method_attr_index
            }

            pub fn name_and_type_index(&self) -> super::Index {
                self.name_and_type_index
            }
        }
    };
}

dynamic!(DynamicInfo, Dynamic);
dynamic!(InvokeDynamicInfo, InvokeDynamic);

simple_nameable!(ModuleInfo, Module);
simple_nameable!(PackageInfo, Package);
