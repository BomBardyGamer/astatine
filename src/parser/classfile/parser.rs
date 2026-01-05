use super::{constantpool, ClassFile};
use crate::parser::{Parse, ParserError};
use crate::parser::reader::BinaryReader;
use crate::types;
use crate::types::ClassFileVersion;

impl Parse<ClassFile> for ClassFile {
    fn parse(buf: &mut BinaryReader) -> Result<ClassFile, ParserError> {
        parse_impl(buf).map_err(classfile_err)
    }
}

fn parse_impl(buf: &mut BinaryReader) -> Result<ClassFile, ParserError> {
    read_and_check_magic(buf)?;

    let (minor_version, major_version): (u16, u16);
    {
        buf.check_bytes(2 + 2, "minor and major version")?;

        // SAFETY: Both guaranteed by check_bytes
        minor_version = unsafe { buf.unsafe_read_u16() };
        major_version = unsafe { buf.unsafe_read_u16() };
        check_major_minor(major_version, minor_version)?;
    }

    let pool = constantpool::Pool::parse(buf)
        .map_err(|err| ParserError::new(format!("bad constant pool: {err}")))?;

    // 2 for access flags, 2 for this class, 2 for super class
    buf.check_bytes(2 + 2 + 2, "access flags, this class, super class")?;

    // SAFETY: Next 3 reads guaranteed by above check_bytes
    let access_flags = unsafe { buf.unsafe_read_u16() };
    let this_class: constantpool::Index;
    {
        this_class = unsafe { buf.unsafe_read_u16() };
        if !pool.is_valid_index(this_class) {
            return ParserError::new("this class not in constant pool").into();
        }
    }
    let super_class: constantpool::Index;
    {
        super_class = unsafe { buf.unsafe_read_u16() };
        if super_class != 0 && !pool.is_valid_index(super_class) {
            return ParserError::new("super class not in constant pool").into();
        }
    }

    let mut interfaces: Vec<constantpool::Index>;
    {
        buf.check_bytes(2, "interfaces length")?;

        // SAFETY: Guaranteed by check_bytes
        let len = unsafe { buf.unsafe_read_u16() } as usize;
        buf.check_bytes(len * 2, "interfaces")?;

        interfaces = Vec::with_capacity(len);
        // SAFETY: Guaranteed by check_bytes on length
        unsafe {
            buf.unsafe_read_u16_slice(&mut interfaces)
        }
    }

    Ok(ClassFile {
        minor_version,
        major_version,
        constant_pool: pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields: vec![], // TODO: Fields
        methods: vec![], // TODO: Methods
        attributes: vec![], // TODO: Attributes
    })
}

fn classfile_err(err: ParserError) -> ParserError {
    ParserError::new(format!("malformed class file: {err:?}"))
}

fn read_and_check_magic(buf: &mut BinaryReader) -> Result<(), ParserError> {
    const CLASS_FILE_MAGIC_NUMBER: u32 = 0xCAFEBABE;

    buf.check_bytes(4, "classfile magic")?;

    // SAFETY: Guaranteed by check_bytes
    let magic = unsafe { buf.unsafe_read_u32() };
    match magic {
        CLASS_FILE_MAGIC_NUMBER => Ok(()),
        _ => ParserError::new(format!("invalid magic {magic} (not a classfile)")).into()
    }
}

fn check_major_minor(major: u16, minor: u16) -> Result<(), ParserError> {
    const MIN_SUPPORTED: ClassFileVersion = ClassFileVersion::Java1_1;
    const MAX_SUPPORTED: ClassFileVersion = ClassFileVersion::Java1_2;
    const SUPPORT_PREVIEW: bool = false; // TODO: Will we ever support preview features?

    if major < MIN_SUPPORTED as u16 {
        let msg = format!("major version {major} not supported (min is {MIN_SUPPORTED})");
        return ParserError::new(msg).into();
    }
    if major > MAX_SUPPORTED as u16 {
        let msg = format!("major version {major} not supported (max is {MAX_SUPPORTED})");
        return ParserError::new(msg).into();
    }

    if major > ClassFileVersion::Java12 as u16 {
        if minor != 0 && minor != 65535 {
            let msg = format!("minor version must be 0 or 65535 for classfiles major {major}, was {minor}");
            return ParserError::new(msg).into();
        }

        if minor == 65535 {
            if major != types::CURRENT_VIRTUAL_MACHINE_VERSION as u16 {
                return ParserError::new("Astatine only supports preview features for its current version").into();
            }
        }
    }

    Ok(())
}
