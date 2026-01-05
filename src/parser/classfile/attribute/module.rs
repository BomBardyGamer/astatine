use crate::parser::classfile::constantpool;

pub struct Module {
    name_index: constantpool::Index,
    flags: u16,
    version_index: constantpool::Index,
    requires: Vec<ModuleRequires>,
    exports: Vec<ModuleExports>,
    opens: Vec<ModuleOpens>,
    uses: Vec<constantpool::Index>,
    provides: Vec<ModuleProvides>,
}

impl Module {
    pub fn name_index(&self) -> constantpool::Index {
        self.name_index
    }

    pub fn flags(&self) -> u16 {
        self.flags
    }

    pub fn version_index(&self) -> constantpool::Index {
        self.version_index
    }

    pub fn requires(&self) -> &[ModuleRequires] {
        &self.requires
    }

    pub fn exports(&self) -> &[ModuleExports] {
        &self.exports
    }

    pub fn opens(&self) -> &[ModuleOpens] {
        &self.opens
    }

    pub fn uses(&self) -> &[constantpool::Index] {
        &self.uses
    }

    pub fn provides(&self) -> &[ModuleProvides] {
        &self.provides
    }
}

pub struct ModulePackages {
    package_index: Vec<constantpool::Index>,
}

impl ModulePackages {
    pub fn package_index(&self) -> &[constantpool::Index] {
        &self.package_index
    }
}

pub struct ModuleMainClass {
    main_class_index: constantpool::Index,
}

impl ModuleMainClass {
    pub fn main_class_index(&self) -> constantpool::Index {
        self.main_class_index
    }
}

mod _attr_name {
    use super::*;
    use crate::parser::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(Module, MODULE);
    impl_attr_name!(ModulePackages, MODULE_PACKAGES);
    impl_attr_name!(ModuleMainClass, MODULE_MAIN_CLASS);
}

#[repr(u16)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum ModuleFlag {
    // Applies to the module attribute, requires, exports, and opens declarations
    Synthetic = 0x1000,
    Mandated = 0x8000,

    // Applies to the module attribute and requires declarations
    Open = 0x0020,

    // Applies to requires declarations only
    StaticPhase = 0x0040,
}

// Defines a part of a module with the index and flags variables and a third variable.
// requires, exports, and opens all have three variables, two of them (index and flags)
// being shared amongst them.
macro_rules! module_part {
    ($name: ident, $attr_name: ident, $attr_ty: ty) => {
        pub struct $name {
            index: constantpool::Index,
            flags: u16,
            $attr_name: $attr_ty,
        }

        impl $name {
            pub fn index(&self) -> constantpool::Index {
                self.index
            }

            pub fn flags(&self) -> u16 {
                self.flags
            }
        }
    };
}

module_part!(ModuleRequires, version_index, constantpool::Index);
module_part!(ModuleExports, to_index, Vec<constantpool::Index>);
module_part!(ModuleOpens, to_index, Vec<constantpool::Index>);

impl ModuleRequires {
    pub fn version_index(&self) -> constantpool::Index {
        self.version_index
    }
}

impl ModuleExports {
    pub fn to_index(&self) -> &[constantpool::Index] {
        &self.to_index
    }
}

impl ModuleOpens {
    pub fn to_index(&self) -> &[constantpool::Index] {
        &self.to_index
    }
}

pub struct ModuleProvides {
    index: constantpool::Index,
    with_index: Vec<constantpool::Index>,
}

mod _parse {
    use crate::{buf_read_named_type_vec, buf_read_u16_vec};
    use crate::parser::{BinaryReader, Parse, ParserError};
    use super::*;

    impl Parse<Module> for Module {
        fn parse(buf: &mut BinaryReader) -> Result<Module, ParserError> {
            // 2 name index, 2 flags, 2 version index
            buf.check_bytes(2 + 2 + 2, "module")?;

            // SAFETY: Guaranteed by check_bytes
            let name_index = unsafe { buf.unsafe_read_u16() };
            let flags = unsafe { buf.unsafe_read_u16() };
            let version_index = unsafe { buf.unsafe_read_u16() };

            buf_read_named_type_vec!(ModuleRequires, requires, buf,
                "module - requires", "module - requires - idx {}");
            buf_read_named_type_vec!(ModuleExports, exports, buf,
                "module - exports", "module - exports - idx {}");
            buf_read_named_type_vec!(ModuleOpens, opens, buf,
                "module - opens", "module - opens - idx {}");
            buf_read_u16_vec!(uses, buf, "module - uses");
            buf_read_named_type_vec!(ModuleProvides, provides, buf,
                "module - provides", "module - provides - idx {}");

            Ok(Module {
                name_index,
                flags,
                version_index,
                requires,
                exports,
                opens,
                uses,
                provides
            })
        }
    }

    impl Parse<ModuleRequires> for ModuleRequires {
        fn parse(buf: &mut BinaryReader) -> Result<ModuleRequires, ParserError> {
            // 2 index, 2 flags, 2 version index
            buf.check_bytes(2 + 2 + 2, "module requires")?;

            // SAFETY: Guaranteed by check_bytes
            let index = unsafe { buf.unsafe_read_u16() };
            let flags = unsafe { buf.unsafe_read_u16() };
            let version_index = unsafe { buf.unsafe_read_u16() };
            Ok(ModuleRequires { index, flags, version_index })
        }
    }

    macro_rules! parse_exports_opens {
        ($name: ident, $err_msg: expr) => {
            impl Parse<$name> for $name {
                fn parse(buf: &mut BinaryReader) -> Result<$name, ParserError> {
                    // 2 index, 2 flags, 2 to count
                    buf.check_bytes(2 + 2 + 2, $err_msg)?;

                    // SAFETY: Guaranteed by check_bytes
                    let index = unsafe { buf.unsafe_read_u16() };
                    let flags = unsafe { buf.unsafe_read_u16() };
                    buf_read_u16_vec!(to_index, buf, "module exports");

                    Ok($name { index, flags, to_index })
                }
            }
        };
    }
    parse_exports_opens!(ModuleExports, "module exports");
    parse_exports_opens!(ModuleOpens, "module opens");

    impl Parse<ModuleProvides> for ModuleProvides {
        fn parse(buf: &mut BinaryReader) -> Result<ModuleProvides, ParserError> {
            buf.check_bytes(2, "module provides")?;

            // SAFETY: Guaranteed by check_bytes
            let index = unsafe { buf.unsafe_read_u16() };
            buf_read_u16_vec!(with_index, buf, "module provides");

            Ok(ModuleProvides { index, with_index })
        }
    }
}
