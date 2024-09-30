use std::cmp::max;
use std::fs::read;
use std::path::Path;

use crate::cartridge::cartridge_header::CartridgeHeader;
use crate::cartridge::mbc::Mbc;

pub const ROM_BANK_SIZE: usize = 0x4000;
pub const RAM_BANK_SIZE: usize = 0x2000;
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

    // ROM: 0x0000..=0x7FFF
    pub(crate) fn read_8_rom(&mut self, address: u16) -> u8 {
        let result = match &mut self.mbc {
            Mbc::None => {
                self.data[address as usize]
            }
            Mbc::Mbc1 { mbc: mbc1 } => {
                let (rom_lower, rom_upper) = mbc1.rom_offsets();
                match address {
                    0x0000..=0x3FFF => {
                        let adjusted_address = (address as usize & 0x3FFF) | rom_lower;
                        self.data[adjusted_address]
                    },
                    0x4000..=0x7FFF => {
                        let adjusted_address = (address as usize & 0x3FFF) | rom_upper;
                        self.data[adjusted_address]
                    },
                    _ => panic!("This should be unreachable")
                }
            }
            Mbc::Mbc3 { mbc: _ }
            | Mbc::Mbc2
            | Mbc::Mbc5
            | Mbc::Huc1 => panic!("UNIMPLEMENTED: Unsupported cartridge type {:?}, cannot write address {:04X}.", &self.mbc, address),
        };

        result
    }

    pub(crate) fn write_8_rom(&mut self, address: u16, value: u8) {
        match &mut self.mbc {
            Mbc::None => {
                self.data[address as usize] = value;
            }
            Mbc::Mbc1 { mbc: mbc1 } => {
                match address {
                    0x0000..=0x1FFF => { mbc1.ram_enabled = (value & 0x0F) == 0x0A; },
                    0x2000..=0x3FFF => { mbc1.bank1 = max(value & 0x1F, 1); },
                    0x4000..=0x5FFF => { mbc1.bank2 = value & 0x03; },
                    0x6000..=0x7FFF => { mbc1.advram_banking_mode = (value & 1) == 1; },
                    _ => panic!("this should be unreachable")
                }
            }
            Mbc::Mbc3 { mbc: mbc3 } => {
                match address {
                    0x0000..=0x1FFF => { mbc3.map_en = (value & 0x0F) == 0x0A; },
                    0x2000..=0x3FFF => {
                        mbc3.rom_bank = max(value, 1);
                        mbc3.rom_offsets = (0, ROM_BANK_SIZE * mbc3.rom_bank as usize);
                    },
                    0x4000..=0x5FFF => {
                        mbc3.map_select = value & 0x0F;
                        if mbc3.mbc30 {
                            mbc3.ram_offset = RAM_BANK_SIZE * (mbc3.map_select as usize);
                        } else {
                            mbc3.ram_offset = RAM_BANK_SIZE * (mbc3.map_select as usize);
                        }
                    },
                    0x6000..=0x7FFF => {
                        // DO NOTHING
                    },
                    _ => panic!("this should be unreachable")
                }
            }
            Mbc::Mbc2
            | Mbc::Mbc5
            | Mbc::Huc1 => panic!("UNIMPLEMENTED: Unsupported cartridge type {:?}, cannot write address {:04X}.", &self.mbc, address),
        }
    }

    pub(crate) fn read_8_ram(&mut self, address: u16) -> u8 {
        let result = match &mut self.mbc {
            Mbc::None => {
                self.data[address as usize]
            }
            Mbc::Mbc1 { mbc: mbc1 } => {
                match address {
                    0xA000..=0xBFFF => {
                        // The RAM is only accessible if RAM is enabled
                        if mbc1.ram_enabled {
                            let adjusted_address = (address as usize & 0x1FFF) | mbc1.ram_offset();
                            self.data[adjusted_address]
                        }
                        // Otherwise reads return open bus values (often $FF, but not guaranteed)
                        else {
                            0xFF
                        }
                    },
                    _ => panic!("This should be unreachable")
                }
            }
            Mbc::Mbc2
            | Mbc::Mbc3 { mbc: _ }
            | Mbc::Mbc5
            | Mbc::Huc1 => panic!("UNIMPLEMENTED: Unsupported cartridge type {:?}, cannot write address {:04X}.", &self.mbc, address),
        };

        result
    }

    // RAM: 0x8000+
    pub(crate) fn write_8_ram(&mut self, address: u16, value: u8) {
        match &mut self.mbc {
            Mbc::None => {
                self.data[address as usize] = value;
            }
            Mbc::Mbc1 { mbc: mbc1 } => {
                match address {
                    // Writes are ignored if RAM is not enabled.
                    0xA000..=0xBFFF if mbc1.ram_enabled => {
                        let adjusted_address = (address as usize & 0x1FFF) | mbc1.ram_offset();
                        self.data[adjusted_address] = value;
                    },
                    _ => panic!("This should be unreachable")
                }
            }
            Mbc::Mbc2
            | Mbc::Mbc3 { mbc: _ }
            | Mbc::Mbc5
            | Mbc::Huc1 => panic!("UNIMPLEMENTED: Unsupported cartridge type {:?}, cannot write address {:04X}.", &self.mbc, address),
        }
    }
}
