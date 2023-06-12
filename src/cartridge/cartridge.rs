use std::cmp::min;
use std::fs::read;
use std::path::Path;

use crate::cartridge::cartridge_header::CartridgeHeader;
use crate::cartridge::mbc::Mbc;

#[allow(dead_code)]
pub(crate) struct Cartridge {
    pub(crate) header: Option<CartridgeHeader>,
    pub(crate) mbc: Mbc,
    pub(crate) data: Vec<u8>,
}

impl Cartridge {
    pub(crate) fn new(filepath: &Path) -> Cartridge {
        let data = read(filepath).expect(format!("Failed to read from {:?}", filepath).as_str());
        let header = CartridgeHeader::new(&data[..min(0x150, data.len())]);
        let mbc = Mbc::new(data.as_slice());

        Cartridge {
            header,
            mbc,
            data,
        }
    }

    pub(crate) fn read_8_0000_3fff(&self, address: u16) -> u8 {
        let (rom_lower, _) = match &self.mbc {
            Mbc::Mbc1 { mbc } => mbc.rom_offsets(),
            _ => (0, 0)
        };
        self.data[(rom_lower | (address as usize & 0x3FFF))]
    }

    pub(crate) fn read_buffer_0000_3fff(&self, start: u16, end: u16) -> Vec<u8> {
        if end <= start { return vec![]; }

        let (rom_lower, _) = match &self.mbc {
            Mbc::Mbc1 { mbc } => mbc.rom_offsets(),
            _ => (0, 0)
        };

        self.data[(rom_lower | (start as usize & 0x3FFF))..(rom_lower | (end as usize & 0x3FFF))].to_vec()
    }

    pub(crate) fn read_8_4000_7fff(&self, address: u16) -> u8 {
        let (_, rom_upper) = match &self.mbc {
            Mbc::Mbc1 { mbc } => mbc.rom_offsets(),
            _ => (0, 0x4000) // ?
        };
        self.data[(rom_upper | (address as usize & 0x3FFF))]
    }

    pub(crate) fn read_buffer_4000_7fff(&self, start: u16, end: u16) -> Vec<u8> {
        if end <= start { return vec![]; }

        let (_, rom_upper) = match &self.mbc {
            Mbc::Mbc1 { mbc } => mbc.rom_offsets(),
            _ => (0, 0)
        };

        self.data[(rom_upper | (start as usize & 0x3FFF))..(rom_upper | (end as usize & 0x3FFF))].to_vec()
    }
}
