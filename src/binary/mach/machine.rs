#[derive(Clone, Copy, Debug)]
pub enum Endianness {
    Little,
    Big,
}

// From machine.h
#[derive(Debug)]
pub enum CpuType {
    Any = -1,
    Vax = 1,
    Mc680x0 = 6,
    X86 = 7,
    X86_64 = 16777223,
    Mc98000 = 10,
    Hppa = 11,
    Arm = 12,
    Arm64 = 16777228,
    Arm64_32 = 33554444,
    Mc88000 = 13,
    Sparc = 14,
    I860 = 15,
    PowerPc = 18,
    PowerPc64 = 16777234,
}

impl TryFrom<i32> for CpuType {
    // TODO: Implement a proper error type
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::Any),
            1 => Ok(Self::Vax),
            6 => Ok(Self::Mc680x0),
            7 => Ok(Self::X86),
            16777223 => Ok(Self::X86_64),
            10 => Ok(Self::Mc98000),
            11 => Ok(Self::Hppa),
            12 => Ok(Self::Arm),
            16777228 => Ok(Self::Arm64),
            33554444 => Ok(Self::Arm64_32),
            13 => Ok(Self::Mc88000),
            14 => Ok(Self::Sparc),
            15 => Ok(Self::I860),
            18 => Ok(Self::PowerPc),
            16777234 => Ok(Self::PowerPc64),
            _ => Err(value),
        }
    }
}

// TODO: CPU Subtypes
