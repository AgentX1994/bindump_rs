use super::machine::Endianness;
use super::utils::read_u32;

#[derive(Debug)]
pub struct LoadCommand {
    pub size: u32,
    // TODO: Load commands
    pub command: Command,
}

impl LoadCommand {
    pub(super) fn load(data: &[u8], current_offset: usize) -> Self {
        let command_type = read_u32(&data[current_offset..][..4], Endianness::Little);
        let size = read_u32(&data[current_offset + 4..][..4], Endianness::Little);
        let command_data = &data[current_offset..][..size as usize];
        LoadCommand {
            size,
            command: Command::load(command_type, command_data),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Segment,
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
    Segment64,
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
    fn load(command_type: u32, command_data: &[u8]) -> Self {
        match command_type {
            1 => Self::Segment,
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
            25 => Self::Segment64,
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
