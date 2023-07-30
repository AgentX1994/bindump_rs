use std::fmt;

#[derive(Debug)]
pub struct Elf {}

impl Elf {
    pub(crate) fn load(data: Vec<u8>) -> Self {
        todo!("Loading of ELF files is not yet implemented!")
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
