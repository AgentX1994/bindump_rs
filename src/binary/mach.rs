pub mod load_commands;
pub mod machine;
mod utils;

use load_commands::LoadCommand;
use machine::{CpuType, Endianness};
use utils::{read_i32, read_u32};

#[derive(Debug)]
pub enum Mach {
    Universal(u32, Vec<MachArch>),
    MachO(MachODetails),
}

// From loader.h
#[derive(Debug)]
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

impl TryFrom<u32> for FileType {
    // TODO: Implement a proper error type
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Object),
            2 => Ok(Self::Executable),
            3 => Ok(Self::FixedVmLibrary),
            4 => Ok(Self::Core),
            5 => Ok(Self::Preload),
            6 => Ok(Self::DynamicLibrary),
            7 => Ok(Self::DynamicLinkEditor),
            8 => Ok(Self::Bundle),
            9 => Ok(Self::DynamicLibraryStub),
            10 => Ok(Self::DebugSymbols),
            11 => Ok(Self::Kexts),
            12 => Ok(Self::Fileset),
            13 => Ok(Self::GpuProgram),
            14 => Ok(Self::GpuDynamicLibrary),
            _ => Err(value),
        }
    }
}

#[derive(Debug)]
pub enum MachArch {
    Arch32(MachArchDetails),
    // TODO: 64 bit
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
    fn load(data: &[u8], current_position: usize) -> Self {
        let header_data = &data[current_position..][..20];
        let cpu_type_i32 = read_i32(&header_data[0..4], Endianness::Big);
        // TODO: Error handling
        let cpu_type = CpuType::try_from(cpu_type_i32).expect("Unknown CPU Type!");
        let cpu_subtype = read_i32(&header_data[4..8], Endianness::Big);
        let offset = read_u32(&header_data[8..12], Endianness::Big);
        let size = read_u32(&header_data[12..16], Endianness::Big);
        let align = read_u32(&header_data[16..20], Endianness::Big);
        let align = 2u32.pow(align);
        let mach_object = Mach::load(&data[offset as usize..][..size as usize]);
        Self {
            cpu_type,
            cpu_subtype,
            offset,
            size,
            align,
            mach_object,
        }
    }
}

#[derive(Debug)]
pub struct MachODetails {
    header: MachHeader,
    load_commands: Vec<LoadCommand>,
}

#[derive(Debug)]
pub struct MachHeader {
    magic: u32,
    cpu_type: CpuType,
    cpu_subtype: i32,
    file_type: FileType,
    number_of_load_commands: u32,
    total_command_size: u32,
    flags: u32,
    reserved: u32,
}

impl MachHeader {
    fn load(data: &[u8], endianness: Endianness) -> Self {
        let header_data = &data[..32];
        // TODO: Error handling
        let magic = read_u32(&header_data[0..4], endianness);
        let cpu_type_i32 = read_i32(&header_data[4..8], endianness);
        let cpu_type = CpuType::try_from(cpu_type_i32).expect("Unknown CPU Type!");
        let cpu_subtype = read_i32(&header_data[8..12], endianness);
        let file_type = read_u32(&header_data[12..16], endianness);
        let file_type = FileType::try_from(file_type).expect("Unknown file type!");
        let number_of_load_commands = read_u32(&header_data[16..20], endianness);
        let total_command_size = read_u32(&header_data[20..24], endianness);
        let flags = read_u32(&header_data[24..28], endianness);
        let reserved = read_u32(&header_data[28..32], endianness);
        Self {
            magic,
            cpu_type,
            cpu_subtype,
            file_type,
            number_of_load_commands,
            total_command_size,
            flags,
            reserved,
        }
    }
}

impl Mach {
    pub(crate) fn load(data: &[u8]) -> Self {
        // TODO: we are currently assume all binaries are "universal"
        let magic = u32::from_be_bytes(data[0..4].try_into().unwrap());
        println!("Magic = {:#x}", magic);
        match magic {
            0xcafebabe => Self::load_universal_32_bit_little_endian(magic, data),
            0xfeedface => todo!(),
            0xfeedfacf => todo!(),
            0xcffaedfe => Self::load_64_bit_little_endian(magic, data),
            0xcefaedfe => todo!(),
            _ => panic!("Unknown magic: {:x}", magic),
        }
    }

    fn load_universal_32_bit_little_endian(magic: u32, data: &[u8]) -> Self {
        assert_eq!(magic, 0xcafebabe);
        let num_arches = u32::from_be_bytes(data[4..8].try_into().unwrap());
        let arches = (0..num_arches as usize)
            .map(|i| MachArch::Arch32(MachArchDetails::load(data, 8 + (i * 20))))
            .collect();
        Self::Universal(magic, arches)
    }

    fn load_64_bit_little_endian(magic: u32, data: &[u8]) -> Self {
        assert_eq!(magic, 0xcffaedfe);
        let header = MachHeader::load(data, Endianness::Little);
        let commands_start = 32usize;
        let commands_end = commands_start + header.total_command_size as usize;
        let mut current_offset = commands_start;
        let mut load_commands = vec![];
        for _ in 0..header.number_of_load_commands {
            let load_command = LoadCommand::load(data, current_offset);
            let command_size = load_command.size;
            load_commands.push(load_command);
            current_offset += command_size as usize;
        }
        if current_offset != commands_end {
            println!(
                "Warning: size of load commands doesn't match value in header! Corrupted binary?"
            );
        }
        Mach::MachO(MachODetails {
            header,
            load_commands,
        })
    }
}
