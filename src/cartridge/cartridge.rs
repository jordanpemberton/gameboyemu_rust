use std::fs::read;
use std::path::Path;

pub(crate) struct Cartridge {
    pub(crate) data: Vec<u8>,
}

impl Cartridge {
    pub(crate) fn new(filepath: &Path) -> Cartridge {
        Cartridge {
            data: read(filepath).unwrap(),
        }
    }
}
