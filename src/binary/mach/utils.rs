use super::machine::Endianness;

pub(super) fn read_u32(bytes: &[u8], endianness: Endianness) -> u32 {
    assert_eq!(bytes.len(), 4);
    let bytes = bytes.try_into().unwrap();
    match endianness {
        Endianness::Little => u32::from_le_bytes(bytes),
        Endianness::Big => u32::from_be_bytes(bytes),
    }
}

pub(super) fn read_i32(bytes: &[u8], endianness: Endianness) -> i32 {
    assert_eq!(bytes.len(), 4);
    let bytes = bytes.try_into().unwrap();
    match endianness {
        Endianness::Little => i32::from_le_bytes(bytes),
        Endianness::Big => i32::from_be_bytes(bytes),
    }
}
