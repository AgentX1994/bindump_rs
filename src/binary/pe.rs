use std::fmt;

#[derive(Debug)]
pub struct Pe {}

impl Pe {
    pub(crate) fn load(data: Vec<u8>) -> Self {
        todo!("Loading of PE files is not yet implemented!")
    }
}

impl fmt::Display for Pe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
