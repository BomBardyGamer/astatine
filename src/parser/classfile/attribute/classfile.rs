use crate::parser::classfile::attribute::names::AttributeNames;
use crate::parser::classfile::constantpool::pool::PoolIndex;

pub trait NameableAttribute {
    fn name() -> &'static str;
}

macro_rules! impl_nameable {
    ($name: ident, $const: ident) => {
        impl NameableAttribute for $name {
            fn name() -> &'static str {
                AttributeNames::$const
            }
        }
    };
}

pub struct SourceFile {
    source_file_index: PoolIndex,
}
impl_nameable!(SourceFile, SOURCE_FILE);

pub struct InnerClasses {
    inner_classes: Vec<InnerClass>,
}
impl_nameable!(InnerClasses, INNER_CLASSES);

pub struct InnerClass {
    inner_class_index: PoolIndex,
    outer_class_index: PoolIndex,
    inner_name_index: PoolIndex,
    inner_class_access_flags: u16,
}

pub struct EnclosingMethod {
    class_index: PoolIndex,
    method_index: PoolIndex,
}
impl_nameable!(EnclosingMethod, ENCLOSING_METHOD);

pub struct BootstrapMethods {
    methods: Vec<BootstrapMethod>,
}
impl_nameable!(BootstrapMethods, BOOTSTRAP_METHODS);

pub struct BootstrapMethod {
    method_ref: PoolIndex,
    bootstrap_arguments: Vec<PoolIndex>,
}

pub struct NestHost {
    host_class_index: PoolIndex,
}
impl_nameable!(NestHost, NEST_HOST);

pub struct NestMembers {
    classes: Vec<PoolIndex>,
}
impl_nameable!(NestMembers, NEST_MEMBERS);

pub struct PermittedSubclasses {
    classes: Vec<PoolIndex>,
}
impl_nameable!(PermittedSubclasses, PERMITTED_SUBCLASSES);
