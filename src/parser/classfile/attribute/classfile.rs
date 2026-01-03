use crate::parser::classfile::attribute::names::{
    impl_attr_name, AttributeNames, NameableAttribute,
};
use crate::parser::classfile::constantpool::PoolIndex;

pub struct SourceFile {
    source_file_index: PoolIndex,
}
impl_attr_name!(SourceFile, SOURCE_FILE);

impl SourceFile {
    pub fn source_file_index(&self) -> PoolIndex {
        self.source_file_index
    }
}

pub struct InnerClasses {
    inner_classes: Vec<InnerClass>,
}
impl_attr_name!(InnerClasses, INNER_CLASSES);

impl InnerClasses {
    pub fn classes(&self) -> &[InnerClass] {
        &self.inner_classes
    }
}

pub struct InnerClass {
    inner_class_index: PoolIndex,
    outer_class_index: PoolIndex,
    inner_name_index: PoolIndex,
    inner_class_access_flags: u16,
}

impl InnerClass {
    pub fn inner_class_index(&self) -> PoolIndex {
        self.inner_class_index
    }

    pub fn outer_class_index(&self) -> PoolIndex {
        self.outer_class_index
    }

    pub fn inner_name_index(&self) -> PoolIndex {
        self.inner_name_index
    }

    pub fn inner_class_access_flags(&self) -> u16 {
        self.inner_class_access_flags
    }
}

pub struct EnclosingMethod {
    class_index: PoolIndex,
    method_index: PoolIndex,
}
impl_attr_name!(EnclosingMethod, ENCLOSING_METHOD);

impl EnclosingMethod {
    pub fn class_index(&self) -> PoolIndex {
        self.class_index
    }

    pub fn method_index(&self) -> PoolIndex {
        self.method_index
    }
}

pub struct BootstrapMethods {
    methods: Vec<BootstrapMethod>,
}
impl_attr_name!(BootstrapMethods, BOOTSTRAP_METHODS);

impl BootstrapMethods {
    pub fn methods(&self) -> &[BootstrapMethod] {
        &self.methods
    }
}

pub struct BootstrapMethod {
    method_ref: PoolIndex,
    bootstrap_arguments: Vec<PoolIndex>,
}

impl BootstrapMethod {
    pub fn method_ref(&self) -> PoolIndex {
        self.method_ref
    }

    pub fn bootstrap_arguments(&self) -> &[PoolIndex] {
        &self.bootstrap_arguments
    }
}

pub struct NestHost {
    host_class_index: PoolIndex,
}
impl_attr_name!(NestHost, NEST_HOST);

impl NestHost {
    pub fn host_class_index(&self) -> PoolIndex {
        self.host_class_index
    }
}

pub struct NestMembers {
    classes: Vec<PoolIndex>,
}
impl_attr_name!(NestMembers, NEST_MEMBERS);

impl NestMembers {
    pub fn classes(&self) -> &[PoolIndex] {
        &self.classes
    }
}

pub struct PermittedSubclasses {
    classes: Vec<PoolIndex>,
}
impl_attr_name!(PermittedSubclasses, PERMITTED_SUBCLASSES);

impl PermittedSubclasses {
    pub fn classes(&self) -> &[PoolIndex] {
        &self.classes
    }
}
