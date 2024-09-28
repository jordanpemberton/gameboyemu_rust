use std::fs::read;
use std::path::Path;

use crate::cartridge::cartridge_header::CartridgeHeader;
use crate::cartridge::mbc::Mbc;

// Allows reading bootram file like a cartridge.
const MIN_CARTRIDGE_SIZE: usize = 0x2000;

pub(crate) struct Cartridge {
    #[allow(dead_code)]
    pub(crate) header: Option<CartridgeHeader>, // TODO use header
    pub(crate) mbc: Mbc,
    pub(crate) data: Vec<u8>,
}

impl Cartridge {
    pub(crate) fn new(filepath: &Path) -> Cartridge {
        let mut data = read(filepath).expect(format!("Failed to read from {:?}", filepath).as_str());
        if data.len() < MIN_CARTRIDGE_SIZE {
            let mut new_data = vec![0; MIN_CARTRIDGE_SIZE];
            new_data[..data.len()].copy_from_slice(&data);
            data = new_data;
        }

        let header_bytes = &data[..0x150];
        let header = CartridgeHeader::new(&header_bytes);

        let mbc = Mbc::new(data.as_slice());

        Cartridge {
            header,
            mbc,
            data,
        }
    }

    pub(crate) fn read_8_0000_3fff(&self, address: u16) -> u8 {
        let (rom_lower, _) = match &self.mbc {
            Mbc::None => (0, 0),
            Mbc::Mbc1 { mbc } => mbc.rom_offsets(),
            Mbc::Mbc2
            | Mbc::Mbc3
            | Mbc::Mbc5
            | _ => { panic!("UNIMPLEMENTED MBC {:?}", &self.mbc) }
        };
        self.data[(rom_lower | (address as usize & 0x3FFF)) & (self.data.len() - 1)]
    }

    pub(crate) fn read_8_4000_7fff(&self, address: u16) -> u8 {
        let (_, rom_upper) = match &self.mbc {
            Mbc::None => (0, 0x4000),
            Mbc::Mbc1 { mbc } => mbc.rom_offsets(),
            Mbc::Mbc2
            | Mbc::Mbc3
            | Mbc::Mbc5
            | _ => { panic!("UNIMPLEMENTED MBC {:?}", &self.mbc) }
        };
        self.data[(rom_upper | (address as usize & 0x3FFF)) & (self.data.len() - 1)]
    }

    pub(crate) fn read_8_A000_BFFF(&self, address: u16) -> u8 {
        match &self.mbc {
            Mbc::None => 0xFF, // ?
            Mbc::Mbc1 { mbc } if mbc.ram_enabled => self.read_8_ram(address),
            Mbc::Mbc2
            | Mbc::Mbc3
            | Mbc::Mbc5
            | _ => { panic!("UNIMPLEMENTED MBC {:?}", &self.mbc) }
        }
    }

    pub(crate) fn write_8_A000_BFFF(&mut self, address: u16, value: u8) {
        match &self.mbc {
            Mbc::None => (),
            Mbc::Mbc1 { mbc } if mbc.ram_enabled => self.write_8_ram(address, value),
            Mbc::Mbc2
            | Mbc::Mbc3
            | Mbc::Mbc5
            | _ => { panic!("UNIMPLEMENTED MBC {:?}", &self.mbc) }
        }
    }

    fn read_8_ram(&self, address: u16) -> u8 {
        let ram_offset = match &self.mbc {
            Mbc::None => 0,
            Mbc::Mbc1 { mbc } => mbc.ram_offset(),
            Mbc::Mbc2
            | Mbc::Mbc3
            | Mbc::Mbc5
            | _ => { panic!("UNIMPLEMENTED MBC {:?}", &self.mbc) }
        };
        // let adjusted_address = (ram_offset | ((address as usize & 0x1FFF) + 0x8000)) & (self.data.len() - 1);
        let adjusted_address = address as usize + ram_offset;
        self.data[adjusted_address]
    }

    fn write_8_ram(&mut self, address: u16, value: u8) {
        let ram_offset = match &self.mbc {
            Mbc::None => 0,
            Mbc::Mbc1 { mbc } => mbc.ram_offset(),
            Mbc::Mbc2
            | Mbc::Mbc3
            | Mbc::Mbc5
            | _ => { panic!("UNIMPLEMENTED MBC {:?}", &self.mbc) }
        };
        // let adjusted_address = (ram_offset | ((address as usize & 0x1FFF) + 0x8000)) & (self.data.len() - 1);
        let adjusted_address = address as usize + ram_offset;
        self.data[adjusted_address] = value;
    }
}
