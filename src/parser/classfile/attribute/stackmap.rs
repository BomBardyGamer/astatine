use crate::parser::classfile::constantpool;

pub enum Frame {
    Same { frame_type: u8 },
    SameLocalsOneStackItem {
        frame_type: u8,
        stack: VerificationType
    },
    SameLocalsOneStackItemExtended {
        offset_delta: u16,
        stack: VerificationType
    },
    Chop {
        frame_type: u8,
        offset_delta: u16,
    },
    SameExtended { offset_delta: u16, },
    Append {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationType>,
    },
    Full {
        offset_delta: u16,
        locals: Vec<VerificationType>,
        stack: Vec<VerificationType>,
    }
}

impl Frame {
    pub fn frame_type(&self) -> u8 {
        match self {
            Frame::Same { frame_type } => *frame_type,
            Frame::SameLocalsOneStackItem { frame_type, stack: _ } => *frame_type,
            Frame::SameLocalsOneStackItemExtended { offset_delta: _, stack: _ } => 247,
            Frame::Chop { frame_type, offset_delta: _ } => *frame_type,
            Frame::SameExtended { offset_delta: _ } => 251,
            Frame::Append { frame_type, offset_delta: _, locals: _ } => *frame_type,
            Frame::Full { offset_delta: _, locals: _, stack: _ } => 255,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VerificationType {
    Top = 0,
    Integer = 1,
    Float = 2,
    Double = 3,
    Long = 4,
    Null = 5,
    UninitializedThis = 6,
    Object { pool_index: constantpool::Index } = 7,
    Uninitialized { offset: u16 } = 8
}
