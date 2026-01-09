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
use crate::types::{Array, OutOfMemoryError};

pub struct Pool {
    tags: Array<u8>,
    constants: Array<Entry>,
}

pub type Index = u16;
pub const INDEX_INVALID: Index = 0;

macro_rules! pool_get_type {
    ($fn_name: ident, $tag: ident, $info: ty, $enum_part: ident) => {
        pub fn $fn_name(&self, idx: Index) -> Option<&$info> {
            let entry = self.get_entry(idx, Tag::$tag)?;
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
    fn new(size: usize) -> Result<Self, OutOfMemoryError> {
        let tags = Array::new(size)?;
        let constants = Array::new(size)?;
        Ok(Self { tags, constants })
    }

    pool_get_type!(get_utf8, UTF8, Utf8Info, Utf8);
    pool_get_type!(get_integer, INTEGER, IntegerInfo, Integer);
    pool_get_type!(get_float, FLOAT, FloatInfo, Float);
    pool_get_type!(get_long, LONG, LongInfo, Long);
    pool_get_type!(get_double, DOUBLE, DoubleInfo, Double);
    pool_get_type!(get_unresolved_class, CLASS, UnresolvedClassInfo, UnresolvedClass);
    pool_get_type!(get_unresolved_string, STRING, UnresolvedStringInfo, UnresolvedString);
    pool_get_type!(get_field_ref, FIELDREF, FieldrefInfo, Fieldref);
    pool_get_type!(get_method_ref, METHODREF, MethodrefInfo, Methodref);
    pool_get_type!(get_interface_method_ref, INTERFACE_METHODREF,
        InterfaceMethodrefInfo, InterfaceMethodref);
    pool_get_type!(get_name_and_type, NAME_AND_TYPE, NameAndTypeInfo, NameAndType);
    pool_get_type!(get_method_handle, METHOD_HANDLE, MethodHandleInfo, MethodHandle);
    pool_get_type!(get_method_type, METHOD_TYPE, MethodTypeInfo, MethodType);
    pool_get_type!(get_dynamic, DYNAMIC, DynamicInfo, Dynamic);
    pool_get_type!(get_invoke_dynamic, INVOKE_DYNAMIC, InvokeDynamicInfo, InvokeDynamic);
    pool_get_type!(get_module, MODULE, ModuleInfo, Module);
    pool_get_type!(get_package, PACKAGE, PackageInfo, Package);

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

        if tag == Tag::LONG || tag == Tag::DOUBLE {
            let next_idx = arr_idx + 1;
            self.put_invalid_raw(next_idx);
        }
    }

    fn put_raw(&mut self, idx: usize, tag: u8, entry: Entry) {
        self.tags.set(idx, tag).expect("big problems");
        self.constants.set(idx, entry).expect("big problems");
    }

    fn put_invalid_raw(&mut self, idx: usize) {
        self.tags.set(idx, Tag::INVALID).expect("big problems");
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

#[repr(u8)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum Tag {
    Utf8 = Tag::UTF8,
    Integer = Tag::INTEGER,
    Float = Tag::FLOAT,
    Long = Tag::LONG,
    Double = Tag::DOUBLE,
    Class = Tag::CLASS,
    String = Tag::STRING,
    Fieldref = Tag::FIELDREF,
    Methodref = Tag::METHODREF,
    InterfaceMethodref = Tag::INTERFACE_METHODREF,
    NameAndType = Tag::NAME_AND_TYPE,
    MethodHandle = Tag::METHOD_HANDLE,
    MethodType = Tag::METHOD_TYPE,
    Dynamic = Tag::DYNAMIC,
    InvokeDynamic = Tag::INVOKE_DYNAMIC,
    Module = Tag::MODULE,
    Package = Tag::PACKAGE,
}

impl Tag {
    const INVALID: u8 = 0;
    const UTF8: u8 = 1;
    const INTEGER: u8 = 3;
    const FLOAT: u8 = 4;
    const LONG: u8 = 5;
    const DOUBLE: u8 = 6;
    const CLASS: u8 = 7;
    const STRING: u8 = 8;
    const FIELDREF: u8 = 9;
    const METHODREF: u8 = 10;
    const INTERFACE_METHODREF: u8 = 11;
    const NAME_AND_TYPE: u8 = 12;
    const METHOD_HANDLE: u8 = 15;
    const METHOD_TYPE: u8 = 16;
    const DYNAMIC: u8 = 17;
    const INVOKE_DYNAMIC: u8 = 18;
    const MODULE: u8 = 19;
    const PACKAGE: u8 = 20;
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
