pub mod load_commands;
pub mod machine;

use std::fmt;

use derive_try_from_primitive::TryFromPrimitive;
use load_commands::LoadCommand;
use machine::CpuType;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::context,
    multi::count,
    number::{
        complete::{self, be_u32},
        Endianness,
    },
    sequence::tuple,
};

use crate::parse::{self, ParseResult};

#[derive(Debug)]
pub enum Mach {
    Universal(Vec<MachArch>),
    MachO(MachODetails),
}

// From loader.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum FileType {
    Object = 1,
    Executable = 2,
    FixedVmLibrary = 3,
    Core = 4,
    Preload = 5,
    DynamicLibrary = 6,
    DynamicLinkEditor = 7,
    Bundle = 8,
    DynamicLibraryStub = 9,
    DebugSymbols = 10,
    Kexts = 11,
    Fileset = 12,
    GpuProgram = 13,
    GpuDynamicLibrary = 14,
}

impl FileType {
    pub(crate) fn parse(
        endianness: Endianness,
    ) -> impl FnMut(parse::Input) -> parse::ParseResult<Self> {
        move |input: parse::Input| {
            context(
                "Parse File Type",
                map(nom::number::complete::u32(endianness), |typ| {
                    Self::try_from(typ).expect("Invalid File Type!")
                }),
            )(input)
        }
    }
}

#[derive(Debug)]
pub enum MachArch {
    Arch32(MachArchDetails),
    // TODO: 64 bit
}

impl fmt::Display for MachArch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MachArch::Arch32(arch) => write!(f, "{}", arch),
        }
    }
}

#[derive(Debug)]
pub struct MachArchDetails {
    /// The type of CPU
    cpu_type: CpuType,
    /// The sub-type of the CPU
    cpu_subtype: i32,
    /// The offset of this Mach O file into the universal binary
    offset: u32,
    /// The size of this Mach O file
    size: u32,
    /// The alignment of this Mach O File
    align: u32,
    mach_object: Mach,
}

impl MachArchDetails {
    fn parse<'a>(input: parse::Input<'a>, full_input: parse::Input<'a>) -> ParseResult<'a, Self> {
        let (input, (cpu_type, cpu_subtype, offset, size, align)) = context(
            "Parse Mach Arch Header",
            tuple((
                CpuType::parse(Endianness::Big),
                complete::i32(Endianness::Big),
                complete::u32(Endianness::Big),
                complete::u32(Endianness::Big),
                map(complete::u32(Endianness::Big), |align| 2u32.pow(align)),
            )),
        )(input)?;
        let (_, mach_object) = Mach::parse(&full_input[offset as usize..][..size as usize])?;
        Ok((
            input,
            Self {
                cpu_type,
                cpu_subtype,
                offset,
                size,
                align,
                mach_object,
            },
        ))
    }
}

impl fmt::Display for MachArchDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cpu Type: {}", self.cpu_type)?;
        writeln!(f, "Cpu Subtype: {:x}", self.cpu_subtype)?;
        writeln!(f, "Offset: {:#x}", self.offset)?;
        writeln!(f, "Size: {} bytes", self.size)?;
        writeln!(f, "Align: {}", self.align)?;
        writeln!(f, "Mach Object:")?;
        writeln!(f, "{}", self.mach_object)
    }
}

#[derive(Debug)]
pub struct MachODetails {
    header: MachHeader,
    load_commands: Vec<LoadCommand>,
}

#[derive(Debug)]
pub struct MachHeader {
    cpu_type: CpuType,
    cpu_subtype: i32,
    file_type: FileType,
    number_of_load_commands: u32,
    total_command_size: u32,
    flags: u32,
    reserved: u32,
}

impl MachHeader {
    fn parse(input: parse::Input, endianness: Endianness) -> parse::ParseResult<Self> {
        let (
            input,
            (
                cpu_type,
                cpu_subtype,
                file_type,
                number_of_load_commands,
                total_command_size,
                flags,
                reserved,
            ),
        ) = context(
            "Load MachO Header",
            tuple((
                CpuType::parse(endianness),
                complete::i32(endianness),
                FileType::parse(endianness),
                complete::u32(endianness),
                complete::u32(endianness),
                complete::u32(endianness),
                complete::u32(endianness),
            )),
        )(input)?;
        Ok((
            input,
            Self {
                cpu_type,
                cpu_subtype,
                file_type,
                number_of_load_commands,
                total_command_size,
                flags,
                reserved,
            },
        ))
    }
}

impl Mach {
    pub(crate) fn parse(input: parse::Input) -> parse::ParseResult<Self> {
        let full_input = input;
        let (input, magic) = context(
            "Magic",
            alt((
                tag(&[0xca, 0xfe, 0xba, 0xbe]),
                tag(&[0xfe, 0xed, 0xfa, 0xce]),
                tag(&[0xfe, 0xed, 0xfa, 0xcf]),
                tag(&[0xcf, 0xfa, 0xed, 0xfe]),
                tag(&[0xce, 0xfa, 0xed, 0xfe]),
            )),
        )(input)?;
        match magic {
            [0xca, 0xfe, 0xba, 0xbe] => {
                Self::parse_universal_32_bit_little_endian(input, full_input)
            }
            [0xfe, 0xed, 0xfa, 0xce] => todo!(),
            [0xfe, 0xed, 0xfa, 0xcf] => todo!(),
            [0xcf, 0xfa, 0xed, 0xfe] => Self::parse_64_bit_little_endian(input, full_input),
            [0xce, 0xfa, 0xed, 0xfe] => todo!(),
            _ => unreachable!(),
        }
    }

    fn parse_universal_32_bit_little_endian<'a>(
        input: parse::Input<'a>,
        full_input: parse::Input<'a>,
    ) -> ParseResult<'a, Self> {
        let (input, num_arches) = context("Number of arches", be_u32)(input)?;
        let (input, arches) = {
            let mut input = input;
            let mut arches = Vec::new();
            for _ in 0..num_arches {
                let (next_input, arch) = MachArchDetails::parse(input, full_input)?;
                input = next_input;
                arches.push(MachArch::Arch32(arch));
            }
            (input, arches)
        };

        Ok((input, Self::Universal(arches)))
    }

    fn parse_64_bit_little_endian<'a>(
        input: parse::Input<'a>,
        _full_input: parse::Input<'a>,
    ) -> parse::ParseResult<'a, Self> {
        let (input, header) = MachHeader::parse(input, Endianness::Little)?;
        let (input, load_commands) = context(
            "Load Load Commands",
            count(
                LoadCommand::parse(Endianness::Little),
                header.number_of_load_commands as usize,
            ),
        )(input)?;
        Ok((
            input,
            Mach::MachO(MachODetails {
                header,
                load_commands,
            }),
        ))
    }
}

impl fmt::Display for Mach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mach::Universal(arches) => {
                writeln!(f, "Universal Binary: {} arches", arches.len())?;
                for (i, arch) in arches.iter().enumerate() {
                    writeln!(f, "Arch {}:", i)?;
                    writeln!(f, "{}", arch)?;
                }
                Ok(())
            }
            Mach::MachO(details) => {
                let header = &details.header;
                writeln!(f, "Mach-O: {} Architecture", header.cpu_type)?;
                writeln!(f, "Load Commands:")?;
                for (i, command) in details.load_commands.iter().enumerate() {
                    writeln!(f, "Load Command {}", i)?;
                    writeln!(f, "{}", command)?;
                }
                Ok(())
            }
        }
    }
}
