use std::collections::HashMap;
use std::hash::Hash;

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

    pub(crate) fn load_rom(&mut self, rom: &[u8], start: usize, size: usize) {
        let mut n = start + size;
        if n >= self.rom.len() {
            n = self.rom.len()
        }
        self.rom[start..n].clone_from_slice(&rom[..size]);
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        if address < 0x8000 {
            self.rom[address as usize]
        } else {
            self.ram[(address - 0x8000) as usize]
        }

    }

    pub(crate) fn read_word(&self, address: u16, endian: Endianness) -> u16 {
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
            Endianness::LITTLE | _ => (lsb as u16) << 8 | msb as u16,
        }
    }

    pub(crate) fn read_buffer(&self, mut begin: usize, mut end: usize, endian: Endianness) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];

        if begin >= 0x8000 {
            begin -= 0x8000;
            end -= 0x8000;
        }
        if end > self.rom.len() {
            end = self.rom.len();
        }
        if begin >= end {
            panic!("invalid range of {}..{}", begin, end);
        }

        let slice: &[u8] = &self.rom[begin..end];

        match endian {
            Endianness::BIG => {
                let mut i = 0;
                while i < slice.len() - 1 {
                    buffer.push(slice[i + 1]);
                    buffer.push(slice[i]);
                    i += 2;
                }
            },
            Endianness::LITTLE | _ => {
                let mut i = 0;
                while i < slice.len() - 1 {
                    buffer.push(slice[i]);
                    buffer.push(slice[i + 1]);
                    i += 2;
                }
            },
        }

        buffer
    }

    pub(crate) fn load_byte(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            self.rom[address as usize] = value;
        } else {
            self.ram[(address - 0x8000) as usize] = value;
        }
    }

    pub(crate) fn load_word(&mut self, address: u16, value: u16, endian: Endianness) {
        let mut lsb = ((value & 0xFF00) >> 8) as u8;
        let mut msb = (value & 0x00FF) as u8;

        if endian == Endianness::BIG {
            let temp = lsb;
            lsb = msb;
            msb = temp;
        }

        self.load_byte(address, lsb);
        self.load_byte(address + 1, msb);
    }
}
