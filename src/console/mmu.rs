const CARTRIDGE_ROM_BEGIN: usize = 0x0000;
const CARTRIDGE_ROM_END: usize = 0x7fff;
const CARTRIDGE_ROM_SIZE: usize = CARTRIDGE_ROM_END - CARTRIDGE_ROM_BEGIN + 1;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[allow(non_camel_case_types)]
enum Memory {
    CARTRIDGE_ROM,
    VRAM,
}

pub(crate) struct Mmu {
    pub(crate) cartridge_rom: [u8; CARTRIDGE_ROM_SIZE],
    vram: [u8; VRAM_SIZE],
}

impl Mmu {
    pub(crate) fn new() -> Mmu {
        Mmu {
            cartridge_rom: [0; CARTRIDGE_ROM_SIZE],
            vram: [0; VRAM_SIZE],
        }
    }

    pub(crate) fn read_byte(&self, address: usize) -> u8 {
        if CARTRIDGE_ROM_BEGIN <= address && address <= CARTRIDGE_ROM_END {
            self.cartridge_rom[address]
        } else if VRAM_BEGIN <= address && address <= VRAM_END {
            self.vram[address]
        } else {
            0
        }
    }

    pub(crate) fn read_word(&self, address: usize) -> u16 {
        if CARTRIDGE_ROM_BEGIN <= address && address <= CARTRIDGE_ROM_END {
            (self.cartridge_rom[address] as u16) << 8 | self.cartridge_rom[address + 1] as u16
        } else if VRAM_BEGIN <= address && address <= VRAM_END {
            (self.vram[address] as u16) << 8 | self.vram[address + 1] as u16
        }
        else {
            0
        }
    }

    pub(crate) fn load_byte(&mut self, address: usize, value: u8) {
        if CARTRIDGE_ROM_BEGIN <= address && address <= CARTRIDGE_ROM_END {
            self.cartridge_rom[address] = value;
        } else if VRAM_BEGIN <= address && address <= VRAM_END {
            self.vram[address] = value;
        } else { }
    }

    pub(crate) fn load_word(&mut self, address: usize, value: u16) {
        let lsb = ((value & 0xFF00) >> 8) as u8;
        let msb = (value & 0x00FF) as u8;

        if CARTRIDGE_ROM_BEGIN <= address && address <= CARTRIDGE_ROM_END {
            self.cartridge_rom[address] = lsb;
            self.cartridge_rom[address + 1] = msb;
        } else if VRAM_BEGIN <= address && address <= VRAM_END {
            self.vram[address] = lsb;
            self.vram[address + 1] = msb;
        } else { }
    }
}
