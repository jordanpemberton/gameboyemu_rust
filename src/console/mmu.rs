const BOOT_ROM_BEGIN: usize = 0x0000;
const BOOT_ROM_END: usize = 0x0100;
const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

const CARTRIDGE_ROM_BEGIN: usize = 0x0000;
const CARTRIDGE_ROM_END: usize = 0x7fff;
const CARTRIDGE_ROM_SIZE: usize = CARTRIDGE_ROM_END - CARTRIDGE_ROM_BEGIN + 1;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[allow(non_camel_case_types)]
enum Memory {
    BOOT_ROM,
    CARTRIDGE_ROM,
    VRAM,
}

const BOOT_ROM: &[u8] = include_bytes!("DMG_ROM.bin");

pub(crate) struct Mmu {
    boot_rom: [u8; BOOT_ROM_SIZE],
    cartridge_rom: [u8; CARTRIDGE_ROM_SIZE],
    vram: [u8; VRAM_SIZE],
}

impl Mmu {
    pub(crate) fn new() -> Mmu {
        let mut _boot_rom = [0; BOOT_ROM_SIZE];
        _boot_rom[..BOOT_ROM.len()].clone_from_slice(&BOOT_ROM);

        Mmu {
            boot_rom: _boot_rom,
            cartridge_rom: [0; CARTRIDGE_ROM_SIZE],
            vram: [0; VRAM_SIZE],
        }
    }

    pub(crate) fn read_byte(&self, address: usize) -> u8 {
        if BOOT_ROM_BEGIN <= address && address <= BOOT_ROM_END {
            self.boot_rom[address - BOOT_ROM_BEGIN]
        } else if CARTRIDGE_ROM_BEGIN <= address && address <= CARTRIDGE_ROM_END {
            self.cartridge_rom[address - CARTRIDGE_ROM_BEGIN]
        } else if VRAM_BEGIN <= address && address <= VRAM_END {
            self.vram[address - VRAM_BEGIN]
        } else { 0 }
    }

    pub(crate) fn read_word(&self, address: usize) -> u16 {
        if BOOT_ROM_BEGIN <= address && address < BOOT_ROM_END {
            (self.boot_rom[address - BOOT_ROM_BEGIN] as u16) << 8
                | self.boot_rom[address - BOOT_ROM_BEGIN + 1] as u16
        } else if CARTRIDGE_ROM_BEGIN <= address && address < CARTRIDGE_ROM_END {
            (self.cartridge_rom[address - CARTRIDGE_ROM_BEGIN] as u16) << 8
                | self.cartridge_rom[address - CARTRIDGE_ROM_BEGIN + 1] as u16
        } else if VRAM_BEGIN <= address && address < VRAM_END {
            (self.vram[address - VRAM_BEGIN] as u16) << 8
                | self.vram[address - VRAM_BEGIN + 1] as u16
        }
        else { 0 }
    }

    pub(crate) fn load_byte(&mut self, address: usize, value: u8) {
        if BOOT_ROM_BEGIN <= address && address <= BOOT_ROM_END {
            self.boot_rom[address - BOOT_ROM_BEGIN] = value;
        } else if CARTRIDGE_ROM_BEGIN <= address && address <= CARTRIDGE_ROM_END {
            self.cartridge_rom[address - CARTRIDGE_ROM_BEGIN] = value;
        } else if VRAM_BEGIN <= address && address <= VRAM_END {
            self.vram[address - VRAM_BEGIN] = value;
        } else { }
    }

    pub(crate) fn load_word(&mut self, address: usize, value: u16) {
        let lsb = ((value & 0xFF00) >> 8) as u8;
        let msb = (value & 0x00FF) as u8;

        if BOOT_ROM_BEGIN <= address && address < BOOT_ROM_END {
            self.boot_rom[address - BOOT_ROM_BEGIN] = lsb;
            self.boot_rom[address - BOOT_ROM_BEGIN + 1] = msb;
        } else if CARTRIDGE_ROM_BEGIN <= address && address < CARTRIDGE_ROM_END {
            self.cartridge_rom[address - CARTRIDGE_ROM_BEGIN] = lsb;
            self.cartridge_rom[address - CARTRIDGE_ROM_BEGIN + 1] = msb;
        } else if VRAM_BEGIN <= address && address < VRAM_END {
            self.vram[address - VRAM_BEGIN] = lsb;
            self.vram[address - VRAM_BEGIN + 1] = msb;
        } else { }
    }
}
