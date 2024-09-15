use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read;
use crate::cartridge::cartridge::Cartridge;
use crate::cartridge::mbc::Mbc;
use crate::console::input::JoypadInput;
use crate::console::ppu;

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
pub(crate) enum LcdControlRegBit {
    // 0	BG and Window enable/priority	0=Off, 1=On
    BackgroundAndWindowEnabled = 0,
    // 1	OBJ enable	0=Off, 1=On
    ObjEnabled = 1,
    // 2	OBJ size	0=8x8, 1=8x16
    SpriteSizeIs16 = 2,
    // 3	BG tile map area	0=9800-9BFF, 1=9C00-9FFF
    BackgroundTilemapIsAt9C00 = 3,
    // 4	BG and Window tile data area	0=8800-97FF, 1=8000-8FFF
    AddressingMode8000 = 4,
    // 5	Window enable	0=Off, 1=On
    WindowEnabled = 5,
    // 6	Window tile map area	0=9800-9BFF, 1=9C00-9FFF
    WindowTilemapIsAt9C00 = 6,
    // 7	LCD and PPU enable	0=Off, 1=On
    // This bit controls whether the LCD is on and the PPU is active.
    // Setting it to 0 turns both off, which grants immediate and full access to VRAM, OAM, etc. TODO
    LcdAndPpuEnabled = 7,
}
pub(crate) const LCD_STATUS_REG: u16 = 0xFF41;
#[allow(dead_code)]
pub(crate) enum LcdStatRegBit {
    // Bit 0-1 - Mode Flag
    ModeBit0 = 0,
    ModeBit1 = 1,
    // Bit 2 - LYC=LY Flag
    LycEqLy = 2,
    // STAT Interrupt bits
    // Bit 3 - Mode 0 HBlank STAT Interrupt source  (1=Enable)
    HBlankInterruptEnabled = 3,
    // Bit 4 - Mode 1 VBlank STAT Interrupt source  (1=Enable)
    VBlankInterruptEnabled = 4,
    // Bit 5 - Mode 2 OAM STAT Interrupt source     (1=Enable)
    OamInterruptEnabled = 5,
    // Bit 6 - LYC=LY STAT Interrupt source         (1=Enable)
    LycInterruptEnabled = 6,
}
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
    debug_written_value: u8,
    debug_read_value: u8,
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
            debug_address: Option::from(LCD_CONTROL_REG),
            debug_written_value: 0,
            debug_read_value: 0,
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

    //noinspection RsNonExhaustiveMatch
    pub(crate) fn read_8(&mut self, address: u16, caller: Caller) -> u8 {
        let result = match address {
            0x0000..=0x7FFF => self.read_8_rom(address, caller),
            0x8000..=0xFFFF => self.read_8_ram(address, caller),
        };

        // For debugging because conditional breakpoints are unuseably slow
        if let Some(debug_address) = self.debug_address {
            if address == debug_address && result != self.debug_read_value { // only log if it changed
                self.debug_read_value = result;
                println!("READ [{:04X}] => {:02X}", debug_address, self.debug_read_value);
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

    //noinspection RsNonExhaustiveMatch
    pub(crate) fn write_8(&mut self, address: u16, value: u8, caller: Caller) {
        let result = match address {
            0x0000..=0x7FFF => self.write_8_rom(address, value, caller),
            0x8000..=0xFFFF => self.write_8_ram(address, value, caller),
        };

        // For debugging because conditional breakpoints are unuseably slow
        if let Some(debug_address) = self.debug_address {
            if address == debug_address { // && result != self.debug_written_value { // only log if it changed
                self.debug_written_value = result;
                println!("WRITE {:02X} -> [{:04X}] => {:02X}",
                    value, debug_address, self.debug_written_value);
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

    #[allow(unused_variables)]
    fn read_8_rom(&mut self, address: u16, caller: Caller) -> u8 {
        let result = match address {
            // BOOT ROM
            0x0000..=0x0100 if self.is_booting => {
                self.rom[address as usize]
            }

            // CARTRIDGE ROM FIXED
            0x0000..=0x3FFF => if let Some(cartridge) = &self.cartridge {
                cartridge.read_8_0000_3fff(address)
            } else {
                panic!("UNIMPLEMENTED: No cartridge loaded, cannot read address {:04X}.", address);
            }

            // CARTRIDGE ROM SWITCHABLE
            0x4000..=0x7FFF => if let Some(cartridge) = &self.cartridge {
                cartridge.read_8_4000_7fff(address)
            } else {
                panic!("UNIMPLEMENTED: No cartridge loaded, cannot read address {:04X}.", address);
            },

            _ => {
                panic!("Invalid ROM address {:6X}", address);
            }
        };

        result
    }

    fn read_8_ram(&mut self, address: u16, caller: Caller) -> u8 {
        let ram_address = address as usize - 0x8000;

        let result = match address {
            // VRAM
            0x8000..=0x9FFF => {
                if caller == Caller::PPU || self.ppu_mode != ppu::StatMode::PixelTransfer {
                    self.ram[ram_address]
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
                    panic!("UNIMPLEMENTED: No cartridge loaded, cannot read address {:04X}.", address);
                }
            }

            // INTERNAL RAM
            0xC000..=0xDFFF => {
                self.ram[ram_address]
            }

            // ECHO RAM -- Mirrors C000-DDFF
            0xE000..=0xFDFF => {
                let ram_address = address as usize - 0xA000;
                self.ram[ram_address]
            }

            // OAM RAM
            0xFE00..=0xFE9F => {
                if caller == Caller::PPU
                    || (self.ppu_mode != ppu::StatMode::OamSearch && self.ppu_mode != ppu::StatMode::PixelTransfer) {
                    self.ram[ram_address]
                } else {
                    println!("OAM LOCKED by PPU: Cannot read address {:04X}.", address);
                    0xFF
                }
            }

            // PROHIBITED -- returns $FF when OAM is blocked, otherwise the behavior depends on the hardware revision.
            // On DMG, MGB, SGB, and SGB2, reads during OAM block trigger OAM corruption. Reads otherwise return $00.
            0xFEA0..=0xFEFF => {
                // println!("PROHIBITED RAM: Not supposed to read address {:04X}.", address);
                if self.ppu_mode == ppu::StatMode::OamSearch {
                    0xFF
                } else {
                    0x00
                }
            }

            // IO
            0xFF00..=0xFF7F => {
                match address {
                    JOYPAD_REG => self.read_joypad_reg(self.ram[ram_address]),
                    _ => self.ram[ram_address]
                }
            }

            // HRAM, IE
            0xFF80..=0xFFFF => {
                self.ram[ram_address]
            }

            _ => {
                panic!("Invalid RAM address {:6X}", address);
            }
        };

        result
    }

    #[allow(unused_variables)]
    fn write_8_rom(&mut self, address: u16, value: u8, caller: Caller) -> u8 {
        match address {
            // BOOT ROM
            0x0000..=0x0100 if self.is_booting => {
                panic!("UNIMPLEMENTED: BOOT ROM, cannot write address {:04X}.", address);
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
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc2, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc3 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc3, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc5 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc5, cannot write address {:04X}.", address);
                        }
                        Mbc::Huc1 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Huc1, cannot write address {:04X}.", address);
                        }
                    }
                } else {
                    panic!("UNIMPLEMENTED: No cartridge loaded, cannot write address {:04X}.", address);
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
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc2, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc3 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc3, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc5 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc5, cannot write address {:04X}.", address);
                        }
                        Mbc::Huc1 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Huc1, cannot write address {:04X}.", address);
                        }
                    }
                } else {
                    panic!("UNIMPLEMENTED: No cartridge loaded, cannot write address {:04X}.", address);
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
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc2, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc3 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc3, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc5 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc5, cannot write address {:04X}.", address);
                        }
                        Mbc::Huc1 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Huc1, cannot write address {:04X}.", address);
                        }
                    }
                } else {
                    panic!("UNIMPLEMENTED: No cartridge loaded, cannot write address {:04X}.", address);
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
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc2, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc3 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc3, cannot write address {:04X}.", address);
                        }
                        Mbc::Mbc5 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Mbc5, cannot write address {:04X}.", address);
                        }
                        Mbc::Huc1 => {
                            panic!("UNIMPLEMENTED: Unsupported cartridge type Huc1, cannot write address {:04X}.", address);
                        }
                    }
                } else {
                    panic!("UNIMPLEMENTED: No cartridge loaded, cannot write address {:04X}.", address);
                }
            }
            
            _ => {
                panic!("Invalid ROM address: {:6X}", address);
            }
        }

        // For debugging because conditional breakpoints are unuseably slow
        self.rom[address as usize]
    }
   
    fn write_8_ram(&mut self, address: u16, value: u8, caller: Caller) -> u8 {
        let ram_address = address as usize - 0x8000;

        match address {
            // VRAM
            0x8000..=0x9FFF => {
                if caller == Caller::PPU || self.ppu_mode != ppu::StatMode::PixelTransfer {
                    self.ram[ram_address] = value;
                } else {
                    println!("VRAM LOCKED by PPU: Cannot write address {:04X}.", address);
                }
            }

            // EXTERNAL RAM
            0xA000..=0xBFFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    cartridge.write_8_a000_bfff(address, value);
                } else {
                    self.ram[ram_address] = value;
                }
            }

            // INTERNAL RAM
            0xC000..=0xDFFF => {
                self.ram[ram_address] = value;
            }

            // ECHO RAM -- Mirrors C000-DDFF
            0xE000..=0xFDFF => {
                // let ram_address = address as usize - 0xA000; // ?
                self.ram[ram_address] = value;
            }

            // OAM RAM
            0xFE00..=0xFE9F => {
                if caller == Caller::PPU
                    || (self.ppu_mode != ppu::StatMode::OamSearch && self.ppu_mode != ppu::StatMode::PixelTransfer) {
                    self.ram[ram_address] = value;
                } else {
                    println!("OAM LOCKED by PPU: Cannot write address {:04X}.", address);
                }
            }

            // PROHIBITED -- Dr Mario explicitly clears this memory when starting
            0xFEA0..=0xFEFF => {
                // println!("PROHIBITED RAM: Not supposed to write address {:04X}.", address);
                self.ram[ram_address] = value;
            }

            // IO
            0xFF00..=0xFF7F => {
                match address {
                    // TIMER
                    DIV_REG => {
                        if caller == Caller::TIMER { // so that Timer can update DIV
                            self.ram[ram_address] = value;
                        } else {
                            // All writes to timer DIV register reset it to 0.
                            self.ram[ram_address] = 0;
                            self.sysclock = 0; // Internal clock is also reset(?)
                            // TODO How is TIMA affected...?
                        }
                    }
                    TIMA_REG => {
                        self.ram[ram_address] = value;
                    }
                    TMA_REG => {
                        self.ram[ram_address] = value;
                    }
                    TAC_REG => {
                        let written_value = value & 0b0111;

                        // TODO Does other timer registers also need to be updated on writes?
                        // let old_tac = self.read_8(address, caller);
                        // let tima_incr_is_enabled = Timer::is_tac_enabled(old_tac);
                        // let old_counter_bit = tima_incr_is_enabled && Timer::is_counter_bit_flipped(self.sysclock, old_tac);

                        self.ram[ram_address] = written_value;

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
                        let curr_value = self.ram[ram_address];
                        let adjusted_value = (value & 0xF0) | (curr_value & 0x0F);
                        self.ram[ram_address] = adjusted_value;
                    }

                    // PPU
                    LY_REG => {
                        if caller == Caller::PPU {
                            self.ram[ram_address] = value;
                        } else {
                            // Read-only
                        }
                    }
                    LCD_STATUS_REG => {
                        if caller == Caller::PPU {
                            self.ram[ram_address] = value;
                            self.ppu_mode = ppu::STAT_MODES[(value & 0x03) as usize]; // dr mario getting locked in non hblank...
                        } else {
                            // Bits 0,1,2 are read-only (only PPU can update)
                            let curr_value = self.ram[ram_address];
                            let adjusted_value = (value & 0xF8) | (curr_value & 0x07);
                            self.ram[ram_address] = adjusted_value;
                        }
                    }
                    LCD_CONTROL_REG => {
                        if self.ppu_mode == ppu::StatMode::VBlank {
                            println!("{:?} Writing CONTROL during VBlank: {:02X}", caller, value);
                            self.ram[ram_address] = value;
                        }
                        // else if caller == Caller::PPU {
                        //     println!("PPU writing CONTROL outside VBlank: {:02X}", value);
                        //     self.ram[ram_address] = value;
                        // }
                        else {
                            let curr_value = self.ram[ram_address];
                            let adjusted_value = value | (curr_value & 0x80); // Enforce not clearing bit 7
                            println!("{:?} Writing CONTROL outside VBlank, clearing bit 7 is not allowed: {:02X} -> {:02X}",
                                caller, value, adjusted_value);
                            self.ram[ram_address] = adjusted_value;
                        }
                    }
                    DMA_REG => {
                        self.ram[ram_address] = value;
                        self.oam_dma_src_addr = Option::from((value as u16) << 8);
                    }

                    // BANKING
                    BANK_REG => {
                        self.ram[ram_address] = value;
                        if self.is_booting && (value & 1) == 1 {
                            self.is_booting = false;
                        }
                    }

                    _ => {
                        self.ram[ram_address] = value;
                    }
                }
            }

            // HRAM, IE
            0xFF80..=0xFFFF => {
                self.ram[ram_address] = value;
            }
            
            _ => { 
                panic!("Invalid RAM address: {:6X}", address);
            }
        }

        self.ram[ram_address]
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
