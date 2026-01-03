use crate::parser::classfile::attribute::names::{
    impl_attr_name, AttributeNames, NameableAttribute,
};
use crate::parser::classfile::constantpool::PoolIndex;

pub struct Module {
    name_index: PoolIndex,
    flags: u16,
    version_index: PoolIndex,
    requires: Vec<ModuleRequires>,
    exports: Vec<ModuleExports>,
    opens: Vec<ModuleOpens>,
    uses: Vec<PoolIndex>,
    provides: Vec<ModuleProvides>,
}
impl_attr_name!(Module, MODULE);

impl Module {
    pub fn name_index(&self) -> PoolIndex {
        self.name_index
    }

    pub fn flags(&self) -> u16 {
        self.flags
    }

    pub fn version_index(&self) -> PoolIndex {
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

    pub fn uses(&self) -> &[PoolIndex] {
        &self.uses
    }

    pub fn provides(&self) -> &[ModuleProvides] {
        &self.provides
    }
}

pub struct ModulePackages {
    package_index: Vec<PoolIndex>,
}
impl_attr_name!(ModulePackages, MODULE_PACKAGES);

impl ModulePackages {
    pub fn package_index(&self) -> &[PoolIndex] {
        &self.package_index
    }
}

pub struct ModuleMainClass {
    main_class_index: PoolIndex,
}
impl_attr_name!(ModuleMainClass, MODULE_MAIN_CLASS);

impl ModuleMainClass {
    pub fn main_class_index(&self) -> PoolIndex {
        self.main_class_index
    }
}

#[repr(u16)]
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
            index: PoolIndex,
            flags: u16,
            $attr_name: $attr_ty,
        }

        impl $name {
            pub fn index(&self) -> PoolIndex {
                self.index
            }

            pub fn flags(&self) -> u16 {
                self.flags
            }
        }
    };
}

module_part!(ModuleRequires, version_index, PoolIndex);
module_part!(ModuleExports, to_index, Vec<PoolIndex>);
module_part!(ModuleOpens, to_index, Vec<PoolIndex>);

impl ModuleRequires {
    pub fn version_index(&self) -> PoolIndex {
        self.version_index
    }
}

impl ModuleExports {
    pub fn to_index(&self) -> &[PoolIndex] {
        &self.to_index
    }
}

impl ModuleOpens {
    pub fn to_index(&self) -> &[PoolIndex] {
        &self.to_index
    }
}

pub struct ModuleProvides {
    index: PoolIndex,
    with_index: Vec<PoolIndex>,
}
