use crate::parser::classfile::attribute::verificationtypes::VerificationType;

pub enum StackMapFrame {
    SameFrame { frame_type: u8 },
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
    SameFrameExtended { offset_delta: u16, },
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

impl StackMapFrame {
    pub fn frame_type(&self) -> u8 {
        match self {
            StackMapFrame::SameFrame { frame_type } => *frame_type,
            StackMapFrame::SameLocalsOneStackItem { frame_type, stack: _ } => *frame_type,
            StackMapFrame::SameLocalsOneStackItemExtended { offset_delta: _, stack: _ } => 247,
            StackMapFrame::Chop { frame_type, offset_delta: _ } => *frame_type,
            StackMapFrame::SameFrameExtended { offset_delta: _ } => 251,
            StackMapFrame::Append { frame_type, offset_delta: _, locals: _ } => *frame_type,
            StackMapFrame::Full { offset_delta: _, locals: _, stack: _ } => 255,
        }
    }
}
