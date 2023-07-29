use std::ffi::{CStr, CString};

use super::machine::Endianness;
use super::utils::{read_i32, read_u32, read_u64};

#[derive(Debug)]
pub struct LoadCommand {
    pub size: u32,
    // TODO: Load commands
    pub command: Command,
}

impl LoadCommand {
    pub(super) fn load(data: &[u8], current_offset: usize, endianness: Endianness) -> Self {
        let command_type = read_u32(&data[current_offset..][..4], Endianness::Little);
        let size = read_u32(&data[current_offset + 4..][..4], Endianness::Little);
        let command_data = &data[current_offset + 8..][..size as usize];
        LoadCommand {
            size,
            command: Command::load(command_type, command_data, endianness),
        }
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
    fn load(command_type: u32, command_data: &[u8], endianness: Endianness) -> Self {
        match command_type {
            1 => Self::Segment(SegmentDetails::load(command_data, endianness)),
            2 => Self::SymbolTable,
            3 => Self::SymbolSegment,
            4 => Self::Thread,
            5 => Self::UnixThread,
            6 => Self::LoadFixedVmLibrary,
            7 => Self::IdentifyFixedVmLibrary,
            8 => Self::Identify,
            9 => Self::IncludeFixedVmLibrary,
            10 => Self::Prepage,
            11 => Self::DynamicSymbolTable,
            12 => Self::LoadDynamicLibrary,
            13 => Self::IdentifyDynamicLibrary,
            14 => Self::LoadDynamicLinker,
            15 => Self::IdentifyDynamicLinker,
            16 => Self::PreboundDynamicLibrary,
            17 => Self::Routines,
            18 => Self::SubFramework,
            19 => Self::SubUmbrella,
            20 => Self::SubClient,
            21 => Self::SubLibrary,
            22 => Self::TwoLevelHints,
            23 => Self::PrebindChecksum,
            /* 24 */ 0x80000018 => Self::LoadWeakDynamicLibrary,
            25 => Self::Segment64(Segment64Details::load(command_data, endianness)),
            26 => Self::Routines64,
            27 => Self::Uuid,
            /* 28 */ 0x8000001c => Self::RPath,
            29 => Self::CodeSignature,
            30 => Self::SegmentSplitInfo,
            /* 31 */ 0x8000001f => Self::ReexportDynamicLibrary,
            32 => Self::LazyLoadDynamicLibrary,
            33 => Self::EncryptionInfo,
            34 => Self::DynamicLinkerInfo,
            /* 34 */ 0x80000022 => Self::DynamicLinkerInfoOnly,
            /* 35 */ 0x80000023 => Self::LoadUpwardDynamicLibrary,
            36 => Self::VersionMinMacOsx,
            37 => Self::VersionMinIphoneOs,
            38 => Self::FunctionStarts,
            39 => Self::DynamicLinkerEnvironment,
            /* 40 */ 0x80000028 => Self::Main,
            41 => Self::DataInCode,
            42 => Self::SourceVersion,
            43 => Command::DynamicLibraryCodeSignDrs,
            44 => Self::EncryptionInfo64,
            45 => Self::LinkerOption,
            46 => Self::LinkerOptimizationHint,
            47 => Self::VersionMinTvOs,
            48 => Self::VersionMinWatchOs,
            49 => Self::Note,
            50 => Self::BuildVersion,
            /* 51 */ 0x80000033 => Self::DynamicLinkerExportsTrie,
            /* 52 */ 0x80000034 => Self::DynamicLinkerChainedFixups,
            /* 53 */ 0x80000035 => Self::FileSetEntry,
            _ => panic!("Unknown command type {}", command_type),
        }
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
    number_sections: u32,
    flags: u32,
}

impl SegmentDetails {
    fn load(command_data: &[u8], endianness: Endianness) -> Self {
        let name_buf = &command_data[..16];
        let name_cstr = CStr::from_bytes_until_nul(name_buf);
        let name = match name_cstr {
            Ok(name_cstr) => name_cstr.to_string_lossy().to_string(),
            Err(_) => CString::new(name_buf)
                .expect("Invalid Segment Name")
                .to_string_lossy()
                .to_string(),
        };
        let vm_addr = read_u32(&command_data[16..][..4], endianness);
        let vm_size = read_u32(&command_data[20..][..4], endianness);
        let file_offset = read_u32(&command_data[24..][..4], endianness);
        let file_size = read_u32(&command_data[28..][..4], endianness);
        let max_protection = read_i32(&command_data[32..][..4], endianness);
        let initial_protection = read_i32(&command_data[36..][..4], endianness);
        let number_sections = read_u32(&command_data[40..][..4], endianness);
        let flags = read_u32(&command_data[44..][..4], endianness);

        Self {
            name,
            vm_addr,
            vm_size,
            file_offset,
            file_size,
            max_protection,
            initial_protection,
            number_sections,
            flags,
        }
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
    number_sections: u32,
    flags: u32,
}

impl Segment64Details {
    fn load(command_data: &[u8], endianness: Endianness) -> Self {
        let name_buf = &command_data[..16];
        let name_cstr = CStr::from_bytes_until_nul(name_buf);
        let name = match name_cstr {
            Ok(name_cstr) => name_cstr.to_string_lossy().to_string(),
            Err(_) => CString::new(name_buf)
                .expect("Invalid Segment Name")
                .to_string_lossy()
                .to_string(),
        };
        let vm_addr = read_u64(&command_data[16..][..8], endianness);
        let vm_size = read_u64(&command_data[24..][..8], endianness);
        let file_offset = read_u64(&command_data[32..][..8], endianness);
        let file_size = read_u64(&command_data[40..][..8], endianness);
        let max_protection = read_i32(&command_data[48..][..4], endianness);
        let initial_protection = read_i32(&command_data[52..][..4], endianness);
        let number_sections = read_u32(&command_data[56..][..4], endianness);
        let flags = read_u32(&command_data[60..][..4], endianness);

        Self {
            name,
            vm_addr,
            vm_size,
            file_offset,
            file_size,
            max_protection,
            initial_protection,
            number_sections,
            flags,
        }
    }
}
