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

const VERIFICATION_TYPE_TOP: u8 = 0;
const VERIFICATION_TYPE_INTEGER: u8 = 1;
const VERIFICATION_TYPE_FLOAT: u8 = 2;
const VERIFICATION_TYPE_DOUBLE: u8 = 3;
const VERIFICATION_TYPE_LONG: u8 = 4;
const VERIFICATION_TYPE_NULL: u8 = 5;
const VERIFICATION_TYPE_UNINIT_THIS: u8 = 6;
const VERIFICATION_TYPE_OBJECT: u8 = 7;
const VERIFICATION_TYPE_UNINIT: u8 = 8;
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VerificationType {
    Top = VERIFICATION_TYPE_TOP,
    Integer = VERIFICATION_TYPE_INTEGER,
    Float = VERIFICATION_TYPE_FLOAT,
    Double = VERIFICATION_TYPE_DOUBLE,
    Long = VERIFICATION_TYPE_LONG,
    Null = VERIFICATION_TYPE_NULL,
    UninitializedThis = VERIFICATION_TYPE_UNINIT_THIS,
    Object { pool_index: constantpool::Index } = VERIFICATION_TYPE_OBJECT,
    Uninitialized { offset: u16 } = VERIFICATION_TYPE_UNINIT,
}

mod _parse {
    use super::*;
    use crate::parser::{BinaryReader, Parse, ParserError};

    impl Parse<Frame> for Frame {
        fn parse(buf: &mut BinaryReader) -> Result<Frame, ParserError> {
            buf.check_bytes(1, "stack map frame")?;

            // SAFETY: Guaranteed by check_bytes
            let frame_type = unsafe { buf.unsafe_read_u8() };
            if frame_type > 127 && frame_type < 247 {
                // This range is reserved for future use and is this invalid
                return ParserError::new(format!("stack map frame - invalid frame type {frame_type}")).into()
            }

            match frame_type {
                0..=63 => Ok(Frame::Same { frame_type }),
                64..=127 => parse_same_locals_one_stack_item(buf, frame_type),
                247 => parse_same_locals_one_stack_item_extended(buf),
                248..=250 => parse_chop(buf, frame_type),
                251 => parse_same_frame_extended(buf),
                252..=254 => parse_append(buf, frame_type),
                255 => parse_full(buf),
                // This branch is impossible to reach, as the only remaining range
                // is 128..246 which is covered by the if that comes before this match
                _ => unreachable!()
            }
        }
    }

    fn parse_same_locals_one_stack_item(buf: &mut BinaryReader, frame_type: u8) -> Result<Frame, ParserError> {
        let stack = VerificationType::parse(buf)
            .map_err(ParserError::wrap("same locals one stack item"))?;
        Ok(Frame::SameLocalsOneStackItem { frame_type, stack })
    }

    fn parse_same_locals_one_stack_item_extended(buf: &mut BinaryReader) -> Result<Frame, ParserError> {
        buf.check_bytes(2, "same locals one stack item extended")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };
        let stack = VerificationType::parse(buf)
            .map_err(ParserError::wrap("same locals one stack item extended"))?;
        Ok(Frame::SameLocalsOneStackItemExtended { offset_delta, stack })
    }

    fn parse_chop(buf: &mut BinaryReader, frame_type: u8) -> Result<Frame, ParserError> {
        buf.check_bytes(2, "chop")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };
        Ok(Frame::Chop { frame_type, offset_delta })
    }

    fn parse_same_frame_extended(buf: &mut BinaryReader) -> Result<Frame, ParserError> {
        buf.check_bytes(2, "same frame extended")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };
        Ok(Frame::SameExtended { offset_delta })
    }

    fn parse_append(buf: &mut BinaryReader, frame_type: u8) -> Result<Frame, ParserError> {
        buf.check_bytes(2, "append")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };

        let num_locals = frame_type - 251;
        let mut locals = Vec::with_capacity(num_locals as usize);

        for _ in 0..num_locals {
            locals.push(VerificationType::parse(buf)?);
        }
        Ok(Frame::Append { frame_type, offset_delta, locals })
    }

    fn parse_full(buf: &mut BinaryReader) -> Result<Frame, ParserError> {
        // 2 offset delta, 2 number of locals
        buf.check_bytes(2 + 2, "full")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };
        let num_locals = unsafe { buf.unsafe_read_u16() };

        let mut locals = Vec::with_capacity(num_locals as usize);
        for _ in 0..num_locals {
            locals.push(VerificationType::parse(buf)?);
        }

        buf.check_bytes(2, "full")?;
        // SAFETY: Guaranteed by check_bytes
        let stack_size = unsafe { buf.unsafe_read_u16() };

        let mut stack = Vec::with_capacity(stack_size as usize);
        for _ in 0..stack_size {
            stack.push(VerificationType::parse(buf)?);
        }

        Ok(Frame::Full { offset_delta, locals, stack })
    }

    impl Parse<VerificationType> for VerificationType {
        fn parse(buf: &mut BinaryReader) -> Result<VerificationType, ParserError> {
            buf.check_bytes(1, "stack map verification type")?;

            let tag = unsafe { buf.unsafe_read_u8() };
            match tag {
                VERIFICATION_TYPE_TOP => Ok(VerificationType::Top),
                VERIFICATION_TYPE_INTEGER => Ok(VerificationType::Integer),
                VERIFICATION_TYPE_FLOAT => Ok(VerificationType::Float),
                VERIFICATION_TYPE_DOUBLE => Ok(VerificationType::Double),
                VERIFICATION_TYPE_LONG => Ok(VerificationType::Long),
                VERIFICATION_TYPE_NULL => Ok(VerificationType::Null),
                VERIFICATION_TYPE_UNINIT_THIS => Ok(VerificationType::UninitializedThis),
                VERIFICATION_TYPE_OBJECT => {
                    buf.check_bytes(2, "stack map verification type - object")?;
                    let pool_index = unsafe { buf.unsafe_read_u16() };
                    Ok(VerificationType::Object { pool_index })
                },
                VERIFICATION_TYPE_UNINIT => {
                    buf.check_bytes(2, "stack map verification type - uninitialized")?;
                    let offset = unsafe { buf.unsafe_read_u16() };
                    Ok(VerificationType::Uninitialized { offset })
                },
                _ => ParserError::new(format!("invalid verification type tag {tag}")).into()
            }
        }
    }
}
