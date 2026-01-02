use crate::parser::classfile::constantpool::pool::PoolIndex;
use crate::parser::classfile::constantpool::tag::EntryTag;
use crate::types::methodhandle::MethodHandleRef;
use crate::types::primitives::{Double, Float, Int, Long};

pub enum Entry {
    Utf8(EntryUtf8),
    Integer(EntryInteger),
    Float(EntryFloat),
    Long(EntryLong),
    Double(EntryDouble),
    Class(EntryClass),
    String(EntryString),
    Fieldref(EntryFieldref),
    Methodref(EntryMethodref),
    InterfaceMethodref(EntryInterfaceMethodref),
    NameAndType(EntryNameAndType),
    MethodHandle(EntryMethodHandle),
    MethodType(EntryMethodType),
    Dynamic(EntryDynamic),
    InvokeDynamic(EntryInvokeDynamic),
    Module(EntryModule),
    Package(EntryPackage),
}

pub trait EntryTaggable {
    fn tag() -> EntryTag;
}

macro_rules! impl_taggable {
    ($name: ident, $tag: ident) => {
        impl EntryTaggable for $name {
            fn tag() -> EntryTag {
                EntryTag::$tag
            }
        }
    };
}

pub trait NameableEntry {
    fn name_index(&self) -> PoolIndex;
}

macro_rules! impl_nameable {
    ($name: ident) => {
        impl NameableEntry for $name {
            fn name_index(&self) -> PoolIndex {
                self.name_index
            }
        }
    };
}

macro_rules! simple_nameable {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            name_index: PoolIndex,
        }
        impl_taggable!($name, $tag);
        impl_nameable!($name);
    };
}

simple_nameable!(EntryClass, Class);

pub trait EntryRef {
    fn class_index(&self) -> PoolIndex;
    fn name_and_type_index(&self) -> PoolIndex;
}

macro_rules! ref_entry {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            class_index: PoolIndex,
            name_and_type_index: PoolIndex,
        }
        impl_taggable!($name, $tag);

        impl EntryRef for $name {
            fn class_index(&self) -> PoolIndex {
                self.class_index
            }

            fn name_and_type_index(&self) -> PoolIndex {
                self.name_and_type_index
            }
        }
    };
}

ref_entry!(EntryFieldref, Fieldref);
ref_entry!(EntryMethodref, Methodref);
ref_entry!(EntryInterfaceMethodref, InterfaceMethodref);

pub struct EntryString {
    string_index: PoolIndex,
}
impl_taggable!(EntryString, String);

pub trait EntryNumber32 {
    fn int(&self) -> Int;
    fn float(&self) -> Float;
}

macro_rules! number32_entry {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            bytes: u32
        }
        impl_taggable!($name, $tag);

        impl EntryNumber32 for $name {
            fn int(&self) -> Int {
                self.bytes as Int
            }

            fn float(&self) -> Float {
                f32::from_bits(self.bytes)
            }
        }
    };
}

number32_entry!(EntryInteger, Integer);
number32_entry!(EntryFloat, Float);

pub trait EntryNumber64 {
    fn long(&self) -> Long;
    fn double(&self) -> Double;
}

macro_rules! number64_entry {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            bytes: u64
        }
        impl_taggable!($name, $tag);

        impl EntryNumber64 for $name {
            fn long(&self) -> Long {
                self.bytes as Long
            }

            fn double(&self) -> Double {
                f64::from_bits(self.bytes)
            }
        }
    };
}

number64_entry!(EntryLong, Long);
number64_entry!(EntryDouble, Double);

pub struct EntryNameAndType {
    name_index: PoolIndex,
    descriptor_index: PoolIndex,
}
impl_taggable!(EntryNameAndType, NameAndType);
impl_nameable!(EntryNameAndType);

impl EntryNameAndType {
    pub fn descriptor_index(&self) -> PoolIndex {
        self.descriptor_index
    }
}

// TODO: Figure out about how to do string stuff with this
pub struct EntryUtf8 {
    bytes: Vec<u8>
}
impl_taggable!(EntryUtf8, Utf8);

pub struct EntryMethodHandle {
    reference_kind: MethodHandleRef,
    reference_index: PoolIndex
}
impl_taggable!(EntryMethodHandle, MethodHandle);

pub struct EntryMethodType {
    descriptor_index: PoolIndex,
}
impl_taggable!(EntryMethodType, MethodType);

macro_rules! dynamic {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            bootstrap_method_attr_index: PoolIndex,
            name_and_type_index: PoolIndex,
        }
        impl_taggable!($name, $tag);

        impl $name {
            pub fn bootstrap_method_attr_index(&self) -> PoolIndex {
                self.bootstrap_method_attr_index
            }

            pub fn name_and_type_index(&self) -> PoolIndex {
                self.name_and_type_index
            }
        }
    };
}

dynamic!(EntryDynamic, Dynamic);
dynamic!(EntryInvokeDynamic, InvokeDynamic);

simple_nameable!(EntryModule, Module);
simple_nameable!(EntryPackage, Package);
