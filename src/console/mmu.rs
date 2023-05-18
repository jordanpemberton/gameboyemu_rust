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

use crate::console::timer::DIV_REG_ADDRESS;

#[allow(dead_code)]
#[derive(PartialEq)]
pub(crate) enum Endianness {
    BIG,
    LITTLE,
}

pub(crate) struct Mmu {
    rom: [u8; 0x8000 as usize],
    ram: [u8; 0x8000 as usize],
}

impl Mmu {
    pub(crate) fn new() -> Mmu {
        Mmu {
            rom: [0; 0x8000],
            ram: [0; 0x8000],
        }
    }

    pub(crate) fn read(&self, begin: usize, end: usize) -> Vec<u8> {
        if begin >= end {
            panic!("invalid range of {:#06X}..{:#06X}", begin, end);
        }

        if begin < 0x8000 {
            if end > 0x8000 {
                panic!("invalid range of {:#06X}..{:#06X}", begin, end);
            }
            self.rom[begin..end].to_vec()
        } else {
            if end - 0x8000 > 0x8000 {
                panic!("invalid range of {:#06X}..{:#06X}", begin, end);
            }
            self.rom[begin-0x8000..end-0x8000].to_vec()
        }
    }

    pub(crate) fn read_8(&self, address: u16) -> u8 {
        if address < 0x8000 {
            self.rom[address as usize]
        } else {
            self.ram[(address - 0x8000) as usize]
        }
    }

    pub(crate) fn read_16(&self, address: u16, endian: Endianness) -> u16 {
        let lsb: u8;
        let msb: u8;

        if address < 0x8000 {
            lsb = self.rom[address as usize];
            msb = self.rom[(address + 1) as usize];
        } else {
            lsb = self.ram[(address - 0x8000) as usize];
            msb = self.ram[(address - 0x8000 + 1) as usize];
        }

        match endian {
            Endianness::BIG => (msb as u16) << 8 | lsb as u16,
            Endianness::LITTLE => (lsb as u16) << 8 | msb as u16,
        }
    }

    pub(crate) fn write(&mut self, data: &[u8], start: usize, mut size: usize) {
        if size > data.len() {
            size = data.len();
        }
        let mut end = start + size;
        if start < 0x8000 {
            if end > 0x8000 {
                end = 0x8000;
            }
            if size > end {
                size = end;
            }
            self.rom[start..end].clone_from_slice(&data[..size]);
        } else {
            if end >= 0xFFFF {
                end = 0xFFFF;
            }
            if size > end {
                size = end;
            }
            self.ram[start - 0x8000..end - 0x8000].clone_from_slice(&data[..size - 0x8000]);

            // All writes to timer DIV register reset it to 0
            if start <= (DIV_REG_ADDRESS as usize) && (DIV_REG_ADDRESS as usize) < end {
                self.ram[DIV_REG_ADDRESS as usize - 0x8000] = 0;
            }
        }
    }

    pub(crate) fn write_8(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            self.rom[address as usize] = value;
        } else {
            self.ram[(address - 0x8000) as usize] = if address == DIV_REG_ADDRESS { // All writes to timer DIV register reset it to 0
                0
            } else {
                value
            };
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
}
