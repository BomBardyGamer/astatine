// Copyright (C) 2026 Callum Jay Seabrook Hefford (BomBardyGamer)
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, see <https://www.gnu.org/licenses/>.

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

pub use _attr_name::*;
pub use _parse::*;

use std::sync::OnceLock;
use self::names::{Names, Nameable};
use crate::loader::classfile::constantpool;

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
    Signature(Signature),
    RuntimeVisibleAnnotations(annotations::RuntimeVisible),
    RuntimeInvisibleAnnotations(annotations::RuntimeInvisible),
    Synthetic(Synthetic),
    Deprecated(Deprecated),
    SourceFile(classfile::SourceFile),
    InnerClasses(classfile::InnerClasses),
    EnclosingMethod(classfile::EnclosingMethod),
    BootstrapMethods(classfile::BootstrapMethods),
    Module(module::Module),
    ModulePackages(module::ModulePackages),
    ModuleMainClass(module::ModuleMainClass),
    NestHost(classfile::NestHost),
    NestMembers(classfile::NestMembers),
    Record(record::Record),
    PermittedSubclasses(classfile::PermittedSubclasses),
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
    Signature(Signature),
    RuntimeVisibleAnnotations(annotations::RuntimeVisible),
    RuntimeInvisibleAnnotations(annotations::RuntimeInvisible),
    Synthetic(Synthetic),
    Deprecated(Deprecated),
    ConstantValue(field::ConstantValue),
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
    Signature(Signature),
    RuntimeVisibleAnnotations(annotations::RuntimeVisible),
    RuntimeInvisibleAnnotations(annotations::RuntimeInvisible),
    Synthetic(Synthetic),
    Deprecated(Deprecated),
    Code(code::Code),
    Exceptions(method::Exceptions),
    RuntimeVisibleParameterAnnotations(annotations::ParameterRuntimeVisible),
    RuntimeInvisibleParameterAnnotations(annotations::ParameterRuntimeInvisible),
    AnnotationDefault(method::AnnotationDefault),
    MethodParameters(method::MethodParameters),
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
    Signature(Signature),
    RuntimeVisibleAnnotations(annotations::RuntimeVisible),
    RuntimeInvisibleAnnotations(annotations::RuntimeInvisible),
}

impl RecordAttribute {
    attr_names!(
        SIGNATURE,
        RUNTIME_VISIBLE_ANNOTATIONS,
        RUNTIME_INVISIBLE_ANNOTATIONS
    );
}

pub enum CodeAttribute {
    StackMapTable(code::StackMapTable),
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
    use crate::loader::classfile::attribute::names::impl_attr_name;

    impl_attr_name!(Signature, SIGNATURE);
    impl_attr_name!(Synthetic, SYNTHETIC);
    impl_attr_name!(Deprecated, DEPRECATED);
}

mod _parse {
    use super::*;
    use crate::loader::{BinaryReader, Parse, ParserError};

    impl Parse<Signature> for Signature {
        fn parse(buf: &mut BinaryReader) -> Result<Signature, ParserError> {
            buf.check_bytes(2, "signature")?;

            // SAFETY: Guaranteed by check_bytes
            let signature_index = unsafe { buf.unsafe_read_u16() };
            Ok(Signature { signature_index })
        }
    }

    impl Parse<Synthetic> for Synthetic {
        fn parse(_: &mut BinaryReader) -> Result<Synthetic, ParserError> {
            Ok(Synthetic {})
        }
    }

    impl Parse<Deprecated> for Deprecated {
        fn parse(_: &mut BinaryReader) -> Result<Deprecated, ParserError> {
            Ok(Deprecated {})
        }
    }
}
