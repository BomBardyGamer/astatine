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

use crate::loader::classfile::constantpool;
use crate::types::{AccessFlags, Array};

pub struct Module {
    name_index: constantpool::Index,
    flags: AccessFlags,
    version_index: constantpool::Index,
    requires: Array<ModuleRequires>,
    exports: Array<ModuleExports>,
    opens: Array<ModuleOpens>,
    uses: Array<constantpool::Index>,
    provides: Array<ModuleProvides>,
}

impl Module {
    pub fn name_index(&self) -> constantpool::Index {
        self.name_index
    }

    pub fn flags(&self) -> AccessFlags {
        self.flags
    }

    pub fn version_index(&self) -> constantpool::Index {
        self.version_index
    }

    pub fn requires(&self) -> &[ModuleRequires] {
        // SAFETY: We know this array is fully initialized
        unsafe { self.requires.as_slice() }
    }

    pub fn exports(&self) -> &[ModuleExports] {
        // SAFETY: We know this array is fully initialized
        unsafe { self.exports.as_slice() }
    }

    pub fn opens(&self) -> &[ModuleOpens] {
        // SAFETY: We know this array is fully initialized
        unsafe { self.opens.as_slice() }
    }

    pub fn uses(&self) -> &[constantpool::Index] {
        // SAFETY: We know this array is fully initialized
        unsafe { self.uses.as_slice() }
    }

    pub fn provides(&self) -> &[ModuleProvides] {
        // SAFETY: We know this array is fully initialized
        unsafe { self.provides.as_slice() }
    }
}

pub struct ModulePackages {
    package_index: Array<constantpool::Index>,
}

impl ModulePackages {
    pub fn package_index(&self) -> &[constantpool::Index] {
        // SAFETY: We know this array is fully initialized
        unsafe { self.package_index.as_slice() }
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
    use crate::loader::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(Module, MODULE);
    impl_attr_name!(ModulePackages, MODULE_PACKAGES);
    impl_attr_name!(ModuleMainClass, MODULE_MAIN_CLASS);
}

// Defines a part of a module with the index and flags variables and a third variable.
// requires, exports, and opens all have three variables, two of them (index and flags)
// being shared amongst them.
macro_rules! module_part {
    ($name: ident, $attr_name: ident, $attr_ty: ty) => {
        pub struct $name {
            index: constantpool::Index,
            flags: AccessFlags,
            $attr_name: $attr_ty,
        }

        impl $name {
            pub fn index(&self) -> constantpool::Index {
                self.index
            }

            pub fn flags(&self) -> AccessFlags {
                self.flags
            }
        }
    };
}

module_part!(ModuleRequires, version_index, constantpool::Index);
module_part!(ModuleExports, to_index, Array<constantpool::Index>);
module_part!(ModuleOpens, to_index, Array<constantpool::Index>);

impl ModuleRequires {
    pub fn version_index(&self) -> constantpool::Index {
        self.version_index
    }
}

impl ModuleExports {
    pub fn to_index(&self) -> &[constantpool::Index] {
        // SAFETY: We know this array is fully initialized
        unsafe { self.to_index.as_slice() }
    }
}

impl ModuleOpens {
    pub fn to_index(&self) -> &[constantpool::Index] {
        // SAFETY: We know this array is fully initialized
        unsafe { self.to_index.as_slice() }
    }
}

pub struct ModuleProvides {
    index: constantpool::Index,
    with_index: Array<constantpool::Index>,
}

mod _parse {
    use crate::{buf_read_named_type_arr, buf_read_u16_arr};
    use crate::loader::{BinaryReader, Parse, ParseError};
    use super::*;

    impl Parse<Module> for Module {
        fn parse(buf: &mut BinaryReader) -> Result<Module, ParseError> {
            // 2 name index, 2 flags, 2 version index
            buf.check_bytes(2 + 2 + 2, "module")?;

            // SAFETY: Guaranteed by check_bytes
            let name_index = unsafe { buf.unsafe_read_u16() };
            let flags = unsafe { buf.unsafe_read_u16() };
            let version_index = unsafe { buf.unsafe_read_u16() };

            buf_read_named_type_arr!(ModuleRequires, requires, buf,
                "module - requires", "module - requires - idx {}");
            buf_read_named_type_arr!(ModuleExports, exports, buf,
                "module - exports", "module - exports - idx {}");
            buf_read_named_type_arr!(ModuleOpens, opens, buf,
                "module - opens", "module - opens - idx {}");
            buf_read_u16_arr!(uses, buf, "module - uses");
            buf_read_named_type_arr!(ModuleProvides, provides, buf,
                "module - provides", "module - provides - idx {}");

            Ok(Module {
                name_index,
                flags: AccessFlags::new(flags),
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
        fn parse(buf: &mut BinaryReader) -> Result<ModuleRequires, ParseError> {
            // 2 index, 2 flags, 2 version index
            buf.check_bytes(2 + 2 + 2, "module requires")?;

            // SAFETY: Guaranteed by check_bytes
            let index = unsafe { buf.unsafe_read_u16() };
            let flags = unsafe { buf.unsafe_read_u16() };
            let version_index = unsafe { buf.unsafe_read_u16() };
            Ok(ModuleRequires { index, flags: AccessFlags::new(flags), version_index })
        }
    }

    macro_rules! parse_exports_opens {
        ($name: ident, $err_msg: expr) => {
            impl Parse<$name> for $name {
                fn parse(buf: &mut BinaryReader) -> Result<$name, ParseError> {
                    // 2 index, 2 flags, 2 to count
                    buf.check_bytes(2 + 2 + 2, $err_msg)?;

                    // SAFETY: Guaranteed by check_bytes
                    let index = unsafe { buf.unsafe_read_u16() };
                    let flags = unsafe { buf.unsafe_read_u16() };
                    buf_read_u16_arr!(to_index, buf, "module exports");

                    Ok($name { index, flags: AccessFlags::new(flags), to_index })
                }
            }
        };
    }
    parse_exports_opens!(ModuleExports, "module exports");
    parse_exports_opens!(ModuleOpens, "module opens");

    impl Parse<ModuleProvides> for ModuleProvides {
        fn parse(buf: &mut BinaryReader) -> Result<ModuleProvides, ParseError> {
            buf.check_bytes(2, "module provides")?;

            // SAFETY: Guaranteed by check_bytes
            let index = unsafe { buf.unsafe_read_u16() };
            buf_read_u16_arr!(with_index, buf, "module provides");

            Ok(ModuleProvides { index, with_index })
        }
    }
}
