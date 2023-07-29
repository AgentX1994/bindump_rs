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
        let command_type = read_u32(&data[current_offset..][..4], endianness);
        let size = read_u32(&data[current_offset + 4..][..4], endianness);
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
    fn size() -> usize {
        68
    }
    fn load(data: &[u8], endianness: Endianness) -> Self {
        let name_buf = &data[..16];
        let name_cstr = CStr::from_bytes_until_nul(name_buf);
        let name = match name_cstr {
            Ok(name_cstr) => name_cstr.to_string_lossy().to_string(),
            Err(_) => CString::new(name_buf)
                .expect("Invalid Section Name")
                .to_string_lossy()
                .to_string(),
        };
        let segment_name_buf = &data[16..][..16];
        let segment_name_cstr = CStr::from_bytes_until_nul(segment_name_buf);
        let segment_name = match segment_name_cstr {
            Ok(segment_name_cstr) => segment_name_cstr.to_string_lossy().to_string(),
            Err(_) => CString::new(segment_name_buf)
                .expect("Invalid Segment Name")
                .to_string_lossy()
                .to_string(),
        };
        let addr = read_u32(&data[32..][..4], endianness);
        let size = read_u32(&data[36..][..4], endianness);
        let offset = read_u32(&data[40..][..4], endianness);
        let align = read_u32(&data[44..][..4], endianness);
        let align = 2u32.pow(align);
        let relocation_offset = read_u32(&data[38..][..4], endianness);
        let number_relocations = read_u32(&data[42..][..4], endianness);
        let flags = read_u32(&data[46..][..4], endianness);
        let reserved_1 = read_u32(&data[50..][..4], endianness);
        let reserved_2 = read_u32(&data[54..][..4], endianness);

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
    flags: u32,
    sections: Vec<Section>,
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

        let mut sections = Vec::with_capacity(number_sections as usize);
        let mut current_offset = 48usize;
        for _ in 0..number_sections {
            let section = Section::load(&command_data[current_offset..], endianness);
            sections.push(section);
            current_offset += Section::size();
        }

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
        }
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
    fn size() -> usize {
        80
    }
    fn load(data: &[u8], endianness: Endianness) -> Self {
        let name_buf = &data[..16];
        let name_cstr = CStr::from_bytes_until_nul(name_buf);
        let name = match name_cstr {
            Ok(name_cstr) => name_cstr.to_string_lossy().to_string(),
            Err(_) => CString::new(name_buf)
                .expect("Invalid Section Name")
                .to_string_lossy()
                .to_string(),
        };
        let segment_name_buf = &data[16..][..16];
        let segment_name_cstr = CStr::from_bytes_until_nul(segment_name_buf);
        let segment_name = match segment_name_cstr {
            Ok(segment_name_cstr) => segment_name_cstr.to_string_lossy().to_string(),
            Err(_) => CString::new(segment_name_buf)
                .expect("Invalid Segment Name")
                .to_string_lossy()
                .to_string(),
        };
        let addr = read_u64(&data[32..][..8], endianness);
        let size = read_u64(&data[40..][..8], endianness);
        let offset = read_u32(&data[48..][..4], endianness);
        let align = read_u32(&data[52..][..4], endianness);
        let align = 2u32.pow(align);
        let relocation_offset = read_u32(&data[56..][..4], endianness);
        let number_relocations = read_u32(&data[60..][..4], endianness);
        let flags = read_u32(&data[64..][..4], endianness);
        let reserved_1 = read_u32(&data[68..][..4], endianness);
        let reserved_2 = read_u32(&data[72..][..4], endianness);
        let reserved_3 = read_u32(&data[76..][..4], endianness);

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
    flags: u32,
    sections: Vec<Section64>,
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

        let mut sections = Vec::with_capacity(number_sections as usize);
        let mut current_offset = 64usize;
        for _ in 0..number_sections {
            let section = Section64::load(&command_data[current_offset..], endianness);
            sections.push(section);
            current_offset += Section64::size();
        }

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
        }
    }
}
