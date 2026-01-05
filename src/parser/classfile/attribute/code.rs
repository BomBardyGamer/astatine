use crate::parser::classfile::attribute::CodeAttribute;
use crate::parser::classfile::attribute::stackmap;

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
    use crate::parser::classfile::attribute::names::{Names, Nameable, impl_attr_name};

    impl_attr_name!(Code, CODE);
    impl_attr_name!(StackMapTable, STACK_MAP_TABLE);
}

mod _parse {
    use super::*;
    use crate::parser::{Parse, ParserError, BinaryReader};

    impl Parse<Code> for Code {
        fn parse(buf: &mut BinaryReader) -> Result<Code, ParserError> {
            // 2 max stack, 2 max locals, 4 code length
            buf.check_bytes(2 + 2 + 4, "code - max stack, max locals, code length")?;

            // Safety: Guaranteed by check_bytes
            let max_stack = unsafe { buf.unsafe_read_u16() };
            let max_locals = unsafe { buf.unsafe_read_u16() };
            let code_len = unsafe { buf.unsafe_read_u16() };

            buf.check_bytes(code_len as usize, "code - code array")?;
            let mut code = Vec::with_capacity(code_len as usize);
            buf.read(&mut code);

            let exceptions = parse_exceptions(buf)?;

            Ok(Code {
                max_stack,
                max_locals,
                code,
                exceptions,
                attributes: vec![] // TODO: Attributes
            })
        }
    }

    fn parse_exceptions(buf: &mut BinaryReader) -> Result<Vec<Exception>, ParserError> {
        buf.check_bytes(2, "code - exceptions")?;

        let len = unsafe { buf.unsafe_read_u16() };
        let mut exceptions = Vec::with_capacity(len as usize);

        for _ in 0..len {
            exceptions.push(Exception::parse(buf)?);
        }
        Ok(exceptions)
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
            buf.check_bytes(2, "code - stack map table")?;

            // SAFETY: Guaranteed by check_bytes
            let len = unsafe { buf.unsafe_read_u16() };
            let mut entries = Vec::with_capacity(len as usize);

            for _ in 0..len {
                entries.push(stackmap::Frame::parse(buf)?);
            }

            Ok(StackMapTable { entries })
        }
    }
}
