use std::cell::{Cell, RefCell};
use crate::class::{Class, ClassObject};
use crate::class::parse::{BinaryReader, ParseError};
use crate::types::{AccessFlags, Array, Jboolean, Jbyte, Jchar, Jdouble, Jfloat, Jint, Jlong, Jshort};

pub struct Field {
    name: String,
    descriptor: String,
    access_flags: AccessFlags
}

fn parse_field(class: &mut Class, buf: &mut BinaryReader) -> Result<Field, ParseError> {
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

    Ok(Field {
        name,
        descriptor,
        access_flags: AccessFlags::new(flags),
    })
}
