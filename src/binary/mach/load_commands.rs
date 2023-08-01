use std::ffi::CStr;
use std::fmt;

use nom::bytes::complete::take;
use nom::combinator::map;
use nom::error::context;
use nom::multi::count;
use nom::number::{complete, Endianness};
use nom::sequence::tuple;

use crate::parse;

#[derive(Debug)]
pub struct LoadCommand {
    pub size: u32,
    // TODO: Load commands
    pub command: Command,
}

impl LoadCommand {
    pub(super) fn parse(
        endianness: Endianness,
    ) -> impl FnMut(parse::Input) -> parse::ParseResult<Self> {
        move |input: parse::Input| {
            let start = input;
            let (input, (command_type, command_size)) = context(
                "Parse Load Command type and size",
                tuple((complete::u32(endianness), complete::u32(endianness))),
            )(input)?;
            let (_input, command) = Command::parse(command_type, command_size, input, endianness)?;
            Ok((
                &start[command_size as usize..],
                LoadCommand {
                    size: command_size,
                    command,
                },
            ))
        }
    }
}

impl fmt::Display for LoadCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Command Size: {} bytes", self.size)?;
        write!(f, "Command: {}", self.command)
    }
}

#[derive(Debug)]
pub enum Command {
    Segment(SegmentDetails),
    SymbolTable,
    SymbolSegment,
    Thread,
    UnixThread,
    LoadFixedVmLibrary,
    IdentifyFixedVmLibrary,
    Identify,
    IncludeFixedVmLibrary,
    Prepage,
    DynamicSymbolTable,
    LoadDynamicLibrary,
    IdentifyDynamicLibrary,
    LoadDynamicLinker,
    IdentifyDynamicLinker,
    PreboundDynamicLibrary,
    Routines,
    SubFramework,
    SubUmbrella,
    SubClient,
    SubLibrary,
    TwoLevelHints,
    PrebindChecksum,
    LoadWeakDynamicLibrary,
    Segment64(Segment64Details),
    Routines64,
    Uuid,
    RPath,
    CodeSignature,
    SegmentSplitInfo,
    ReexportDynamicLibrary,
    LazyLoadDynamicLibrary,
    EncryptionInfo,
    DynamicLinkerInfo,
    DynamicLinkerInfoOnly,
    LoadUpwardDynamicLibrary,
    VersionMinMacOsx,
    VersionMinIphoneOs,
    FunctionStarts,
    DynamicLinkerEnvironment,
    Main,
    DataInCode,
    SourceVersion,
    DynamicLibraryCodeSignDrs,
    EncryptionInfo64,
    LinkerOption,
    LinkerOptimizationHint,
    VersionMinTvOs,
    VersionMinWatchOs,
    Note,
    BuildVersion,
    DynamicLinkerExportsTrie,
    DynamicLinkerChainedFixups,
    FileSetEntry,
}

impl Command {
    pub(super) fn parse(
        command_type: u32,
        command_size: u32,
        input: parse::Input,
        endianness: Endianness,
    ) -> parse::ParseResult<Self> {
        match command_type {
            1 => context(
                "Parse Segment",
                map(SegmentDetails::parse(endianness), Self::Segment),
            )(input),
            2 => Ok((&input[command_size as usize - 8..], Self::SymbolTable)),
            3 => Ok((&input[command_size as usize - 8..], Self::SymbolSegment)),
            4 => Ok((&input[command_size as usize - 8..], Self::Thread)),
            5 => Ok((&input[command_size as usize - 8..], Self::UnixThread)),
            6 => Ok((
                &input[command_size as usize - 8..],
                Self::LoadFixedVmLibrary,
            )),
            7 => Ok((
                &input[command_size as usize - 8..],
                Self::IdentifyFixedVmLibrary,
            )),
            8 => Ok((&input[command_size as usize - 8..], Self::Identify)),
            9 => Ok((
                &input[command_size as usize - 8..],
                Self::IncludeFixedVmLibrary,
            )),
            10 => Ok((&input[command_size as usize - 8..], Self::Prepage)),
            11 => Ok((
                &input[command_size as usize - 8..],
                Self::DynamicSymbolTable,
            )),
            12 => Ok((
                &input[command_size as usize - 8..],
                Self::LoadDynamicLibrary,
            )),
            13 => Ok((
                &input[command_size as usize - 8..],
                Self::IdentifyDynamicLibrary,
            )),
            14 => Ok((&input[command_size as usize - 8..], Self::LoadDynamicLinker)),
            15 => Ok((
                &input[command_size as usize - 8..],
                Self::IdentifyDynamicLinker,
            )),
            16 => Ok((
                &input[command_size as usize - 8..],
                Self::PreboundDynamicLibrary,
            )),
            17 => Ok((&input[command_size as usize - 8..], Self::Routines)),
            18 => Ok((&input[command_size as usize - 8..], Self::SubFramework)),
            19 => Ok((&input[command_size as usize - 8..], Self::SubUmbrella)),
            20 => Ok((&input[command_size as usize - 8..], Self::SubClient)),
            21 => Ok((&input[command_size as usize - 8..], Self::SubLibrary)),
            22 => Ok((&input[command_size as usize - 8..], Self::TwoLevelHints)),
            23 => Ok((&input[command_size as usize - 8..], Self::PrebindChecksum)),
            /* 24 */
            0x80000018 => Ok((
                &input[command_size as usize - 8..],
                Self::LoadWeakDynamicLibrary,
            )),
            25 => context(
                "Parse Segment64",
                map(Segment64Details::parse(endianness), Self::Segment64),
            )(input),
            26 => Ok((&input[command_size as usize - 8..], Self::Routines64)),
            27 => Ok((&input[command_size as usize - 8..], Self::Uuid)),
            /* 28 */ 0x8000001c => Ok((&input[command_size as usize - 8..], Self::RPath)),
            29 => Ok((&input[command_size as usize - 8..], Self::CodeSignature)),
            30 => Ok((&input[command_size as usize - 8..], Self::SegmentSplitInfo)),
            /* 31 */
            0x8000001f => Ok((
                &input[command_size as usize - 8..],
                Self::ReexportDynamicLibrary,
            )),
            32 => Ok((
                &input[command_size as usize - 8..],
                Self::LazyLoadDynamicLibrary,
            )),
            33 => Ok((&input[command_size as usize - 8..], Self::EncryptionInfo)),
            34 => Ok((&input[command_size as usize - 8..], Self::DynamicLinkerInfo)),
            /* 34 */
            0x80000022 => Ok((
                &input[command_size as usize - 8..],
                Self::DynamicLinkerInfoOnly,
            )),
            /* 35 */
            0x80000023 => Ok((
                &input[command_size as usize - 8..],
                Self::LoadUpwardDynamicLibrary,
            )),
            36 => Ok((&input[command_size as usize - 8..], Self::VersionMinMacOsx)),
            37 => Ok((
                &input[command_size as usize - 8..],
                Self::VersionMinIphoneOs,
            )),
            38 => Ok((&input[command_size as usize - 8..], Self::FunctionStarts)),
            39 => Ok((
                &input[command_size as usize - 8..],
                Self::DynamicLinkerEnvironment,
            )),
            /* 40 */ 0x80000028 => Ok((&input[command_size as usize - 8..], Self::Main)),
            41 => Ok((&input[command_size as usize - 8..], Self::DataInCode)),
            42 => Ok((&input[command_size as usize - 8..], Self::SourceVersion)),
            43 => Ok((
                &input[command_size as usize - 8..],
                Self::DynamicLibraryCodeSignDrs,
            )),
            44 => Ok((&input[command_size as usize - 8..], Self::EncryptionInfo64)),
            45 => Ok((&input[command_size as usize - 8..], Self::LinkerOption)),
            46 => Ok((
                &input[command_size as usize - 8..],
                Self::LinkerOptimizationHint,
            )),
            47 => Ok((&input[command_size as usize - 8..], Self::VersionMinTvOs)),
            48 => Ok((&input[command_size as usize - 8..], Self::VersionMinWatchOs)),
            49 => Ok((&input[command_size as usize - 8..], Self::Note)),
            50 => Ok((&input[command_size as usize - 8..], Self::BuildVersion)),
            /* 51 */
            0x80000033 => Ok((
                &input[command_size as usize - 8..],
                Self::DynamicLinkerExportsTrie,
            )),
            /* 52 */
            0x80000034 => Ok((
                &input[command_size as usize - 8..],
                Self::DynamicLinkerChainedFixups,
            )),
            /* 53 */
            0x80000035 => Ok((&input[command_size as usize - 8..], Self::FileSetEntry)),
            _ => panic!("Unknown command type {}", command_type),
        }
    }
}

#[derive(Debug)]
pub struct Section {
    name: String,
    segment_name: String,
    addr: u32,
    size: u32,
    offset: u32,
    align: u32,
    relocation_offset: u32,
    number_relocations: u32,
    flags: u32,
    reserved_1: u32,
    reserved_2: u32,
}

impl Section {
    fn parse(endianness: Endianness) -> impl FnMut(parse::Input) -> parse::ParseResult<Self> {
        move |input: parse::Input| {
            let (
                input,
                (
                    name,
                    segment_name,
                    addr,
                    size,
                    offset,
                    align,
                    relocation_offset,
                    number_relocations,
                    flags,
                    reserved_1,
                    reserved_2,
                ),
            ) = context(
                "Parse Section",
                tuple((
                    map(take(16usize), |name_buf| {
                        CStr::from_bytes_until_nul(name_buf)
                            .expect("Invalid Section name")
                            .to_str()
                            .expect("Invalid Section name")
                            .to_string()
                    }),
                    map(take(16usize), |name_buf| {
                        CStr::from_bytes_until_nul(name_buf)
                            .expect("Invalid Segment name")
                            .to_str()
                            .expect("Invalid Segment name")
                            .to_string()
                    }),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    map(complete::u32(endianness), |align| 2u32.pow(align)),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                )),
            )(input)?;

            Ok((
                input,
                Self {
                    name,
                    segment_name,
                    addr,
                    size,
                    offset,
                    align,
                    relocation_offset,
                    number_relocations,
                    flags,
                    reserved_1,
                    reserved_2,
                },
            ))
        }
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Segment Name: {}", self.segment_name)?;
        writeln!(f, "Addr: {:x}", self.addr)?;
        writeln!(f, "Size: {} bytes", self.size)?;
        writeln!(f, "Offset: {:x}", self.offset)?;
        writeln!(f, "Align: {} bytes", self.align)?;
        writeln!(f, "Relocations Offset: {:x}", self.relocation_offset)?;
        writeln!(f, "Number of relocations: {}", self.number_relocations)?;
        writeln!(f, "Flags: {:x}", self.flags)?;
        writeln!(f, "Reserved 1: {:x}", self.reserved_1)?;
        writeln!(f, "Reserved 2: {:x}", self.reserved_2)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct SegmentDetails {
    name: String,
    vm_addr: u32,
    vm_size: u32,
    file_offset: u32,
    file_size: u32,
    max_protection: i32,
    initial_protection: i32,
    flags: u32,
    sections: Vec<Section>,
}

impl SegmentDetails {
    fn parse(endianness: Endianness) -> impl FnMut(parse::Input) -> parse::ParseResult<Self> {
        move |input| {
            let (
                input,
                (
                    name,
                    vm_addr,
                    vm_size,
                    file_offset,
                    file_size,
                    max_protection,
                    initial_protection,
                    number_sections,
                    flags,
                ),
            ) = context(
                "Parse Segment",
                tuple((
                    map(take(16usize), |name_buf| {
                        CStr::from_bytes_until_nul(name_buf)
                            .expect("Invalid Segment name")
                            .to_str()
                            .expect("Invalid Segment name")
                            .to_string()
                    }),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::i32(endianness),
                    complete::i32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                )),
            )(input)?;

            let (input, sections) = context(
                "Parse Segment sections",
                count(Section::parse(endianness), number_sections as usize),
            )(input)?;

            Ok((
                input,
                Self {
                    name,
                    vm_addr,
                    vm_size,
                    file_offset,
                    file_size,
                    max_protection,
                    initial_protection,
                    flags,
                    sections,
                },
            ))
        }
    }
}

impl fmt::Display for SegmentDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "VM Addr: {:x}", self.vm_addr)?;
        writeln!(f, "VM Size: {} bytes", self.vm_size)?;
        writeln!(f, "File Offset: {:x}", self.file_offset)?;
        writeln!(f, "File Size: {} bytes", self.file_size)?;
        // TODO: Print protections better!
        writeln!(f, "Max Protection: {}", self.max_protection)?;
        writeln!(f, "Initial Protection: {}", self.initial_protection)?;
        writeln!(f, "Flags: {:x}", self.flags)?;
        writeln!(f, "Number of sections: {}", self.sections.len())?;
        for (i, section) in self.sections.iter().enumerate() {
            writeln!(f, "Section {}:", i)?;
            writeln!(f, "{}", section)?;
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct Section64 {
    name: String,
    segment_name: String,
    addr: u64,
    size: u64,
    offset: u32,
    align: u32,
    relocation_offset: u32,
    number_relocations: u32,
    flags: u32,
    reserved_1: u32,
    reserved_2: u32,
    reserved_3: u32,
}

impl Section64 {
    fn parse(endianness: Endianness) -> impl FnMut(parse::Input) -> parse::ParseResult<Self> {
        move |input: parse::Input| {
            let (
                input,
                (
                    name,
                    segment_name,
                    addr,
                    size,
                    offset,
                    align,
                    relocation_offset,
                    number_relocations,
                    flags,
                    reserved_1,
                    reserved_2,
                    reserved_3,
                ),
            ) = context(
                "Parse Section64",
                tuple((
                    map(take(16usize), |name_buf| {
                        CStr::from_bytes_until_nul(name_buf)
                            .expect("Invalid Section name")
                            .to_str()
                            .expect("Invalid Section name")
                            .to_string()
                    }),
                    map(take(16usize), |name_buf| {
                        CStr::from_bytes_until_nul(name_buf)
                            .expect("Invalid Segment name")
                            .to_str()
                            .expect("Invalid Segment name")
                            .to_string()
                    }),
                    complete::u64(endianness),
                    complete::u64(endianness),
                    complete::u32(endianness),
                    map(complete::u32(endianness), |align| 2u32.pow(align)),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                )),
            )(input)?;

            Ok((
                input,
                Self {
                    name,
                    segment_name,
                    addr,
                    size,
                    offset,
                    align,
                    relocation_offset,
                    number_relocations,
                    flags,
                    reserved_1,
                    reserved_2,
                    reserved_3,
                },
            ))
        }
    }
}

impl fmt::Display for Section64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Segment Name: {}", self.segment_name)?;
        writeln!(f, "Addr: {:x}", self.addr)?;
        writeln!(f, "Size: {} bytes", self.size)?;
        writeln!(f, "Offset: {:x}", self.offset)?;
        writeln!(f, "Align: {} bytes", self.align)?;
        writeln!(f, "Relocations Offset: {:x}", self.relocation_offset)?;
        writeln!(f, "Number of relocations: {}", self.number_relocations)?;
        writeln!(f, "Flags: {:x}", self.flags)?;
        writeln!(f, "Reserved 1: {:x}", self.reserved_1)?;
        writeln!(f, "Reserved 2: {:x}", self.reserved_2)?;
        writeln!(f, "Reserved 3: {:x}", self.reserved_3)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Segment64Details {
    name: String,
    vm_addr: u64,
    vm_size: u64,
    file_offset: u64,
    file_size: u64,
    max_protection: i32,
    initial_protection: i32,
    flags: u32,
    sections: Vec<Section64>,
}

impl Segment64Details {
    fn parse(endianness: Endianness) -> impl FnMut(parse::Input) -> parse::ParseResult<Self> {
        move |input| {
            let (
                input,
                (
                    name,
                    vm_addr,
                    vm_size,
                    file_offset,
                    file_size,
                    max_protection,
                    initial_protection,
                    number_sections,
                    flags,
                ),
            ) = context(
                "Parse Segment64",
                tuple((
                    map(take(16usize), |name_buf| {
                        CStr::from_bytes_until_nul(name_buf)
                            .expect("Invalid Segment name")
                            .to_str()
                            .expect("Invalid Segment name")
                            .to_string()
                    }),
                    complete::u64(endianness),
                    complete::u64(endianness),
                    complete::u64(endianness),
                    complete::u64(endianness),
                    complete::i32(endianness),
                    complete::i32(endianness),
                    complete::u32(endianness),
                    complete::u32(endianness),
                )),
            )(input)?;

            let (input, sections) = context(
                "Parse Segment64 sections",
                count(Section64::parse(endianness), number_sections as usize),
            )(input)?;

            Ok((
                input,
                Self {
                    name,
                    vm_addr,
                    vm_size,
                    file_offset,
                    file_size,
                    max_protection,
                    initial_protection,
                    flags,
                    sections,
                },
            ))
        }
    }
}

impl fmt::Display for Segment64Details {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "VM Addr: {:x}", self.vm_addr)?;
        writeln!(f, "VM Size: {} bytes", self.vm_size)?;
        writeln!(f, "File Offset: {:x}", self.file_offset)?;
        writeln!(f, "File Size: {} bytes", self.file_size)?;
        // TODO: Print protections better!
        writeln!(f, "Max Protection: {}", self.max_protection)?;
        writeln!(f, "Initial Protection: {}", self.initial_protection)?;
        writeln!(f, "Flags: {:x}", self.flags)?;
        writeln!(f, "Number of sections: {}", self.sections.len())?;
        for (i, section) in self.sections.iter().enumerate() {
            writeln!(f, "Section {}:", i)?;
            writeln!(f, "{}", section)?;
        }
        Ok(())
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Segment(details) => {
                writeln!(f, "Segment")?;
                writeln!(f, "{}", details)
            }
            Command::SymbolTable => writeln!(f, "SymbolTable"),
            Command::SymbolSegment => writeln!(f, "SymbolSegment"),
            Command::Thread => writeln!(f, "Thread"),
            Command::UnixThread => writeln!(f, "UnixThread"),
            Command::LoadFixedVmLibrary => writeln!(f, "LoadFixedVmLibrary"),
            Command::IdentifyFixedVmLibrary => writeln!(f, "IdentifyFixedVmLibrary"),
            Command::Identify => writeln!(f, "Identify"),
            Command::IncludeFixedVmLibrary => writeln!(f, "IncludeFixedVmLibrary"),
            Command::Prepage => writeln!(f, "Prepage"),
            Command::DynamicSymbolTable => writeln!(f, "DynamicSymbolTable"),
            Command::LoadDynamicLibrary => writeln!(f, "LoadDynamicLibrary"),
            Command::IdentifyDynamicLibrary => writeln!(f, "IdentifyDynamicLibrary"),
            Command::LoadDynamicLinker => writeln!(f, "LoadDynamicLinker"),
            Command::IdentifyDynamicLinker => writeln!(f, "IdentifyDynamicLinker"),
            Command::PreboundDynamicLibrary => writeln!(f, "PreboundDynamicLibrary"),
            Command::Routines => writeln!(f, "Routines"),
            Command::SubFramework => writeln!(f, "SubFramework"),
            Command::SubUmbrella => writeln!(f, "SubUmbrella"),
            Command::SubClient => writeln!(f, "SubClient"),
            Command::SubLibrary => writeln!(f, "SubLibrary"),
            Command::TwoLevelHints => writeln!(f, "TwoLevelHints"),
            Command::PrebindChecksum => writeln!(f, "PrebindChecksum"),
            Command::LoadWeakDynamicLibrary => writeln!(f, "LoadWeakDynamicLibrary"),
            Command::Segment64(details) => {
                writeln!(f, "Segment64")?;
                write!(f, "{}", details)
            }
            Command::Routines64 => writeln!(f, "Routines64"),
            Command::Uuid => writeln!(f, "Uuid"),
            Command::RPath => writeln!(f, "RPath"),
            Command::CodeSignature => writeln!(f, "CodeSignature"),
            Command::SegmentSplitInfo => writeln!(f, "SegmentSplitInfo"),
            Command::ReexportDynamicLibrary => writeln!(f, "ReexportDynamicLibrary"),
            Command::LazyLoadDynamicLibrary => writeln!(f, "LazyLoadDynamicLibrary"),
            Command::EncryptionInfo => writeln!(f, "EncryptionInfo"),
            Command::DynamicLinkerInfo => writeln!(f, "DynamicLinkerInfo"),
            Command::DynamicLinkerInfoOnly => writeln!(f, "DynamicLinkerInfoOnly"),
            Command::LoadUpwardDynamicLibrary => writeln!(f, "LoadUpwardDynamicLibrary"),
            Command::VersionMinMacOsx => writeln!(f, "VersionMinMacOsx"),
            Command::VersionMinIphoneOs => writeln!(f, "VersionMinIphoneOs"),
            Command::FunctionStarts => writeln!(f, "FunctionStarts"),
            Command::DynamicLinkerEnvironment => writeln!(f, "DynamicLinkerEnvironment"),
            Command::Main => writeln!(f, "Main"),
            Command::DataInCode => writeln!(f, "DataInCode"),
            Command::SourceVersion => writeln!(f, "SourceVersion"),
            Command::DynamicLibraryCodeSignDrs => writeln!(f, "DynamicLibraryCodeSignDrs"),
            Command::EncryptionInfo64 => writeln!(f, "EncryptionInfo64"),
            Command::LinkerOption => writeln!(f, "LinkerOption"),
            Command::LinkerOptimizationHint => writeln!(f, "LinkerOptimizationHint"),
            Command::VersionMinTvOs => writeln!(f, "VersionMinTvOs"),
            Command::VersionMinWatchOs => writeln!(f, "VersionMinWatchOs"),
            Command::Note => writeln!(f, "Note"),
            Command::BuildVersion => writeln!(f, "BuildVersion"),
            Command::DynamicLinkerExportsTrie => writeln!(f, "DynamicLinkerExportsTrie"),
            Command::DynamicLinkerChainedFixups => writeln!(f, "DynamicLinkerChainedFixups"),
            Command::FileSetEntry => writeln!(f, "FileSetEntry"),
        }
    }
}
