use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read;
use crate::cartridge::cartridge::Cartridge;
use crate::cartridge::mbc::Mbc;
use crate::console::input::JoypadInput;
use crate::console::ppu;
use crate::console::timer::Timer;

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

const BOOTROM_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/bootrom/dmg.bin";

#[allow(dead_code)]
#[derive(PartialEq)]
pub(crate) enum Endianness {
    BIG,
    LITTLE,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum Caller {
    CPU,
    PPU,
    TIMER,
}

pub(crate) struct Mmu {
    pub(crate) sysclock: u16, // 16bit internal DIV reg, incremented by Timer, TODO relocate?
    pub(crate) is_booting: bool,
    pub(crate) oam_dma_src_addr: Option<u16>,
    pub(crate) active_input: HashSet<JoypadInput>,  // TODO this doesn't belong here
    cartridge: Option<Cartridge>,
    debug_address: Option<u16>,
    debug_value: u8,
    rom: [u8; 0x8000usize],
    ram: [u8; 0x8000usize],
    pub(crate) ppu_mode: ppu::StatMode,
}

impl Mmu {
    pub(crate) fn new(cartridge: Option<Cartridge>, skip_boot: bool) -> Mmu {
        let mut mmu = Mmu {
            sysclock: 0,
            is_booting: true,
            oam_dma_src_addr: None,
            active_input: HashSet::from([]),
            cartridge,
            debug_address: None, // Option::from(IF_REG),
            debug_value: 0,
            rom: [0; 0x8000],
            ram: [0; 0x8000],
            ppu_mode: ppu::StatMode::HBlank,
        };

        if !skip_boot {
            mmu.load_bootrom();
        }

        mmu.ram[(JOYPAD_REG - 0x8000) as usize] = 0x1F;

        mmu
    }

    // noinspection RsNonExhaustiveMatch -- u16 range covered
    pub(crate) fn read_8(&mut self, address: u16, caller: Caller) -> u8 {
        let result = match address {
            // BOOT ROM
            0x0000..=0x0100 if self.is_booting => {
                self.rom[address as usize]
            }

            // CARTRIDGE ROM FIXED
            0x0000..=0x3FFF => if let Some(cartridge) = &self.cartridge {
                cartridge.read_8_0000_3fff(address)
            } else {
                // println!("UNIMPLEMENTED: No cartridge loaded, cannot read address {:04X}.", address);
                // 0x00
                self.rom[address as usize]
            }

            // CARTRIDGE ROM SWITCHABLE
            0x4000..=0x7FFF => if let Some(cartridge) = &self.cartridge {
                cartridge.read_8_4000_7fff(address)
            } else {
                // println!("UNIMPLEMENTED: No cartridge loaded, cannot read address {:04X}.", address);
                // 0x00
                self.rom[address as usize]
            }

            // VRAM
            0x8000..=0x9FFF => {
                if caller == Caller::PPU || self.ppu_mode != ppu::StatMode::PixelTransfer {
                    self.ram[address as usize - 0x8000]
                } else {
                    // println!("VRAM LOCKED by PPU: Cannot read address {:04X}.", address);
                    0xFF
                }
            }

            // EXTERNAL RAM
            0xA000..=0xBFFF => {
                if let Some(cartridge) = &self.cartridge {
                    cartridge.read_8_a000_bfff(address)
                } else {
                    // println!("UNIMPLEMENTED: No cartridge loaded, cannot read address {:04X}.", address);
                    // 0x00
                    self.ram[address as usize - 0x8000]
                }
            }

            // INTERNAL RAM
            0xC000..=0xDFFF => {
                self.ram[address as usize - 0x8000]
            }

            // ECHO RAM C000-DDFF Mirror
            0xE000..=0xFDFF => {
                self.ram[address as usize - 0xA000]
            }

            // OAM RAM
            0xFE00..=0xFE9F => {
                if caller == Caller::PPU
                    || (self.ppu_mode != ppu::StatMode::OamSearch && self.ppu_mode != ppu::StatMode::PixelTransfer) {
                    self.ram[address as usize - 0x8000]
                } else {
                    println!("OAM LOCKED by PPU: Cannot read address {:04X}.", address);
                    0xFF
                }
            }

            // PROHIBITED -- returns $FF when OAM is blocked, otherwise the behavior depends on the hardware revision.
            // On DMG, MGB, SGB, and SGB2, reads during OAM block trigger OAM corruption. Reads otherwise return $00.
            0xFEA0..=0xFEFF => {
                println!("PROHIBITED RAM: Not supposed to read address {:04X}.", address);
                if self.ppu_mode == ppu::StatMode::OamSearch {
                    0xFF
                } else {
                    0x00
                }
            }

            // IO
            0xFF00..=0xFF7F => {
                match address {
                    JOYPAD_REG => self.read_joypad_reg(self.ram[address as usize - 0x8000]),
                    _ => self.ram[address as usize - 0x8000]
                }
            }

            // HRAM, IE
            0xFF80..=0xFFFF => {
                self.ram[address as usize - 0x8000]
            }
        };

        // For debugging because conditional breakpoints are unuseably slow
        if let Some(debug_address) = self.debug_address {
            if address == debug_address && result != self.debug_value {
                self.debug_value = result;
                println!("(READ) [{:04X}] = {:02X}", debug_address, self.debug_value);
            }
        }

        result
    }

    pub(crate) fn read_16(&mut self, address: u16, endian: Endianness, caller: Caller) -> u16 {
        let lsb: u8 = self.read_8(address, caller);
        let msb: u8 = self.read_8(address + 1, caller);

        match endian {
            Endianness::BIG => (msb as u16) << 8 | lsb as u16,
            Endianness::LITTLE => (lsb as u16) << 8 | msb as u16,
        }
    }

    pub(crate) fn read_buffer(&mut self, start: u16, end: u16, caller: Caller) -> Vec<u8> {
        let mut result = vec![0; end as usize - start as usize];
        for address in start..end {
            result[address as usize - start as usize] = self.read_8(address, caller);
        }
        result
    }

    // noinspection RsNonExhaustiveMatch -- u16 range covered
    pub(crate) fn write_8(&mut self, address: u16, value: u8, caller: Caller) {
        let mut adjusted_address = address as usize;

        match address {
            // BOOT ROM
            0x0000..=0x0100 if self.is_booting => {
                // println!("UNIMPLEMENTED: BOOT ROM, cannot write address {:04X}.", address);
                self.rom[address as usize] = value;
            }

            // CARTRIDGE ROM
            0x0000..=0x1FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::None => {
                            self.rom[address as usize] = value;
                        }
                        Mbc::Mbc1 { mbc } => {
                            mbc.ram_enabled = (value & 0x0F) == 0x0A;
                        }
                        Mbc::Mbc2 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc2, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc3 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc3, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc5 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc5, cannot write address {:04X}.", address);
                        }
                    }
                } else {
                    // println!("UNIMPLEMENTED: No cartridge loaded, cannot write address {:04X}.", address);
                    self.rom[address as usize] = value;
                }
            }

            // CARTRIDGE ROM
            0x2000..=0x3FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::None => {
                            self.rom[address as usize] = value;
                        }
                        Mbc::Mbc1 { mbc } => {
                            mbc.bank1 = max(value & 0x1F, 1);
                        }
                        Mbc::Mbc2 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc2, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc3 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc3, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc5 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc5, cannot write address {:04X}.", address);
                        }
                    }
                } else {
                    // println!("UNIMPLEMENTED: No cartridge loaded, cannot write address {:04X}.", address);
                    self.rom[address as usize] = value;
                }
            }

            // CARTRIDGE ROM
            0x4000..=0x5FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::None => {
                            self.rom[address as usize] = value;
                        }
                        Mbc::Mbc1 { mbc } => {
                            mbc.bank2 = value & 0x03;
                        }
                        Mbc::Mbc2 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc2, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc3 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc3, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc5 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc5, cannot write address {:04X}.", address);
                        }
                    }
                } else {
                    // println!("UNIMPLEMENTED: No cartridge loaded, cannot write address {:04X}.", address);
                    self.rom[address as usize] = value;
                }
            }

            // CARTRIDGE ROM
            0x6000..=0x7FFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    match &mut cartridge.mbc {
                        Mbc::None => {
                            self.rom[address as usize] = value;
                        }
                        Mbc::Mbc1 { mbc } => {
                            mbc.advram_banking_mode = (value & 1) == 1;
                        }
                        Mbc::Mbc2 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc2, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc3 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc3, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc5 => {
                            println!("UNIMPLEMENTED: Unsupported cartridge type Mbc5, cannot write address {:04X}.", address);
                        }
                    }
                } else {
                    // println!("UNIMPLEMENTED: No cartridge loaded, cannot write address {:04X}.", address);
                    self.rom[address as usize] = value;
                }
            }

            // VRAM
            0x8000..=0x9FFF => {
                if caller == Caller::PPU || self.ppu_mode != ppu::StatMode::PixelTransfer {
                    adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);
                    self.ram[adjusted_address] = value;
                } else {
                    println!("VRAM LOCKED by PPU: Cannot write address {:04X}.", address);
                }
            }

            // EXTERNAL RAM
            0xA000..=0xBFFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    cartridge.write_8_a000_bfff(address, value);
                } else {
                    adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);
                    self.ram[adjusted_address] = value;
                }
            }

            // INTERNAL RAM
            0xC000..=0xDFFF => {
                adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);
                self.ram[adjusted_address] = value;
            }

            // ECHO RAM C000-DDFF Mirror
            0xE000..=0xFDFF => {
                adjusted_address = (address as usize - 0xA000) & (self.ram.len() - 1);
                self.ram[adjusted_address] = value;
            }

            // OAM RAM
            0xFE00..=0xFE9F => {
                if caller == Caller::PPU
                    || (self.ppu_mode != ppu::StatMode::OamSearch && self.ppu_mode != ppu::StatMode::PixelTransfer) {
                    adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);
                    self.ram[adjusted_address] = value;
                } else {
                    println!("OAM LOCKED by PPU: Cannot write address {:04X}.", address);
                }
            }

            // PROHIBITED
            0xFEA0..=0xFEFF => {
                // println!("PROHIBITED RAM: Not supposed to write address {:04X}.", address);
                let adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);
                self.ram[adjusted_address] = value;
            }

            // IO
            0xFF00..=0xFF7F => {
                adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);
                let adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);

                match address {
                    // TIMER
                    DIV_REG => {
                        if caller == Caller::TIMER { // so that Timer can update DIV
                            self.ram[adjusted_address] = value;
                        } else {
                            // All writes to timer DIV register reset it to 0.
                            self.ram[adjusted_address] = 0;
                            self.sysclock = 0; // Internal clock is also reset(?)
                            // TODO How is TIMA affected...?
                        }
                    }
                    TIMA_REG => {
                        self.ram[adjusted_address] = value;
                    }
                    TMA_REG => {
                        self.ram[adjusted_address] = value;
                    }
                    TAC_REG => {
                        let new_tac = value & 0b0111;

                        // TODO Does other timer registers also need to be updated on writes?
                        // let old_tac = self.read_8(address, caller);
                        // let tima_incr_is_enabled = Timer::is_tac_enabled(old_tac);
                        // let old_counter_bit = tima_incr_is_enabled && Timer::is_counter_bit_flipped(self.sysclock, old_tac);

                        self.ram[adjusted_address] = new_tac;

                        // TODO Does other timer registers also need to be updated on writes?
                        // let tima_incr_is_enabled = Timer::is_tac_enabled(new_tac);
                        // let new_counter_bit = tima_incr_is_enabled && Timer::is_counter_bit_flipped(self.sysclock, new_tac);
                        //
                        // // if old_counter_bit != new_counter_bit {
                        // if old_counter_bit && !new_counter_bit {
                        //     // increment TIMA
                        //     let old_tima = self.read_8(TIMA_REG, Caller::TIMER);
                        //     let (mut new_tima, tima_overflow) = old_tima.overflowing_add(1);
                        //     if tima_overflow {
                        //         new_tima = self.read_8(TMA_REG, Caller::TIMER);
                        //     }
                        //     self.write_8(TIMA_REG, new_tima, Caller::TIMER);
                        // }
                    }

                    // JOYPAD
                    JOYPAD_REG => {
                        // Bottom nibble is read only
                        let curr_value = self.ram[adjusted_address];
                        self.ram[adjusted_address] = (value & 0xF0) | (curr_value & 0x0F);
                    }

                    // PPU
                    LY_REG => {
                        if caller == Caller::PPU {
                            self.ram[adjusted_address] = value;
                        } else {
                            // Read-only
                        }
                    }
                    LCD_STATUS_REG => {
                        if caller == Caller::PPU {
                            self.ram[adjusted_address] = value;
                            self.ppu_mode = ppu::STAT_MODES[(value & 0x03) as usize]; // dr mario getting locked in non hblank...
                        } else {
                            // Bits 0,1,2 are read-only (only PPU can update)
                            let curr_value = self.ram[adjusted_address];
                            self.ram[adjusted_address] = (value & 0xF8) | (curr_value & 0x07);
                        }
                    }
                    LCD_CONTROL_REG => {
                        if self.ppu_mode == ppu::StatMode::VBlank {
                            println!("Writing CONTROL during VBlank: {:04X}", value);
                            self.ram[adjusted_address] = value;
                        }
                        else {
                            println!("Writing CONTROL outside VBlank, please don't clear bit 7: {:04X}", value);
                            // let curr_value = self.ram[adjusted_address];
                            // self.ram[adjusted_address] = value | (curr_value & 0x80); // Don't clear bit 7
                            self.ram[adjusted_address] = value;
                        }
                    }
                    DMA_REG => {
                        self.ram[adjusted_address] = value;
                        self.oam_dma_src_addr = Option::from((value as u16) << 8);
                    }

                    // BANKING
                    BANK_REG => {
                        self.ram[adjusted_address] = value;
                        if self.is_booting && (value & 1) == 1 {
                            self.is_booting = false;
                        }
                    }

                    _ => {
                        self.ram[adjusted_address] = value;
                    }
                }
            }

            // HRAM, IE
            0xFF80..=0xFFFF => {
                adjusted_address = (address as usize - 0x8000) & (self.ram.len() - 1);
                self.ram[adjusted_address] = value;
            }
        }

        // For debugging because conditional breakpoints are unuseably slow
        if let Some(debug_address) = self.debug_address {
            if address == debug_address { // && self.ram[adjusted_address] != self.debug_value {
                self.debug_value = self.ram[adjusted_address];
                println!("(WRITE {:02X}) [{:04X}] = {:02X} Caller:{:?}",
                    value, debug_address, self.debug_value, caller);
            }
        }
    }

    pub(crate) fn write_16(&mut self, address: u16, value: u16, endian: Endianness, caller: Caller) {
        let mut a = ((value & 0xFF00) >> 8) as u8;
        let mut b = (value & 0x00FF) as u8;

        if endian == Endianness::BIG {
            let temp = a;
            a = b;
            b = temp;
        }

        self.write_8(address, a, caller);
        self.write_8(address + 1, b, caller);
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
