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

const BOOTROM_FILEPATH: &str = "roms/bootrom/DMG_ROM.bin";

const IF_REG_ADDRESS: u16 = 0xFF0F;
const OAM_DMA_ADDRESS: u16 = 0xFF46;
const BANK_REG_ADDRESS: u16 = 0xFF50;

#[allow(dead_code)]
#[derive(PartialEq)]
pub(crate) enum Endianness {
    BIG,
    LITTLE,
}

pub(crate) struct Mmu {
    pub(crate) is_booting: bool,
    pub(crate) oam_dma_source_address: Option<u16>,
    rom: [u8; 0x8000 as usize],
    ram: [u8; 0x8000 as usize],
    cartridge: Option<Cartridge>,
}

// TODO -- refactor, eliminate duplicated code.
impl Mmu {
    pub(crate) fn new(cartridge: Option<Cartridge>) -> Mmu {
        let mut mmu = Mmu {
            is_booting: true,
            oam_dma_source_address: None,
            rom: [0; 0x8000],
            ram: [0; 0x8000],
            cartridge,
        };
        mmu.load_bootrom();
        mmu
    }

    // noinspection RsNonExhaustiveMatch -- u16 range covered
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

            0x8000..=0x9FFF => {
                let result = self.ram[address as usize - 0x8000];
                result
            }

            0xA000..=0xBFFF => {
                let result = if let Some(cartridge) = &self.cartridge {
                    cartridge.read_8_a000_bfff(address)
                } else {
                    self.ram[address as usize - 0x8000]
                };
                result
            }

            0xC000..=0xFFFF => {
                let mut result = self.ram[address as usize - 0x8000];
                if address == IF_REG_ADDRESS {
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

    pub(crate) fn read_buffer(&self, start: u16, end: u16) -> Vec<u8> {
        let mut result = vec![0; end as usize - start as usize];
        for address in start..end {
            result[address as usize - start as usize] = self.read_8(address);
        }
        result
    }

    // noinspection RsNonExhaustiveMatch -- u16 range covered
    pub(crate) fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::Mbc1 { mbc } => {
                            mbc.ram_enabled = (value & 0x0F) == 0x0A;
                        }
                        _ => { }
                    }
                } else { }
            }

            0x2000..=0x3FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::Mbc1 { mbc } => {
                            mbc.bank1 = max(value & 0x1F, 1);
                        }
                        _ => { }
                    }
                } else { }
            }

            0x4000..=0x5FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::Mbc1 { mbc } => {
                            mbc.bank2 = value & 0x03;
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
            0x8000..=0x9FFF => {
                let adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);
                self.ram[adjusted_address] = value;
            }

            0xA000..=0xBFFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    cartridge.write_8_a000_bfff(address, value);
                } else {
                    let adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);
                    self.ram[adjusted_address] = value;
                }
            }

            0xC000..=0xFFFF => {
                let adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);

                self.ram[adjusted_address] = match address {
                    DIV_REG_ADDRESS => 0,           // All writes to timer DIV register reset it to 0
                    _ => value
                };

                if address == OAM_DMA_ADDRESS {
                    if self.oam_dma_source_address.is_none() {
                        self.oam_dma_source_address = Option::from((value as u16) << 8);
                    }
                }

                if self.is_booting && address == BANK_REG_ADDRESS && (value & 1) == 1 {
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
