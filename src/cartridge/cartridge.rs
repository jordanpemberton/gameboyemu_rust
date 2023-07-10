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
        self.data[(rom_lower | (address as usize & 0x3FFF)) & (self.data.len() - 1)]
    }

    pub(crate) fn read_8_4000_7fff(&self, address: u16) -> u8 {
        let (_, rom_upper) = match &self.mbc {
            Mbc::Mbc1 { mbc } => mbc.rom_offsets(),
            _ => (0, 0x4000)
        };
        self.data[(rom_upper | (address as usize & 0x3FFF)) & (self.data.len() - 1)]
    }

    pub(crate) fn read_8_a000_bfff(&self, address: u16) -> u8 {
        match &self.mbc {
            Mbc::Mbc1 { mbc } if mbc.ram_enabled => self.read_8_ram(address),
            _ => 0xFF
        }
    }

    pub(crate) fn write_8_a000_bfff(&mut self, address: u16, value: u8) {
        match &self.mbc {
            Mbc::Mbc1 { mbc } if mbc.ram_enabled => self.write_8_ram(address, value),
            _ => ()
        }
    }

    fn read_8_ram(&self, address: u16) -> u8 {
        let ram_offset = match &self.mbc {
            Mbc::Mbc1 { mbc } => mbc.ram_offset(),
            _ => 0
        };
        let adjusted_address = (ram_offset | ((address as usize & 0x1FFF) + 0x8000)) & (self.data.len() - 1);
            // (ram_offset as usize + address as usize) & (self.data.len() - 1);
        self.data[adjusted_address]
    }

    fn write_8_ram(&mut self, address: u16, value: u8) {
        let ram_offset = match &self.mbc {
            Mbc::Mbc1 { mbc } => mbc.ram_offset(),
            _ => 0
        };
        let adjusted_address = (ram_offset | ((address as usize & 0x1FFF) + 0x8000)) & (self.data.len() - 1);
            // ((ram_offset as usize) + (address as usize)) & (self.data.len() - 1);
        self.data[adjusted_address] = value;
    }
}
