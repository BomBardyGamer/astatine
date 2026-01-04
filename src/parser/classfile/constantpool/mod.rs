mod entry;
mod pool;
mod parser;

// Export everything in submodules in this module so it appears as all one module
pub use entry::{
    ClassInfo, DoubleInfo, DynamicInfo, Entry, Info, FieldrefInfo, FloatInfo, InfoNameable,
    IntegerInfo, InterfaceMethodrefInfo, InvokeDynamicInfo, LongInfo, MethodHandleInfo,
    MethodTypeInfo, MethodrefInfo, ModuleInfo, NameAndTypeInfo, PackageInfo, StringInfo, Utf8Info,
};
pub use pool::{INDEX_INVALID, Pool, Index};

pub(self) const TAG_UTF8: u8 = 1;
pub(self) const TAG_INTEGER: u8 = 3;
pub(self) const TAG_FLOAT: u8 = 4;
pub(self) const TAG_LONG: u8 = 5;
pub(self) const TAG_DOUBLE: u8 = 6;
pub(self) const TAG_CLASS: u8 = 7;
pub(self) const TAG_STRING: u8 = 8;
pub(self) const TAG_FIELDREF: u8 = 9;
pub(self) const TAG_METHODREF: u8 = 10;
pub(self) const TAG_INTERFACE_METHODREF: u8 = 11;
pub(self) const TAG_NAME_AND_TYPE: u8 = 12;
pub(self) const TAG_METHOD_HANDLE: u8 = 15;
pub(self) const TAG_METHOD_TYPE: u8 = 16;
pub(self) const TAG_DYNAMIC: u8 = 17;
pub(self) const TAG_INVOKE_DYNAMIC: u8 = 18;
pub(self) const TAG_MODULE: u8 = 19;
pub(self) const TAG_PACKAGE: u8 = 20;

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
