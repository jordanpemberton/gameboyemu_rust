use std::fs::read;
use std::path::Path;

use crate::cartridge::cartridge_header::CartridgeHeader;
use crate::cartridge::mbc::Mbc;

pub(crate) enum CartridgeOption {
    NONE,
    SOME(Cartridge),
}

#[allow(dead_code)]
pub(crate) struct Cartridge {
    pub(crate) header: CartridgeHeader,
    pub(crate) mbc: Mbc,
    pub(crate) data: Vec<u8>,
}

impl Cartridge {
    pub(crate) fn new(filepath: &Path) -> Cartridge {
        let data = read(filepath).expect(format!("Failed to read from {:?}", filepath).as_str());
        let header = CartridgeHeader::new(&data[..0x150]);
        let mbc = Mbc::new(data.as_slice());

        Cartridge {
            header,
            mbc,
            data,
        }
    }
}
