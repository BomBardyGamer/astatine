use crate::parser::classfile::constantpool;

pub struct SourceFile {
    source_file_index: constantpool::Index,
}

impl SourceFile {
    pub fn source_file_index(&self) -> constantpool::Index {
        self.source_file_index
    }
}

pub struct InnerClasses {
    inner_classes: Vec<InnerClass>,
}

impl InnerClasses {
    pub fn classes(&self) -> &[InnerClass] {
        &self.inner_classes
    }
}

pub struct InnerClass {
    inner_class_index: constantpool::Index,
    outer_class_index: constantpool::Index,
    inner_name_index: constantpool::Index,
    inner_class_access_flags: u16,
}

impl InnerClass {
    pub fn inner_class_index(&self) -> constantpool::Index {
        self.inner_class_index
    }

    pub fn outer_class_index(&self) -> constantpool::Index {
        self.outer_class_index
    }

    pub fn inner_name_index(&self) -> constantpool::Index {
        self.inner_name_index
    }

    pub fn inner_class_access_flags(&self) -> u16 {
        self.inner_class_access_flags
    }
}

pub struct EnclosingMethod {
    class_index: constantpool::Index,
    method_index: constantpool::Index,
}

impl EnclosingMethod {
    pub fn class_index(&self) -> constantpool::Index {
        self.class_index
    }

    pub fn method_index(&self) -> constantpool::Index {
        self.method_index
    }
}

pub struct BootstrapMethods {
    methods: Vec<BootstrapMethod>,
}

impl BootstrapMethods {
    pub fn methods(&self) -> &[BootstrapMethod] {
        &self.methods
    }
}

pub struct BootstrapMethod {
    method_ref: constantpool::Index,
    bootstrap_arguments: Vec<constantpool::Index>,
}

impl BootstrapMethod {
    pub fn method_ref(&self) -> constantpool::Index {
        self.method_ref
    }

    pub fn bootstrap_arguments(&self) -> &[constantpool::Index] {
        &self.bootstrap_arguments
    }
}

pub struct NestHost {
    host_class_index: constantpool::Index,
}

impl NestHost {
    pub fn host_class_index(&self) -> constantpool::Index {
        self.host_class_index
    }
}

pub struct NestMembers {
    classes: Vec<constantpool::Index>,
}

impl NestMembers {
    pub fn classes(&self) -> &[constantpool::Index] {
        &self.classes
    }
}

pub struct PermittedSubclasses {
    classes: Vec<constantpool::Index>,
}

impl PermittedSubclasses {
    pub fn classes(&self) -> &[constantpool::Index] {
        &self.classes
    }
}

mod _attr_name {
    use super::*;
    use crate::parser::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(SourceFile, SOURCE_FILE);
    impl_attr_name!(InnerClasses, INNER_CLASSES);
    impl_attr_name!(EnclosingMethod, ENCLOSING_METHOD);
    impl_attr_name!(BootstrapMethods, BOOTSTRAP_METHODS);
    impl_attr_name!(NestHost, NEST_HOST);
    impl_attr_name!(NestMembers, NEST_MEMBERS);
    impl_attr_name!(PermittedSubclasses, PERMITTED_SUBCLASSES);
}
