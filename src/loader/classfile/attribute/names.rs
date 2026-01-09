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

macro_rules! name_const {
    ($const_name: ident, $str: expr) => {
        pub const $const_name: &'static str = $str;
    };
}

pub trait Nameable {
    fn name() -> &'static str;
}

// TODO: Could we make this a derive procedural macro??
macro_rules! impl_attr_name {
    ($name: ident, $const: ident) => {
        impl Nameable for $name {
            fn name() -> &'static str {
                Names::$const
            }
        }
    };
}
pub(crate) use impl_attr_name;

pub struct Names;

// ClassFile | field_info | method_info | record_component_info | Code
impl Names {
    name_const!(RUNTIME_VISIBLE_TYPE_ANNOTATIONS, "RuntimeVisibleTypeAnnotations");
    name_const!(RUNTIME_INVISIBLE_TYPE_ANNOTATIONS, "RuntimeInvisibleTypeAnnotations");
}

// ClassFile | field_info | method_info | record_component_info
impl Names {
    // MAJOR 49 and above
    name_const!(SIGNATURE, "Signature");
    name_const!(RUNTIME_VISIBLE_ANNOTATIONS, "RuntimeVisibleAnnotations");
    name_const!(RUNTIME_INVISIBLE_ANNOTATIONS, "RuntimeInvisibleAnnotations");
}

// ClassFile | field_info | method_info
impl Names {
    // MAJOR 45, MINOR 3 and above
    name_const!(SYNTHETIC, "Synthetic");
    name_const!(DEPRECATED, "Deprecated");
}

// ClassFile only
impl Names {
    // MAJOR 45, MINOR 3 and above
    name_const!(SOURCE_FILE, "SourceFile");
    name_const!(INNER_CLASSES, "InnerClasses");

    // MAJOR 49 and above
    name_const!(ENCLOSING_METHOD, "EnclosingMethod");
    // Debugging attribute we don't use in the VM
    //name_const!(SOURCE_DEBUG_EXTENSION, "SourceDebugExtension");

    // MAJOR 51 and above
    name_const!(BOOTSTRAP_METHODS, "BootstrapMethods");

    // MAJOR 53 and above
    name_const!(MODULE, "Module");
    name_const!(MODULE_PACKAGES, "ModulePackages");
    name_const!(MODULE_MAIN_CLASS, "ModuleMainClass");

    // MAJOR 55 and above
    name_const!(NEST_HOST, "NestHost");
    name_const!(NEST_MEMBERS, "NestMembers");

    // MAJOR 60 and above
    name_const!(RECORD, "Record");

    // MAJOR 61 and above
    name_const!(PERMITTED_SUBCLASSES, "PermittedSubclasses");
}

// field_info only
impl Names {
    // MAJOR 45, MINOR 3 and above
    name_const!(CONSTANT_VALUE, "ConstantValue");
}

// method_info only
impl Names {
    // MAJOR 45, MINOR 3 and above
    name_const!(CODE, "Code");
    name_const!(EXCEPTIONS, "Exceptions");

    // MAJOR 49 and above
    name_const!(RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS, "RuntimeVisibleParameterAnnotations");
    name_const!(RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS, "RuntimeInvisibleParameterAnnotations");
    name_const!(ANNOTATION_DEFAULT, "AnnotationDefault");

    // MAJOR 52 and above
    name_const!(METHOD_PARAMETERS, "MethodParameters");
}

// Code attribute only
impl Names {
    name_const!(STACK_MAP_TABLE, "StackMapTable");

    // Debugging attributes we don't use in the VM
    // name_const!(LINE_NUMBER_TABLE, "LineNumberTable");
    // name_const!(LOCAL_VARIABLE_TABLE, "LocalVariableTable");
    // name_const!(LOCAL_VARIABLE_TYPE_TABLE, "LocalVariableTypeTable");
}
