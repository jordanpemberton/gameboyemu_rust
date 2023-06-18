/*
    BOOT_ROM        0x0000..=0x0100
    CARTRIDGE_ROM   0x0000..=0x7FFF
    VRAM            0x8000..=0x9FFF
    EXTERNAL_RAM    0xA000..=0xBFFF
    INTERNAL_RAM    0xC000..=0xDFFF
    INVALID         0xE000..=0xFDFF
    EXTERNAL_RAM    0xA000..=0xBFFF
    OAM_RAM         0xFE00..=0xFF9F
    RAM,            0xFEA0..=0xFEFF
    IO              0xFF00..=0xFF79
    HRAM            0xFF80..=0xFFFF

    CARTRIDGE_ROM_BANK_SIZE: u16 = 0x4000;
*/

use std::cmp::{max, min};
use std::fs::read;
use crate::cartridge::cartridge::Cartridge;
use crate::cartridge::mbc::Mbc;
use crate::console::timer::DIV_REG_ADDRESS;

const BOOTROM_FILEPATH: &str = "src/console/DMG_ROM.bin";

#[allow(dead_code)]
#[derive(PartialEq)]
pub(crate) enum Endianness {
    BIG,
    LITTLE,
}

pub(crate) struct Mmu {
    pub(crate) is_booting: bool,
    rom: [u8; 0x8000 as usize],
    ram: [u8; 0x8000 as usize],
    cartridge: Option<Cartridge>,
}

// TODO -- refactor, eliminate duplicated code.
impl Mmu {
    pub(crate) fn new(cartridge: Option<Cartridge>) -> Mmu {
        let mut mmu = Mmu {
            is_booting: true,
            rom: [0; 0x8000],
            ram: [0; 0x8000],
            cartridge,
        };
        mmu.load_bootrom();
        mmu
    }

    pub(crate) fn read_8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF if self.is_booting => {
                self.rom[address as usize]
            }

            0x0000..=0x3FFF => if let Some(cartridge) = &self.cartridge {
                cartridge.read_8_0000_3fff(address)
            } else {
                self.rom[address as usize]
            }

            0x4000..=0x7FFF => if let Some(cartridge) = &self.cartridge {
                cartridge.read_8_4000_7fff(address)
            } else {
                self.rom[address as usize]
            }

            _ => {
                let mut result = self.ram[address as usize - 0x8000];

                if address == 0xFF0F {
                    result |= 0xE0; // top 3 bits of IF register will always return 1s.
                }

                result
            }
        }
    }

    pub(crate) fn read_16(&self, address: u16, endian: Endianness) -> u16 {
        let lsb: u8 = self.read_8(address);
        let msb: u8 = self.read_8(address + 1);

        match endian {
            Endianness::BIG => (msb as u16) << 8 | lsb as u16,
            Endianness::LITTLE => (lsb as u16) << 8 | msb as u16,
        }
    }

    pub(crate) fn read_buffer(&self, start: usize, end: usize) -> Vec<u8> {
        let mut t = min(end, 0x10000);
        let mut result = vec![0; t - start];

        match start {
            0x0000..=0x3FFF => {
                if start < 0x0100 && self.is_booting {
                    t =  min(end, 0x0100);
                    result[0..t - start].clone_from_slice(&self.rom[start..t]);

                    if end > 0x0100 {
                        t = min(end, 0x4000);
                        if let Some(cartridge) = &self.cartridge {
                            let data = cartridge.read_buffer_0000_3fff(0x0100 as u16,  t as u16);
                            result[0x0100..t].clone_from_slice(&data[..]);
                        } else {
                            result[0x100..t].clone_from_slice(&self.rom[0x0100..t]);
                        }
                    }
                } else {
                    t = min(end, 0x4000);
                    if let Some(cartridge) = &self.cartridge {
                        let data = cartridge.read_buffer_0000_3fff(start as u16, t as u16);
                        result[0..t - start].clone_from_slice(&data[..]);
                    } else {
                        result[0..t - start].clone_from_slice(&self.rom[start..t]);
                    }
                }
            }

            // TODO banks!
            0x4000..=0x7FFF => {
                t = min(end, 0x8000);

                if let Some(cartridge) = &self.cartridge {
                    let data = cartridge.read_buffer_4000_7fff(start as u16, t as u16);
                    result[0..t - start].clone_from_slice(&data[..]);
                } else {
                    result[0..t - start].clone_from_slice(&self.rom[start..t]);
                }
            }

            // TODO banks
            _ => {
                t = min(end, 0x10000);
                result[0..t - start].clone_from_slice(&self.ram[start - 0x8000..t - 0x8000]);

                if start >= 0xFF0F && t < 0xFF0F {
                    result[0xFF0F - 0x8000] |= 0xE0; // top 3 bits of IF register will always return 1s.
                }
            }
        }

        result.to_vec()
    }

    //noinspection RsNonExhaustiveMatch -- u16 range covered
    pub(crate) fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::Mbc1 { mbc } => {
                            mbc.ram_enabled = (value & 0b1111) == 0b1010;
                        }
                        _ => { }
                    }
                } else { }
            }

            0x2000..=0x3FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::Mbc1 { mbc } => {
                            mbc.bank1 = max(value & 0b0001_1111, 1);
                        }
                        _ => { }
                    }
                } else { }
            }

            0x4000..=0x5FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::Mbc1 { mbc } => {
                            mbc.bank2 = value & 0b0011;
                        }
                        _ => { }
                    }
                } else {
                    self.rom[address as usize] = value;
                }
            }

            0x6000..=0x7FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::Mbc1 { mbc } => {
                            mbc.advram_banking_mode = (value & 1) == 1;
                        }
                        _ => { }
                    }
                } else {
                    self.rom[address as usize] = value;
                }
            }

            // TODO only write to wrtie-able RAM
            0x8000..=0xFFFF => {
                let ram_offset = if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::Mbc1 { mbc } => {
                            mbc.ram_offset()
                        }
                        _ => 0
                    }
                } else {
                    0
                };

                let adj_address = ram_offset | ((address as usize - 0x8000) & (self.ram.len() - 1));
                // let adj_address = self.ram_offset | (addr as usize & 0x1fff)) & (self.ram.len() - 1); // How is this mask 0x1FFF supposed to work?

                self.ram[adj_address] = if address == DIV_REG_ADDRESS { // All writes to timer DIV register reset it to 0
                    0
                } else {
                    value
                };

                if self.is_booting && address == 0xFF50 && (value & 1) == 1 {
                    self.is_booting = false;
                }
            }
        }
    }

    pub(crate) fn write_16(&mut self, address: u16, value: u16, endian: Endianness) {
        let mut a = ((value & 0xFF00) >> 8) as u8;
        let mut b = (value & 0x00FF) as u8;

        if endian == Endianness::BIG {
            let temp = a;
            a = b;
            b = temp;
        }

        self.write_8(address, a);
        self.write_8(address + 1, b);
    }

    fn load_bootrom(&mut self) {
        let bootrom = read(BOOTROM_FILEPATH).unwrap();
        let size = min(bootrom.len(), 0x100);
        self.rom[0..size].clone_from_slice(&bootrom[..size]);
    }
}
