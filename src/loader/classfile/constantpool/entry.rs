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
            pub(super) name_index: super::Index,
        }
        tag!($name, $tag);

        impl $name {
            pub const fn name_index(&self) -> super::Index {
                self.name_index
            }
        }
    };
}

name_info!(UnresolvedClassInfo, Class);

macro_rules! ref_entry {
    ($name: ident, $tag: ident) => {
        #[derive(Copy, Clone)]
        pub struct $name {
            pub(super) class_index: super::Index,
            pub(super) name_and_type_index: super::Index,
        }
        tag!($name, $tag);

        impl $name {
            pub const fn class_index(&self) -> super::Index {
                self.class_index
            }

            pub const fn name_and_type_index(&self) -> super::Index {
                self.name_and_type_index
            }
        }
    };
}

ref_entry!(FieldrefInfo, Fieldref);
ref_entry!(MethodrefInfo, Methodref);
ref_entry!(InterfaceMethodrefInfo, InterfaceMethodref);

pub struct UnresolvedStringInfo {
    pub(super) string_index: super::Index,
}
tag!(UnresolvedStringInfo, String);

impl UnresolvedStringInfo {
    pub const fn index(&self) -> super::Index {
        self.string_index
    }
}

pub struct IntegerInfo {
    value: i32,
}
tag!(IntegerInfo, Integer);

impl IntegerInfo {
    pub(super) const fn from_bytes(v: u32) -> IntegerInfo {
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
    pub(super) const fn from_bytes(v: u32) -> FloatInfo {
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
    pub(super) const fn from_bytes(low: u32, high: u32) -> LongInfo {
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
    pub(super) const fn from_bytes(low: u32, high: u32) -> DoubleInfo {
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

pub struct NameAndTypeInfo {
    pub(super) name_index: super::Index,
    pub(super) descriptor_index: super::Index,
}
tag!(NameAndTypeInfo, NameAndType);

impl NameAndTypeInfo {
    pub const fn name_index(&self) -> super::Index {
        self.name_index
    }

    pub const fn descriptor_index(&self) -> super::Index {
        self.descriptor_index
    }
}

// TODO: Figure out about how to do string stuff with this
pub struct Utf8Info {
    pub(super) bytes: Array<u8>
}
tag!(Utf8Info, Utf8);

pub struct MethodHandleInfo {
    pub(super) reference_kind: methodhandle::Ref,
    pub(super) reference_index: super::Index
}
tag!(MethodHandleInfo, MethodHandle);

impl MethodHandleInfo {
    pub const fn reference_kind(&self) -> methodhandle::Ref {
        self.reference_kind
    }

    pub const fn reference_index(&self) -> super::Index {
        self.reference_index
    }
}

pub struct MethodTypeInfo {
    pub(super) descriptor_index: super::Index,
}
tag!(MethodTypeInfo, MethodType);

impl MethodTypeInfo {
    pub const fn descriptor_index(&self) -> super::Index {
        self.descriptor_index
    }
}

macro_rules! dynamic {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            pub(super) bootstrap_method_attr_index: super::Index,
            pub(super) name_and_type_index: super::Index,
        }
        tag!($name, $tag);

        impl $name {
            pub const fn bootstrap_method_attr_index(&self) -> super::Index {
                self.bootstrap_method_attr_index
            }

            pub const fn name_and_type_index(&self) -> super::Index {
                self.name_and_type_index
            }
        }
    };
}

dynamic!(DynamicInfo, Dynamic);
dynamic!(InvokeDynamicInfo, InvokeDynamic);

name_info!(ModuleInfo, Module);
name_info!(PackageInfo, Package);
