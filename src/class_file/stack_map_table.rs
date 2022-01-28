use bytes::{Buf, Bytes};
use enum_as_inner::EnumAsInner;
use crate::utils::buffer::BufferExtras;

#[derive(Debug)]
pub struct StackMapTable {
    frames: Vec<StackMapFrame>
}

impl StackMapTable {
    pub(crate) fn parse(buf: &mut Bytes) -> Self {
        StackMapTable::new(buf.get_generic_u16_array(|buf| StackMapFrame::parse(buf)))
    }

    pub const fn new(frames: Vec<StackMapFrame>) -> Self {
        StackMapTable { frames }
    }

    pub fn frames(&self) -> &[StackMapFrame] {
        self.frames.as_slice()
    }

    pub fn get(&self, index: usize) -> Option<&StackMapFrame> {
        self.frames.get(index)
    }
}

#[derive(Debug, Copy, Clone, EnumAsInner)]
pub enum StackFrameType {
    Same,
    SameLocalsOneStack,
    SameLocalsOneStackExtended,
    Chop,
    SameExtended,
    Append,
    Full
}

#[derive(Debug)]
pub struct StackMapFrame {
    frame_type: StackFrameType,
    offset_delta: u16,
    stack: Vec<VerificationType>,
    locals: Vec<VerificationType>
}

impl StackMapFrame {
    pub(crate) fn parse(buf: &mut Bytes) -> Self {
        let frame_type = buf.get_u8();
        let offset_delta;
        let mut stack = Vec::new();
        let mut locals = Vec::new();

        let stack_frame_type = match frame_type {
            0..=63 => {
                offset_delta = frame_type as u16;
                StackFrameType::Same
            },
            64..=127 => {
                offset_delta = (frame_type - 64) as u16;
                stack.push(VerificationType::parse(buf));
                StackFrameType::SameLocalsOneStack
            },
            247 => {
                offset_delta = buf.get_u16();
                stack.push(VerificationType::parse(buf));
                StackFrameType::SameLocalsOneStackExtended
            },
            248..=250 => {
                offset_delta = buf.get_u16();
                StackFrameType::Chop
            },
            251 => {
                offset_delta = buf.get_u16();
                StackFrameType::SameExtended
            },
            252..=254 => {
                offset_delta = buf.get_u16();
                StackMapFrame::parse_types((frame_type - 251) as usize, &mut locals, buf);
                StackFrameType::Append
            },
            255 => {
                offset_delta = buf.get_u16();
                StackMapFrame::parse_types(buf.get_u16() as usize, &mut locals, buf);
                StackMapFrame::parse_types(buf.get_u16() as usize, &mut stack, buf);
                StackFrameType::Full
            },
            _ => panic!("Invalid stack map frame type {}!", frame_type)
        };
        StackMapFrame::new(stack_frame_type, offset_delta, stack, locals)
    }

    #[inline]
    fn parse_types(count: usize, result: &mut Vec<VerificationType>, buf: &mut Bytes) {
        for _ in 0..count {
            result.push(VerificationType::parse(buf));
        }
    }

    pub const fn new(
        frame_type: StackFrameType,
        offset_delta: u16,
        stack: Vec<VerificationType>,
        locals: Vec<VerificationType>
    ) -> Self {
        StackMapFrame { frame_type, offset_delta, stack, locals }
    }

    pub fn frame_type(&self) -> StackFrameType {
        self.frame_type
    }

    pub fn offset_delta(&self) -> u16 {
        self.offset_delta
    }

    pub fn stack(&self) -> &[VerificationType] {
        self.stack.as_slice()
    }

    pub fn locals(&self) -> &[VerificationType] {
        self.locals.as_slice()
    }
}

const ITEM_TOP: u8 = 0;
const ITEM_INTEGER: u8 = 1;
const ITEM_FLOAT: u8 = 2;
const ITEM_DOUBLE: u8 = 3;
const ITEM_LONG: u8 = 4;
const ITEM_NULL: u8 = 5;
const ITEM_UNINITIALIZED_THIS: u8 = 6;
const ITEM_OBJECT: u8 = 7;
const ITEM_UNINITIALIZED: u8 = 8;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, EnumAsInner)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Double,
    Long,
    Null,
    UninitializedThis,
    Object { constant_pool_index: u16 },
    Uninitialized { offset: u16 }
}

impl VerificationType {
    fn parse(buf: &mut Bytes) -> Self {
        let tag = buf.get_u8();
        match tag {
            ITEM_TOP => VerificationType::Top,
            ITEM_INTEGER => VerificationType::Integer,
            ITEM_FLOAT => VerificationType::Float,
            ITEM_DOUBLE => VerificationType::Double,
            ITEM_LONG => VerificationType::Long,
            ITEM_NULL => VerificationType::Null,
            ITEM_UNINITIALIZED_THIS => VerificationType::UninitializedThis,
            ITEM_OBJECT => VerificationType::Object { constant_pool_index: buf.get_u16() },
            ITEM_UNINITIALIZED => VerificationType::Uninitialized { offset: buf.get_u16() },
            _ => panic!("Could not parse verification type with tag {}!", tag)
        }
    }
}
