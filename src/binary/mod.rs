use std::{fmt, path::Path};

pub mod elf;
pub mod mach;
pub mod pe;

use elf::Elf;
use mach::Mach;
use pe::Pe;

use crate::error::BinDumpResult;

#[derive(Debug)]
pub enum Object {
    Pe(Pe),
    Elf(Elf),
    Mach(Mach),
}

impl Object {
    pub fn load<P: AsRef<Path>>(path: P) -> BinDumpResult<Self> {
        let data = std::fs::read(path)?;
        // Check magic bytes
        // TODO find a better way to match the magic bytes?
        match data[0..4] {
            // PE
            [0x4d, 0x5a, _, _] => Ok(Self::Pe(Pe::load(data))),
            // ELF
            [0x7f, 0x45, 0x4c, 0x46] => Ok(Self::Elf(Elf::load(data))),
            // Mach has multiple possible magic byte sequences
            [0xca, 0xfe, 0xba, 0xbe]
            | [0xfe, 0xed, 0xfa, 0xce]
            | [0xfe, 0xed, 0xfa, 0xcf]
            | [0xcf, 0xfa, 0xed, 0xfe]
            | [0xce, 0xfa, 0xed, 0xfe] => match Mach::parse(&data) {
                Ok((_input, mach)) => Ok(Self::Mach(mach)),
                Err(err) => match err.to_owned() {
                    nom::Err::Incomplete(_x) => todo!(),
                    nom::Err::Error(e) => Err(e.into()),
                    nom::Err::Failure(e) => Err(e.into()),
                },
            },
            _ => todo!(),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Pe(pe) => write!(f, "{}", pe),
            Object::Elf(elf) => write!(f, "{}", elf),
            Object::Mach(macho) => write!(f, "{}", macho),
        }
    }
}
