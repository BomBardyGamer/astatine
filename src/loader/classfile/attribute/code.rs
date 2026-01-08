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

use crate::loader::classfile::attribute::CodeAttribute;
use crate::loader::classfile::attribute::stackmap;

pub struct Code {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exceptions: Vec<Exception>,
    attributes: Vec<CodeAttribute>,
}

impl Code {
    pub fn max_stack(&self) -> u16 {
        self.max_stack
    }

    pub fn max_locals(&self) -> u16 {
        self.max_locals
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn exceptions(&self) -> &[Exception] {
        &self.exceptions
    }

    pub fn attributes(&self) -> &[CodeAttribute] {
        &self.attributes
    }
}

pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl Exception {
    pub fn start_pc(&self) -> u16 {
        self.start_pc
    }

    pub fn end_pc(&self) -> u16 {
        self.end_pc
    }

    pub fn handler_pc(&self) -> u16 {
        self.handler_pc
    }

    pub fn catch_type(&self) -> u16 {
        self.catch_type
    }
}

pub struct StackMapTable {
    entries: Vec<stackmap::Frame>,
}

impl StackMapTable {
    pub fn entries(&self) -> &[stackmap::Frame] {
        &self.entries
    }
}

mod _attr_name {
    use super::*;
    use crate::loader::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(Code, CODE);
    impl_attr_name!(StackMapTable, STACK_MAP_TABLE);
}

mod _parse {
    use crate::{buf_read_named_type_vec, buf_read_u8_vec_lensize};
    use super::*;
    use super::stackmap::Frame;
    use crate::loader::{Parse, ParserError, BinaryReader};

    impl Parse<Code> for Code {
        fn parse(buf: &mut BinaryReader) -> Result<Code, ParserError> {
            // 2 max stack, 2 max locals, 4 code length
            buf.check_bytes(2 + 2, "code - max stack, max locals, code length")?;

            // Safety: Guaranteed by check_bytes
            let max_stack = unsafe { buf.unsafe_read_u16() };
            let max_locals = unsafe { buf.unsafe_read_u16() };

            buf_read_u8_vec_lensize!(code, buf, unsafe_read_u32, "code - code array");
            buf_read_named_type_vec!(Exception, exceptions, buf,
                "code - exceptions", "code - exceptions - idx {}");

            Ok(Code {
                max_stack,
                max_locals,
                code,
                exceptions,
                attributes: vec![] // TODO: Attributes
            })
        }
    }

    impl Parse<Exception> for Exception {
        fn parse(buf: &mut BinaryReader) -> Result<Exception, ParserError> {
            buf.check_bytes(2 + 2 + 2 + 2, "exception")?;

            // SAFETY: Guaranteed by check_bytes
            let start_pc = unsafe { buf.unsafe_read_u16() };
            let end_pc = unsafe { buf.unsafe_read_u16() };
            let handler_pc = unsafe { buf.unsafe_read_u16() };
            let catch_type = unsafe { buf.unsafe_read_u16() };

            Ok(Exception { start_pc, end_pc, handler_pc, catch_type })
        }
    }

    impl Parse<StackMapTable> for StackMapTable {
        fn parse(buf: &mut BinaryReader) -> Result<StackMapTable, ParserError> {
            buf_read_named_type_vec!(Frame, entries, buf,
                "code - stack map table", "code - stack map table - idx {}");
            Ok(StackMapTable { entries })
        }
    }
}
