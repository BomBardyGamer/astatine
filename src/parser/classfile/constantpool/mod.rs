mod entry;
mod pool;

// Export everything in submodules in this module so it appears as all one module
pub use entry::{
    ClassInfo, DoubleInfo, DynamicInfo, Entry, Info, FieldrefInfo, FloatInfo, InfoNameable,
    IntegerInfo, InterfaceMethodrefInfo, InvokeDynamicInfo, LongInfo, MethodHandleInfo,
    MethodTypeInfo, MethodrefInfo, ModuleInfo, NameAndTypeInfo, PackageInfo, StringInfo, Utf8Info,
};
pub use pool::{INDEX_INVALID, Pool, Index};

#[repr(u8)]
pub enum EntryTag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}
