use crate::{buf_read_named_type_arr, buf_read_u8_arr_lensize};
use crate::class::Class;
use crate::class::parse::{BinaryReader, ParseError};
use crate::loader::Parse;
use crate::types::{AccessFlags, Array};

pub struct Method {
    name: String,
    descriptor: String,
    access_flags: AccessFlags,
    code: Code,
}

pub struct Code {
    max_stack: u16,
    max_locals: u16,
    code: Array<u8>,
    exception_handlers: Array<ExceptionHandler>,
}

pub struct ExceptionHandler {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub fn parse_method(class: &mut Class, buf: &mut BinaryReader) -> Result<Method, ParseError> {
    // 2 access flags, 2 name index, 2 descriptor index
    buf.check_bytes(2 + 2 + 2, "access flags, name index, descriptor index")?;

    // SAFETY: Guaranteed by check_bytes
    let flags = unsafe { buf.unsafe_read_u16() };
    let name_index = unsafe { buf.unsafe_read_u16() };
    let descriptor_index = unsafe { buf.unsafe_read_u16() };

    let name = class.constant_pool.resolve_utf8(name_index)
        .expect("bad things").as_string();
    let descriptor = class.constant_pool.resolve_utf8(descriptor_index)
        .expect("bad things").as_string();

    let code = Code::parse(buf)?;

    Ok(Method {
        name,
        descriptor,
        access_flags: AccessFlags::new(flags),
        code
    })
}

impl Code {
    fn parse(buf: &mut BinaryReader) -> Result<Code, ParseError> {
        // 2 max stack, 2 max locals, 4 code length
        buf.check_bytes(2 + 2, "code - max stack, max locals, code length")?;

        // Safety: Guaranteed by check_bytes
        let max_stack = unsafe { buf.unsafe_read_u16() };
        let max_locals = unsafe { buf.unsafe_read_u16() };

        buf_read_u8_arr_lensize!(code, buf, unsafe_read_u32, "code - code array");
        buf_read_named_type_arr!(ExceptionHandler, exception_handlers, buf,
                "code - exception handlers", "code - exception handlers - idx {}");

        // TODO: Parse stack map table and verify here

        Ok(Code {
            max_stack,
            max_locals,
            code,
            exception_handlers
        })
    }
}

impl ExceptionHandler {
    fn parse(buf: &mut BinaryReader) -> Result<ExceptionHandler, ParseError> {
        buf.check_bytes(2 + 2 + 2 + 2, "exception")?;

        // SAFETY: Guaranteed by check_bytes
        let start_pc = unsafe { buf.unsafe_read_u16() };
        let end_pc = unsafe { buf.unsafe_read_u16() };
        let handler_pc = unsafe { buf.unsafe_read_u16() };
        let catch_type = unsafe { buf.unsafe_read_u16() };

        Ok(ExceptionHandler { start_pc, end_pc, handler_pc, catch_type })
    }
}
