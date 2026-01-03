use crate::parser::classfile::constantpool::pool::PoolIndex;
use crate::parser::classfile::constantpool::tag::EntryTag;
use crate::types::methodhandle::MethodHandleRef;
use crate::types::primitives::{Double, Float, Int, Long};

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
}

pub trait EntryInfo {
    fn tag() -> EntryTag;
}

macro_rules! impl_info {
    ($name: ident, $tag: ident) => {
        impl EntryInfo for $name {
            fn tag() -> EntryTag {
                EntryTag::$tag
            }
        }
    };
}

pub trait InfoNameable {
    fn name_index(&self) -> PoolIndex;
}

macro_rules! impl_nameable {
    ($name: ident) => {
        impl InfoNameable for $name {
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
        impl_info!($name, $tag);
        impl_nameable!($name);
    };
}

simple_nameable!(ClassInfo, Class);

pub trait RefInfo {
    fn class_index(&self) -> PoolIndex;
    fn name_and_type_index(&self) -> PoolIndex;
}

macro_rules! ref_entry {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            class_index: PoolIndex,
            name_and_type_index: PoolIndex,
        }
        impl_info!($name, $tag);

        impl RefInfo for $name {
            fn class_index(&self) -> PoolIndex {
                self.class_index
            }

            fn name_and_type_index(&self) -> PoolIndex {
                self.name_and_type_index
            }
        }
    };
}

ref_entry!(FieldrefInfo, Fieldref);
ref_entry!(MethodrefInfo, Methodref);
ref_entry!(InterfaceMethodrefInfo, InterfaceMethodref);

pub struct StringInfo {
    string_index: PoolIndex,
}
impl_info!(StringInfo, String);

pub trait Number32Info {
    fn int(&self) -> Int;
    fn float(&self) -> Float;
}

macro_rules! number32_info {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            bytes: u32
        }
        impl_info!($name, $tag);

        impl Number32Info for $name {
            fn int(&self) -> Int {
                self.bytes as Int
            }

            fn float(&self) -> Float {
                f32::from_bits(self.bytes)
            }
        }
    };
}

number32_info!(IntegerInfo, Integer);
number32_info!(FloatInfo, Float);

pub trait Number64Info {
    fn long(&self) -> Long;
    fn double(&self) -> Double;
}

macro_rules! number64_info {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            bytes: u64
        }
        impl_info!($name, $tag);

        impl Number64Info for $name {
            fn long(&self) -> Long {
                self.bytes as Long
            }

            fn double(&self) -> Double {
                f64::from_bits(self.bytes)
            }
        }
    };
}

number64_info!(LongInfo, Long);
number64_info!(DoubleInfo, Double);

pub struct NameAndTypeInfo {
    name_index: PoolIndex,
    descriptor_index: PoolIndex,
}
impl_info!(NameAndTypeInfo, NameAndType);
impl_nameable!(NameAndTypeInfo);

impl NameAndTypeInfo {
    pub fn descriptor_index(&self) -> PoolIndex {
        self.descriptor_index
    }
}

// TODO: Figure out about how to do string stuff with this
pub struct Utf8Info {
    bytes: Vec<u8>
}
impl_info!(Utf8Info, Utf8);

pub struct MethodHandleInfo {
    reference_kind: MethodHandleRef,
    reference_index: PoolIndex
}
impl_info!(MethodHandleInfo, MethodHandle);

pub struct MethodTypeInfo {
    descriptor_index: PoolIndex,
}
impl_info!(MethodTypeInfo, MethodType);

macro_rules! dynamic {
    ($name: ident, $tag: ident) => {
        pub struct $name {
            bootstrap_method_attr_index: PoolIndex,
            name_and_type_index: PoolIndex,
        }
        impl_info!($name, $tag);

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

dynamic!(DynamicInfo, Dynamic);
dynamic!(InvokeDynamicInfo, InvokeDynamic);

simple_nameable!(ModuleInfo, Module);
simple_nameable!(PackageInfo, Package);
