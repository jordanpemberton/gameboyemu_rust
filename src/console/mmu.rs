const BOOT_ROM_BEGIN: u16 = 0x0000;
const BOOT_ROM_END: u16 = 0x0100;
const BOOT_ROM_SIZE: u16 = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

const CARTRIDGE_ROM_BEGIN: u16 = 0x0000;
const CARTRIDGE_ROM_END: u16 = 0x7fff;
const CARTRIDGE_ROM_SIZE: u16 = CARTRIDGE_ROM_END - CARTRIDGE_ROM_BEGIN + 1;

const VRAM_BEGIN: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const VRAM_SIZE: u16 = VRAM_END - VRAM_BEGIN + 1;

#[derive(PartialEq)]
pub(crate) enum Endianness {
    BIG,
    LITTLE,
}

#[allow(non_camel_case_types)]
enum Memory {
    BOOT_ROM,
    CARTRIDGE_ROM,
    VRAM,
}

const BOOT_ROM: &[u8] = include_bytes!("DMG_ROM.bin");

pub(crate) struct Mmu {
    boot_rom: [u8; BOOT_ROM_SIZE as usize],
    cartridge_rom: [u8; CARTRIDGE_ROM_SIZE as usize],
    vram: [u8; VRAM_SIZE as usize],
}

impl Mmu {
    pub(crate) fn new() -> Mmu {
        let mut _boot_rom = [0; BOOT_ROM_SIZE as usize];
        _boot_rom[..BOOT_ROM.len()].clone_from_slice(&BOOT_ROM);

        Mmu {
            boot_rom: _boot_rom,
            cartridge_rom: [0; CARTRIDGE_ROM_SIZE as usize],
            vram: [0; VRAM_SIZE as usize],
        }
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        if BOOT_ROM_BEGIN <= address && address <= BOOT_ROM_END {
            self.boot_rom[(address - BOOT_ROM_BEGIN) as usize]
        } else if CARTRIDGE_ROM_BEGIN <= address && address <= CARTRIDGE_ROM_END {
            self.cartridge_rom[(address - CARTRIDGE_ROM_BEGIN) as usize]
        } else if VRAM_BEGIN <= address && address <= VRAM_END {
            self.vram[(address - VRAM_BEGIN) as usize]
        } else { 0 }
    }

    pub(crate) fn read_word(&self, address: u16, endian: Endianness) -> u16 {
        let mut lsb: u8 = 0;
        let mut msb: u8 = 0;

        if BOOT_ROM_BEGIN <= address && address < BOOT_ROM_END {
            lsb = self.boot_rom[(address - BOOT_ROM_BEGIN) as usize];
            msb = self.boot_rom[(address - BOOT_ROM_BEGIN + 1) as usize];
        } else if CARTRIDGE_ROM_BEGIN <= address && address < CARTRIDGE_ROM_END {
            lsb = self.cartridge_rom[(address - CARTRIDGE_ROM_BEGIN) as usize];
            msb = self.cartridge_rom[(address - CARTRIDGE_ROM_BEGIN + 1) as usize];
        } else if VRAM_BEGIN <= address && address < VRAM_END {
            lsb = self.vram[(address - VRAM_BEGIN) as usize];
            msb = self.vram[(address - VRAM_BEGIN + 1) as usize];
        }

        match endian {
            Endianness::BIG => (msb as u16) << 8 | lsb as u16,
            Endianness::LITTLE | _ =>  (lsb as u16) << 8 | msb as u16,
        }
    }

    pub(crate) fn load_byte(&mut self, address: u16, value: u8) {
        if BOOT_ROM_BEGIN <= address && address <= BOOT_ROM_END {
            self.boot_rom[(address - BOOT_ROM_BEGIN) as usize] = value;
        } else if CARTRIDGE_ROM_BEGIN <= address && address <= CARTRIDGE_ROM_END {
            self.cartridge_rom[(address - CARTRIDGE_ROM_BEGIN) as usize] = value;
        } else if VRAM_BEGIN <= address && address <= VRAM_END {
            self.vram[(address - VRAM_BEGIN) as usize] = value;
        } else { }
    }

    pub(crate) fn load_word(&mut self, address: u16, value: u16, endian: Endianness) {
        let mut lsb = ((value & 0xFF00) >> 8) as u8;
        let mut msb = (value & 0x00FF) as u8;
        if endian == Endianness::BIG {
            let temp = lsb;
            lsb = msb;
            msb = temp;
        }

        if BOOT_ROM_BEGIN <= address && address < BOOT_ROM_END {
            self.boot_rom[(address - BOOT_ROM_BEGIN) as usize] = lsb;
            self.boot_rom[(address - BOOT_ROM_BEGIN + 1) as usize] = msb;
        } else if CARTRIDGE_ROM_BEGIN <= address && address < CARTRIDGE_ROM_END {
            self.cartridge_rom[(address - CARTRIDGE_ROM_BEGIN) as usize] = lsb;
            self.cartridge_rom[(address - CARTRIDGE_ROM_BEGIN + 1) as usize] = msb;
        } else if VRAM_BEGIN <= address && address < VRAM_END {
            self.vram[(address - VRAM_BEGIN) as usize] = lsb;
            self.vram[(address - VRAM_BEGIN + 1) as usize] = msb;
        } else { }
    }
}
