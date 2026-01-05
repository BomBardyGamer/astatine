use crate::parser::classfile::constantpool;

pub use _attr_name::*;
pub use _parse::*;

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
    index: constantpool::Index,
    outer_index: constantpool::Index,
    name_index: constantpool::Index,
    access_flags: u16,
}

impl InnerClass {
    pub fn inner_class_index(&self) -> constantpool::Index {
        self.index
    }

    pub fn outer_class_index(&self) -> constantpool::Index {
        self.outer_index
    }

    pub fn inner_name_index(&self) -> constantpool::Index {
        self.name_index
    }

    pub fn inner_class_access_flags(&self) -> u16 {
        self.access_flags
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

mod _parse {
    use crate::{buf_read_named_type_vec, buf_read_u16_vec};
    use super::*;
    use crate::parser::{Parse, ParserError, BinaryReader};

    impl Parse<SourceFile> for SourceFile {
        fn parse(buf: &mut BinaryReader) -> Result<SourceFile, ParserError> {
            buf.check_bytes(2, "source file")?;

            // SAFETY: Guaranteed by check_bytes
            let source_file_index = unsafe { buf.unsafe_read_u16() };
            Ok(SourceFile { source_file_index })
        }
    }

    impl Parse<InnerClasses> for InnerClasses {
        fn parse(buf: &mut BinaryReader) -> Result<InnerClasses, ParserError> {
            buf_read_named_type_vec!(InnerClass, inner_classes, buf,
                "inner classes", "inner classes - idx {}");
            Ok(InnerClasses { inner_classes })
        }
    }

    impl Parse<InnerClass> for InnerClass {
        fn parse(buf: &mut BinaryReader) -> Result<InnerClass, ParserError> {
            // 2 index, 2 outer index, 2 name index, 2 access flags
            buf.check_bytes(2 + 2 + 2 + 2, "inner class")?;

            // SAFETY: Guaranteed by check_bytes
            let index = unsafe { buf.unsafe_read_u16() };
            let outer_index = unsafe { buf.unsafe_read_u16() };
            let name_index = unsafe { buf.unsafe_read_u16() };
            let access_flags = unsafe { buf.unsafe_read_u16() };

            Ok(InnerClass { index, outer_index, name_index, access_flags })
        }
    }

    impl Parse<EnclosingMethod> for EnclosingMethod {
        fn parse(buf: &mut BinaryReader) -> Result<EnclosingMethod, ParserError> {
            // 2 class index, 2 method index
            buf.check_bytes(2 + 2, "enclosing method")?;

            let class_index = unsafe { buf.unsafe_read_u16() };
            let method_index = unsafe { buf.unsafe_read_u16() };
            Ok(EnclosingMethod { class_index, method_index })
        }
    }

    impl Parse<BootstrapMethods> for BootstrapMethods {
        fn parse(buf: &mut BinaryReader) -> Result<BootstrapMethods, ParserError> {
            buf_read_named_type_vec!(BootstrapMethod, methods, buf,
                "bootstrap methods", "bootstrap methods - idx {}");
            Ok(BootstrapMethods { methods })
        }
    }

    impl Parse<BootstrapMethod> for BootstrapMethod {
        fn parse(buf: &mut BinaryReader) -> Result<BootstrapMethod, ParserError> {
            buf.check_bytes(2, "method ref")?;

            // SAFETY: Guaranteed by check_bytes
            let method_ref = unsafe { buf.unsafe_read_u16() };
            buf_read_u16_vec!(bootstrap_arguments, buf, "bootstrap arguments");

            Ok(BootstrapMethod { method_ref, bootstrap_arguments })
        }
    }

    impl Parse<NestHost> for NestHost {
        fn parse(buf: &mut BinaryReader) -> Result<NestHost, ParserError> {
            buf.check_bytes(2, "nest host")?;

            // SAFETY: Guaranteed by check_bytes
            let host_class_index = unsafe { buf.unsafe_read_u16() };
            Ok(NestHost { host_class_index })
        }
    }

    impl Parse<NestMembers> for NestMembers {
        fn parse(buf: &mut BinaryReader) -> Result<NestMembers, ParserError> {
            buf_read_u16_vec!(classes, buf, "nest members");
            Ok(NestMembers { classes })
        }
    }

    impl Parse<PermittedSubclasses> for PermittedSubclasses {
        fn parse(buf: &mut BinaryReader) -> Result<PermittedSubclasses, ParserError> {
            buf_read_u16_vec!(classes, buf, "permitted subclasses");
            Ok(PermittedSubclasses { classes })
        }
    }
}
