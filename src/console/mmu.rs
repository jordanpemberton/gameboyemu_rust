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
use std::collections::HashSet;
use std::fs::read;
use crate::cartridge::cartridge::Cartridge;
use crate::cartridge::mbc::Mbc;
use crate::console::input::JoypadInput;

// OAM
pub(crate) const OAM_START: u16 = 0xFE00;
pub(crate) const OAM_END: u16 = 0xFE9F;
// IO
pub(crate) const JOYPAD_REG: u16 = 0xFF00;
// Timer
pub(crate) const DIV_REG: u16 = 0xFF04;
pub(crate) const TIMA_REG: u16 = 0xFF05;
pub(crate) const TMA_REG: u16 = 0xFF06;
pub(crate) const TAC_REG: u16 = 0xFF07;
// Interrupts
pub(crate) const IF_REG: u16 = 0xFF0F;
// PPU
pub(crate) const LCD_CONTROL_REG: u16 = 0xFF40;
pub(crate) const LCD_STATUS_REG: u16 = 0xFF41;
pub(crate) const SCY_REG: u16 = 0xFF42;
pub(crate) const SCX_REG: u16 = 0xFF43;
pub(crate) const LY_REG: u16 = 0xFF44;
pub(crate) const LYC_REG: u16 = 0xFF45;
pub(crate) const DMA_REG: u16 = 0xFF46;
pub(crate) const BGP_REG: u16 = 0xFF47;
pub(crate) const OBP0_REG: u16 = 0xFF48;
pub(crate) const OBP1_REG: u16 = 0xFF49;
pub(crate) const WY_REG: u16 = 0xFF4A;
pub(crate) const WX_REG: u16 = 0xFF4B;
// Banking, MBC
pub(crate) const BANK_REG: u16 = 0xFF50;
// Interrupts
pub(crate) const IE_REG: u16 = 0xFFFF;

const BOOTROM_FILEPATH: &str = "roms/bootrom/DMG_ROM.bin";

#[allow(dead_code)]
#[derive(PartialEq)]
pub(crate) enum Endianness {
    BIG,
    LITTLE,
}

pub(crate) trait Memory {
    fn read_8(&mut self, address: u16) -> u8;
    fn write_8(&mut self, address: u16, value: u8);
}

pub(crate) struct Mmu {
    pub(crate) is_booting: bool,
    pub(crate) oam_dma_source_address: Option<u16>,
    rom: [u8; 0x8000 as usize],
    ram: [u8; 0x8000 as usize],
    cartridge: Option<Cartridge>,
    pub(crate) active_input: HashSet<JoypadInput>,  // TODO this doesn't belong here
}

// TODO -- Rewrite Mmu, add handlers, eliminate duplicated code.
impl Mmu {
    pub(crate) fn new(cartridge: Option<Cartridge>) -> Mmu {
        let mut mmu = Mmu {
            is_booting: true,
            oam_dma_source_address: None,
            rom: [0; 0x8000],
            ram: [0; 0x8000],
            cartridge,
            active_input: HashSet::from([]),
        };
        mmu.load_bootrom();
        mmu.ram[(JOYPAD_REG - 0x8000) as usize] = 0x1F;

        mmu
    }

    // noinspection RsNonExhaustiveMatch -- u16 range covered
    pub(crate) fn read_8(&mut self, address: u16) -> u8 {
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
                let result = match address {
                    JOYPAD_REG => self.read_joypad_reg(self.ram[address as usize - 0x8000]),
                    _ => self.ram[address as usize - 0x8000]
                };

                // For debugging
                // if address == 0xFF40 {
                //     println!("Get {:#04X}: {:#04X}", address, result);
                // }

                result
            }
        }
    }

    pub(crate) fn read_16(&mut self, address: u16, endian: Endianness) -> u16 {
        let lsb: u8 = self.read_8(address);
        let msb: u8 = self.read_8(address + 1);

        match endian {
            Endianness::BIG => (msb as u16) << 8 | lsb as u16,
            Endianness::LITTLE => (lsb as u16) << 8 | msb as u16,
        }
    }

    pub(crate) fn read_buffer(&mut self, start: u16, end: u16) -> Vec<u8> {
        let mut result = vec![0; end as usize - start as usize];
        for address in start..end {
            result[address as usize - start as usize] = self.read_8(address);
        }
        result
    }

    // noinspection RsNonExhaustiveMatch -- u16 range covered
    pub(crate) fn write_8(&mut self, address: u16, mut value: u8) {
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

                let curr_value = self.ram[adjusted_address];
                value = match address {
                    DIV_REG => 0,       // All writes to timer DIV register reset it to 0
                    JOYPAD_REG => (value & 0xF0) | (curr_value & 0x0F), // Bottom nibble is read only
                    _ => value
                };
                self.ram[adjusted_address] = value;

                if address == DMA_REG {
                    if self.oam_dma_source_address.is_none() {
                        self.oam_dma_source_address = Option::from((value as u16) << 8);
                    }
                }

                // For debugging
                // if address == 0xFF40 {
                //     println!("Set {:#04X} = {:#04X}", address, value);
                // }

                if self.is_booting && address == BANK_REG && (value & 1) == 1 {
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

    // TODO rewrite this and relocate
    fn read_joypad_reg(&mut self, reg_value: u8) -> u8 {
        let select_action_is_enabled = (reg_value & (1 << 5)) >> 5 == 0; // "0"==selected
        let directional_is_enabled = (reg_value & (1 << 4)) >> 4 == 0;

        let mut active_inputs: u8 = 0x00;

        for input in &self.active_input {
            let selected_input_bit = if select_action_is_enabled {
                match input {
                    JoypadInput::InputKeyStart => 1 << 3,
                    JoypadInput::InputKeySelect => 1 << 2,
                    JoypadInput::InputKeyB => 1 << 1,
                    JoypadInput::InputKeyA => 1 << 0,
                    _ => 0,
                }
            } else if directional_is_enabled {
                match input {
                    JoypadInput::InputKeyDown => 1 << 3,
                    JoypadInput::InputKeyUp => 1 << 2,
                    JoypadInput::InputKeyLeft => 1 << 1,
                    JoypadInput::InputKeyRight => 1 << 0,
                    _ => 0
                }
            } else {
                0
            };

            active_inputs |= selected_input_bit;
        }

        active_inputs = !active_inputs;         // flip because "0" == selected

        (reg_value & 0xF0) | active_inputs
    }
}
