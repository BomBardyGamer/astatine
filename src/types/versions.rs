use std::fmt;

#[derive(Debug)]
pub enum ClassFileVersion {
    Java1_1 = 45,
    Java1_2 = 46,
    Java1_3 = 47,
    Java1_4 = 48,
    Java5 = 49,
    Java6 = 50,
    Java7 = 51,
    Java8 = 52,
    Java9 = 53,
    Java10 = 54,
    Java11 = 55,
    Java12 = 56,
    Java13 = 57,
    Java14 = 58,
    Java15 = 59,
    Java16 = 60,
    Java17 = 61,
    Java18 = 62,
    Java19 = 63,
    Java20 = 64,
    Java21 = 65,
    Java22 = 66,
    Java23 = 67,
    Java24 = 68,
    Java25 = 69,
}

impl fmt::Display for ClassFileVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub const CURRENT_VIRTUAL_MACHINE_VERSION: ClassFileVersion = ClassFileVersion::Java25;
