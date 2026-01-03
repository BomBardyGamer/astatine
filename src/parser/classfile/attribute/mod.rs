mod names;
mod classfile;
mod field;
mod module;
mod record;
mod method;
mod code;
mod stackmap;
mod verificationtypes;

pub use names::{AttributeNames, NameableAttribute};

use std::sync::OnceLock;
use crate::parser::classfile::attribute::names::{impl_attr_name};
use crate::parser::classfile::constantpool::PoolIndex;

macro_rules! attr_names {
    ($($name: ident),+) => {
        fn names() -> &'static [&'static str] {
            static NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
            NAMES.get_or_init(|| {
                let mut v = Vec::new();
                v.push(AttributeNames::RUNTIME_VISIBLE_TYPE_ANNOTATIONS);
                v.push(AttributeNames::RUNTIME_INVISIBLE_TYPE_ANNOTATIONS);
                $(v.push(AttributeNames::$name);)+
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
    signature_index: PoolIndex,
}
impl_attr_name!(Signature, SIGNATURE);

impl Signature {
    pub fn signature_index(&self) -> PoolIndex {
        self.signature_index
    }
}

pub struct Synthetic {}
impl_attr_name!(Synthetic, SYNTHETIC);

pub struct Deprecated {}
impl_attr_name!(Deprecated, DEPRECATED);
