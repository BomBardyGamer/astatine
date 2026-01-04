#[repr(u8)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum Ref {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

impl Ref {
    const MIN_REF: u8 = 1;
    const MAX_REF: u8 = 9;

    pub const fn is_valid(raw: u8) -> bool {
        raw >= Self::MIN_REF && raw <= Self::MAX_REF
    }
}
