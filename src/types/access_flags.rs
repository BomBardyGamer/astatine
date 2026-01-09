#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct AccessFlags(u16);

impl AccessFlags {
    // Class, Field, Method
    pub const PUBLIC: u16 = 0x0001;
    pub const FINAL: u16 = 0x0010;
    pub const SYNTHETIC: u16 = 0x1000;

    // Class, Field
    pub const ENUM: u16 = 0x4000;

    // Field, Method
    pub const PRIVATE: u16 = 0x0002;
    pub const PROTECTED: u16 = 0x0004;
    pub const STATIC: u16 = 0x0008;

    // Class, Method
    pub const ABSTRACT: u16 = 0x0400;

    // Class
    pub const SUPER: u16 = 0x0020;
    pub const INTERFACE: u16 = 0x0200;
    pub const MODULE: u16 = 0x8000;
    pub const ANNOTATION: u16 = 0x2000;

    // Field
    pub const VOLATILE: u16 = 0x0040;
    pub const TRANSIENT: u16 = 0x0080;

    // Method
    pub const SYNCHRONIZED: u16 = 0x0020;
    pub const BRIDGE: u16 = 0x0040;
    pub const VARARGS: u16 = 0x0080;
    pub const NATIVE: u16 = 0x0100;
    pub const STRICT: u16 = 0x0800;

    // Method Parameter, Module
    pub const MANDATED: u16 = 0x8000;

    // Module & Requires, Opens, Exports
    pub const OPEN: u16 = 0x0020;
    // Module - Requires only
    pub const STATIC_PHASE: u16 = 0x0040;
}

macro_rules! is_flag {
    ($name: ident, $cons: ident) => {
        pub fn $name(&self) -> bool {
            self.0 & Self::$cons != 0
        }
    };
}

impl AccessFlags {
    pub const fn new(v: u16) -> Self {
        Self(v)
    }

    #[inline]
    pub const fn flags(&self) -> u16 {
        self.0
    }

    is_flag!(is_public, PUBLIC);
    is_flag!(is_final, FINAL);
    is_flag!(is_synthetic, SYNTHETIC);
    is_flag!(is_enum, ENUM);
    is_flag!(is_private, PRIVATE);
    is_flag!(is_protected, PROTECTED);
    is_flag!(is_static, STATIC);
    is_flag!(is_abstract, ABSTRACT);
    is_flag!(is_super, SUPER);
    is_flag!(is_interface, INTERFACE);
    is_flag!(is_module, MODULE);
    is_flag!(is_annotation, ANNOTATION);
    is_flag!(is_volatile, VOLATILE);
    is_flag!(is_transient, TRANSIENT);
    is_flag!(is_synchronized, SYNCHRONIZED);
    is_flag!(is_bridge, BRIDGE);
    is_flag!(is_varargs, VARARGS);
    is_flag!(is_native, NATIVE);
    is_flag!(is_strict, STRICT);
    is_flag!(is_mandated, MANDATED);
    is_flag!(is_open, OPEN);
    is_flag!(is_static_phase, STATIC_PHASE);
}
