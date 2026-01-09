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
    Top = VerificationType::TOP,
    Integer = VerificationType::INTEGER,
    Float = VerificationType::FLOAT,
    Double = VerificationType::DOUBLE,
    Long = VerificationType::LONG,
    Null = VerificationType::NULL,
    UninitializedThis = VerificationType::UNINIT_THIS,
    Object { pool_index: constantpool::Index } = VerificationType::OBJECT,
    Uninitialized { offset: u16 } = VerificationType::UNINIT,
}

impl VerificationType {
    const TOP: u8 = 0;
    const INTEGER: u8 = 1;
    const FLOAT: u8 = 2;
    const DOUBLE: u8 = 3;
    const LONG: u8 = 4;
    const NULL: u8 = 5;
    const UNINIT_THIS: u8 = 6;
    const OBJECT: u8 = 7;
    const UNINIT: u8 = 8;
}

mod _parse {
    use crate::buf_read_named_type_vec;
    use super::*;
    use crate::loader::{BinaryReader, Parse, ParseError};

    impl Parse<Frame> for Frame {
        fn parse(buf: &mut BinaryReader) -> Result<Frame, ParseError> {
            buf.check_bytes(1, "stack map frame")?;

            // SAFETY: Guaranteed by check_bytes
            let frame_type = unsafe { buf.unsafe_read_u8() };
            if frame_type > 127 && frame_type < 247 {
                // This range is reserved for future use and is this invalid
                return ParseError::new(format!("stack map frame - invalid frame type {frame_type}")).into()
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

    fn parse_same_locals_one_stack_item(buf: &mut BinaryReader, frame_type: u8) -> Result<Frame, ParseError> {
        let stack = VerificationType::parse(buf)
            .map_err(ParseError::wrap("same locals one stack item - stack"))?;
        Ok(Frame::SameLocalsOneStackItem { frame_type, stack })
    }

    fn parse_same_locals_one_stack_item_extended(buf: &mut BinaryReader) -> Result<Frame, ParseError> {
        buf.check_bytes(2, "same locals one stack item extended - offset delta")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };
        let stack = VerificationType::parse(buf)
            .map_err(ParseError::wrap("same locals one stack item extended - stack"))?;
        Ok(Frame::SameLocalsOneStackItemExtended { offset_delta, stack })
    }

    fn parse_chop(buf: &mut BinaryReader, frame_type: u8) -> Result<Frame, ParseError> {
        buf.check_bytes(2, "chop - offset delta")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };
        Ok(Frame::Chop { frame_type, offset_delta })
    }

    fn parse_same_frame_extended(buf: &mut BinaryReader) -> Result<Frame, ParseError> {
        buf.check_bytes(2, "same frame extended - offset delta")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };
        Ok(Frame::SameExtended { offset_delta })
    }

    fn parse_append(buf: &mut BinaryReader, frame_type: u8) -> Result<Frame, ParseError> {
        buf.check_bytes(2, "append - offset delta")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };

        // Cannot use macro as num_locals is not using a length defined in type
        let num_locals = frame_type - 251;
        let mut locals = Vec::with_capacity(num_locals as usize);

        for i in 0..num_locals {
            let local = VerificationType::parse(buf)
                .map_err(ParseError::wrap(format!("append - locals - idx {i}")))?;
            locals.push(local);
        }
        Ok(Frame::Append { frame_type, offset_delta, locals })
    }

    fn parse_full(buf: &mut BinaryReader) -> Result<Frame, ParseError> {
        buf.check_bytes(2, "full")?;

        // SAFETY: Guaranteed by check_bytes
        let offset_delta = unsafe { buf.unsafe_read_u16() };

        buf_read_named_type_vec!(VerificationType, locals, buf,
            "full - locals", "full - locals - idx {}");
        buf_read_named_type_vec!(VerificationType, stack, buf,
            "full - stack", "full - stack - idx {}");

        Ok(Frame::Full { offset_delta, locals, stack })
    }

    impl Parse<VerificationType> for VerificationType {
        fn parse(buf: &mut BinaryReader) -> Result<VerificationType, ParseError> {
            buf.check_bytes(1, "stack map verification type")?;

            let tag = unsafe { buf.unsafe_read_u8() };
            match tag {
                VerificationType::TOP => Ok(VerificationType::Top),
                VerificationType::INTEGER => Ok(VerificationType::Integer),
                VerificationType::FLOAT => Ok(VerificationType::Float),
                VerificationType::DOUBLE => Ok(VerificationType::Double),
                VerificationType::LONG => Ok(VerificationType::Long),
                VerificationType::NULL => Ok(VerificationType::Null),
                VerificationType::UNINIT_THIS => Ok(VerificationType::UninitializedThis),
                VerificationType::OBJECT => {
                    buf.check_bytes(2, "stack map verification type - object")?;
                    let pool_index = unsafe { buf.unsafe_read_u16() };
                    Ok(VerificationType::Object { pool_index })
                },
                VerificationType::UNINIT => {
                    buf.check_bytes(2, "stack map verification type - uninitialized")?;
                    let offset = unsafe { buf.unsafe_read_u16() };
                    Ok(VerificationType::Uninitialized { offset })
                },
                _ => ParseError::new(format!("invalid verification type tag {tag}")).into()
            }
        }
    }
}
