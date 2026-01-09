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

mod entry;
mod parser;

// Export everything in submodules in this module so it appears as all one module
pub use entry::{
    UnresolvedClassInfo, DoubleInfo, DynamicInfo, FieldrefInfo, FloatInfo,
    IntegerInfo, InterfaceMethodrefInfo, InvokeDynamicInfo, LongInfo, MethodHandleInfo,
    MethodTypeInfo, MethodrefInfo, ModuleInfo, NameAndTypeInfo, PackageInfo, UnresolvedStringInfo, Utf8Info,
};
use crate::types::errors;
use crate::types::array::Array;

pub struct Pool {
    tags: Array<u8>,
    constants: Array<Entry>,
}

pub type Index = u16;
pub const INDEX_INVALID: Index = 0;

macro_rules! pool_get_type {
    ($fn_name: ident, $tag: expr, $info: ty, $enum_part: ident) => {
        pub fn $fn_name(&self, idx: Index) -> Option<&$info> {
            let entry = self.get_entry(idx, $tag)?;
            // SAFETY: Guaranteed by tag check in get_entry
            if let Entry::$enum_part(info) = entry {
                Some(info)
            } else {
                None
            }
        }
    };
}

impl Pool {
    fn new(size: usize) -> Result<Self, errors::OutOfMemoryError> {
        let tags = Array::new(size)?;
        let constants = Array::new(size)?;
        Ok(Self { tags, constants })
    }

    pool_get_type!(get_utf8, TAG_UTF8, Utf8Info, Utf8);
    pool_get_type!(get_integer, TAG_INTEGER, IntegerInfo, Integer);
    pool_get_type!(get_float, TAG_FLOAT, FloatInfo, Float);
    pool_get_type!(get_long, TAG_LONG, LongInfo, Long);
    pool_get_type!(get_double, TAG_DOUBLE, DoubleInfo, Double);
    pool_get_type!(get_unresolved_class, TAG_CLASS, UnresolvedClassInfo, UnresolvedClass);
    pool_get_type!(get_unresolved_string, TAG_STRING, UnresolvedStringInfo, UnresolvedString);
    pool_get_type!(get_field_ref, TAG_FIELDREF, FieldrefInfo, Fieldref);
    pool_get_type!(get_method_ref, TAG_METHODREF, MethodrefInfo, Methodref);
    pool_get_type!(get_interface_method_ref, TAG_INTERFACE_METHODREF,
        InterfaceMethodrefInfo, InterfaceMethodref);
    pool_get_type!(get_name_and_type, TAG_NAME_AND_TYPE, NameAndTypeInfo, NameAndType);
    pool_get_type!(get_method_handle, TAG_METHOD_HANDLE, MethodHandleInfo, MethodHandle);
    pool_get_type!(get_method_type, TAG_METHOD_TYPE, MethodTypeInfo, MethodType);
    pool_get_type!(get_dynamic, TAG_DYNAMIC, DynamicInfo, Dynamic);
    pool_get_type!(get_invoke_dynamic, TAG_INVOKE_DYNAMIC, InvokeDynamicInfo, InvokeDynamic);
    pool_get_type!(get_module, TAG_MODULE, ModuleInfo, Module);
    pool_get_type!(get_package, TAG_PACKAGE, PackageInfo, Package);

    fn get_entry(&self, idx: Index, required_tag: u8) -> Option<&Entry> {
        if idx == INDEX_INVALID {
            return None;
        }

        let index = (idx - 1) as usize;
        let tag = *self.tags.get(index)?;
        if tag != required_tag {
            return None;
        }

        self.constants.get(index)
    }

    pub fn size(&self) -> u16 {
        // CP is indexed from 1 so size is 1 more than array size
        (self.constants.len() + 1) as u16
    }

    pub fn is_valid_index(&self, index: Index) -> bool {
        index >= 1 && index < self.size()
    }

    fn put(&mut self, idx: Index, tag: u8, entry: Entry) {
        let arr_idx = self.cp_idx_to_arr_idx(idx);
        self.put_raw(arr_idx, tag, entry);

        if tag == TAG_LONG || tag == TAG_DOUBLE {
            let next_idx = arr_idx + 1;
            self.put_invalid_raw(next_idx);
        }
    }

    fn put_raw(&mut self, idx: usize, tag: u8, entry: Entry) {
        self.tags.set(idx, tag).expect("big problems");
        self.constants.set(idx, entry).expect("big problems");
    }

    fn put_invalid_raw(&mut self, idx: usize) {
        self.tags.set(idx, TAG_INVALID).expect("big problems");
    }

    #[inline]
    const fn cp_idx_to_arr_idx(&self, cp_idx: Index) -> usize {
        // Internal array starts from 0 but CP starts from 1
        (cp_idx - 1) as usize
    }

    #[inline]
    const fn arr_idx_to_cp_idx(&self, arr_idx: usize) -> Index {
        // Internal array starts from 0 but CP starts from 1
        (arr_idx + 1) as Index
    }
}

const TAG_INVALID: u8 = 0;
const TAG_UTF8: u8 = 1;
const TAG_INTEGER: u8 = 3;
const TAG_FLOAT: u8 = 4;
const TAG_LONG: u8 = 5;
const TAG_DOUBLE: u8 = 6;
const TAG_CLASS: u8 = 7;
const TAG_STRING: u8 = 8;
const TAG_FIELDREF: u8 = 9;
const TAG_METHODREF: u8 = 10;
const TAG_INTERFACE_METHODREF: u8 = 11;
const TAG_NAME_AND_TYPE: u8 = 12;
const TAG_METHOD_HANDLE: u8 = 15;
const TAG_METHOD_TYPE: u8 = 16;
const TAG_DYNAMIC: u8 = 17;
const TAG_INVOKE_DYNAMIC: u8 = 18;
const TAG_MODULE: u8 = 19;
const TAG_PACKAGE: u8 = 20;

#[repr(u8)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum EntryTag {
    Utf8 = TAG_UTF8,
    Integer = TAG_INTEGER,
    Float = TAG_FLOAT,
    Long = TAG_LONG,
    Double = TAG_DOUBLE,
    Class = TAG_CLASS,
    String = TAG_STRING,
    Fieldref = TAG_FIELDREF,
    Methodref = TAG_METHODREF,
    InterfaceMethodref = TAG_INTERFACE_METHODREF,
    NameAndType = TAG_NAME_AND_TYPE,
    MethodHandle = TAG_METHOD_HANDLE,
    MethodType = TAG_METHOD_TYPE,
    Dynamic = TAG_DYNAMIC,
    InvokeDynamic = TAG_INVOKE_DYNAMIC,
    Module = TAG_MODULE,
    Package = TAG_PACKAGE,
}

enum Entry {
    Utf8(Utf8Info),
    Integer(IntegerInfo),
    Float(FloatInfo),
    Long(LongInfo),
    Double(DoubleInfo),
    UnresolvedClass(UnresolvedClassInfo),
    UnresolvedString(UnresolvedStringInfo),
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
}
// union Entry {
//     utf8: Utf8Info,
//     integer: IntegerInfo,
//     float: FloatInfo,
//     long: LongInfo,
//     double: DoubleInfo,
//     class: UnresolvedClassInfo,
//     string: UnresolvedStringInfo,
//     field_ref: FieldrefInfo,
//     method_ref: MethodrefInfo,
//     interface_method_ref: InterfaceMethodrefInfo,
//     name_and_type: NameAndTypeInfo,
//     method_handle: MethodHandleInfo,
//     method_type: MethodTypeInfo,
//     dynamic: DynamicInfo,
//     invoke_dynamic: InvokeDynamicInfo,
//     module: ModuleInfo,
//     package: PackageInfo,
// }

// impl Entry {
//     const fn utf8(info: Utf8Info) -> Entry {
//         Entry { utf8: info }
//     }
//
//     const fn integer(info: IntegerInfo) -> Entry {
//         Entry { integer: info }
//     }
//
//     const fn float(info: FloatInfo) -> Entry {
//         Entry { float: info }
//     }
//
//     const fn long(info: LongInfo) -> Entry {
//         Entry { long: info }
//     }
//
//     const fn double(info: DoubleInfo) -> Entry {
//         Entry { double: info }
//     }
//
//     const fn class(info: UnresolvedClassInfo) -> Entry {
//         Entry { class: info }
//     }
//
//     const fn string(info: UnresolvedStringInfo) -> Entry {
//         Entry { string: info }
//     }
//
//     const fn field_ref(info: FieldrefInfo) -> Entry {
//         Entry { field_ref: info }
//     }
//
//     const fn method_ref(info: MethodrefInfo) -> Entry {
//         Entry { method_ref: info }
//     }
//
//     const fn interface_method_ref(info: InterfaceMethodrefInfo) -> Entry {
//         Entry { interface_method_ref: info }
//     }
//
//     const fn name_and_type(info: NameAndTypeInfo) -> Entry {
//         Entry { name_and_type: info }
//     }
//
//     const fn method_handle(info: MethodHandleInfo) -> Entry {
//         Entry { method_handle: info }
//     }
//
//     const fn method_type(info: MethodTypeInfo) -> Entry {
//         Entry { method_type: info }
//     }
//
//     const fn dynamic(info: DynamicInfo) -> Entry {
//         Entry { dynamic: info }
//     }
//
//     const fn invoke_dynamic(info: InvokeDynamicInfo) -> Entry {
//         Entry { invoke_dynamic: info }
//     }
//
//     const fn module(info: ModuleInfo) -> Entry {
//         Entry { module: info }
//     }
//
//     const fn package(info: PackageInfo) -> Entry {
//         Entry { package: info }
//     }
// }
