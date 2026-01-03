use crate::parser::classfile::constantpool::PoolIndex;

#[repr(u8)]
pub enum VerificationType {
    Top = 0,
    Integer = 1,
    Float = 2,
    Double = 3,
    Long = 4,
    Null = 5,
    UninitializedThis = 6,
    Object { pool_index: PoolIndex } = 7,
    Uninitialized { offset: u16 } = 8
}
