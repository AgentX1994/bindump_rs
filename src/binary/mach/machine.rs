use std::fmt;

use derive_try_from_primitive::TryFromPrimitive;
use nom::{combinator::map, error::context, number::Endianness};

use crate::parse;

// From machine.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i32)]
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

impl CpuType {
    pub(crate) fn parse(
        endianness: Endianness,
    ) -> impl FnMut(parse::Input) -> parse::ParseResult<Self> {
        move |input: parse::Input| {
            context(
                "Parse Cpu Type",
                map(nom::number::complete::i32(endianness), |typ| {
                    Self::try_from(typ).expect("Invalid Cpu Type!")
                }),
            )(input)
        }
    }
}

impl fmt::Display for CpuType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuType::Any => write!(f, "Any"),
            CpuType::Vax => write!(f, "Vax"),
            CpuType::Mc680x0 => write!(f, "Mc680x0"),
            CpuType::X86 => write!(f, "X86"),
            CpuType::X86_64 => write!(f, "X86_64"),
            CpuType::Mc98000 => write!(f, "Mc98000"),
            CpuType::Hppa => write!(f, "Hppa"),
            CpuType::Arm => write!(f, "Arm"),
            CpuType::Arm64 => write!(f, "Arm64"),
            CpuType::Arm64_32 => write!(f, "Arm64_32"),
            CpuType::Mc88000 => write!(f, "Mc88000"),
            CpuType::Sparc => write!(f, "Sparc"),
            CpuType::I860 => write!(f, "I860"),
            CpuType::PowerPc => write!(f, "PowerPc"),
            CpuType::PowerPc64 => write!(f, "PowerPc64"),
        }
    }
}

// TODO: CPU Subtypes
