mod names;
mod classfile;
mod field;
mod module;
mod record;
mod method;
mod code;
mod stackmap;
mod annotations;
mod type_annotations;

use std::sync::OnceLock;
use self::names::{Names, Nameable};
use crate::parser::classfile::constantpool;

macro_rules! attr_names {
    ($($name: ident),+) => {
        fn names() -> &'static [&'static str] {
            static NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
            NAMES.get_or_init(|| {
                let mut v = Vec::new();
                // Common amongst all uses of attributes
                v.push(Names::RUNTIME_VISIBLE_TYPE_ANNOTATIONS);
                v.push(Names::RUNTIME_INVISIBLE_TYPE_ANNOTATIONS);
                $(v.push(Names::$name);)+
                v
            })
        }
    };
}

pub enum ClassFileAttribute {
}

impl ClassFileAttribute {
    attr_names!(
        SIGNATURE,
        RUNTIME_VISIBLE_ANNOTATIONS,
        RUNTIME_INVISIBLE_ANNOTATIONS,
        SYNTHETIC,
        DEPRECATED,
        SOURCE_FILE,
        INNER_CLASSES,
        ENCLOSING_METHOD,
        // SOURCE_DEBUG_EXTENSION,
        BOOTSTRAP_METHODS,
        MODULE,
        MODULE_PACKAGES,
        MODULE_MAIN_CLASS,
        NEST_HOST,
        NEST_MEMBERS,
        RECORD,
        PERMITTED_SUBCLASSES
    );
}

pub enum FieldAttribute {
}

impl FieldAttribute {
    attr_names!(
        SIGNATURE,
        RUNTIME_VISIBLE_ANNOTATIONS,
        RUNTIME_INVISIBLE_ANNOTATIONS,
        SYNTHETIC,
        DEPRECATED,
        CONSTANT_VALUE
    );
}

pub enum MethodAttribute {
}

impl MethodAttribute {
    attr_names!(
        SIGNATURE,
        RUNTIME_VISIBLE_ANNOTATIONS,
        RUNTIME_INVISIBLE_ANNOTATIONS,
        SYNTHETIC,
        DEPRECATED,
        CODE,
        EXCEPTIONS,
        RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS,
        RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS,
        ANNOTATION_DEFAULT,
        METHOD_PARAMETERS
    );
}

pub enum RecordAttribute {
}

impl RecordAttribute {
    attr_names!(
        SIGNATURE,
        RUNTIME_VISIBLE_ANNOTATIONS,
        RUNTIME_INVISIBLE_ANNOTATIONS
    );
}

pub enum CodeAttribute {
}

impl CodeAttribute {
    attr_names!(
        STACK_MAP_TABLE//,
        //LINE_NUMBER_TABLE,
        //LOCAL_VARIABLE_TABLE,
        //LOCAL_VARIABLE_TYPE_TABLE
    );
}

pub struct Signature {
    signature_index: constantpool::Index,
}

impl Signature {
    pub fn signature_index(&self) -> constantpool::Index {
        self.signature_index
    }
}

pub struct Synthetic {}

pub struct Deprecated {}

mod _attr_name {
    use super::*;
    use crate::parser::classfile::attribute::names::impl_attr_name;

    impl_attr_name!(Signature, SIGNATURE);
    impl_attr_name!(Synthetic, SYNTHETIC);
    impl_attr_name!(Deprecated, DEPRECATED);
}
