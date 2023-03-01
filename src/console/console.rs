use std::fs::read;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use crate::cartridge::cartridge::{Cartridge, CartridgeOption};
use crate::console::cpu::Cpu;
use crate::console::display::Display;
use crate::console::input::{CallbackAction, Input};
use crate::console::mmu::Mmu;
use crate::console::registers::RegIndex;

const WINDOW_WIDTH: u32 = 300;
const WINDOW_HEIGHT: u32 = 500;
const WINDOW_TITLE: &str = "_GAMBOY_";

pub(crate) struct Console {
    cpu: Cpu,
    mmu: Mmu,
    display: Display,
    input: Input,
}

impl Console {
    pub(crate) fn new() -> Console {
        let sdl_context: Sdl = sdl2::init().unwrap();

        Console {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
            display: Display::new(&sdl_context, WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE),
            input: Input::new(&sdl_context),
        }
    }

    pub(crate) fn run(&mut self, cartridge: CartridgeOption) {
        match cartridge {
            CartridgeOption::NONE => {
                self.boot_empty();
            }
            CartridgeOption::SOME(cartridge) => {
                self.run_game(&cartridge);
            }
        }
    }

    fn boot(&mut self) {
        // TODO load roms correctly, map memory correctly /MBCs
        let bootrom_filepath = "/home/jordan/RustProjs/GameBoyEmu/src/console/DMG_ROM.bin";
        let bootrom = read(bootrom_filepath).unwrap();
        self.mmu.load_rom(bootrom.as_ref(), 0, bootrom.len());
    }

    fn run_game(&mut self, game: &Cartridge) {
        // TODO load roms correctly, map memory correctly /MBCs
        self.boot();
        self.mmu.load_rom(&game.data[0x100..], 0x100, 0x8000 - 0x100);
        // self.cpu.registers.set_word(RegIndex::PC, 0x100);
        self.main_loop();
    }

    fn boot_empty(&mut self) {
        self.boot();
        self.main_loop();
    }

    fn main_loop(&mut self) {
        let mut max_pc: u16 = 0; // for debugging

        loop {
            let action = self.input.poll();
            match action {
                CallbackAction::ESCAPE => {
                    break;
                }
                CallbackAction::STEP => {
                    let pc = self.cpu.registers.get_word(RegIndex::PC);
                    if pc > max_pc {
                        max_pc = pc;
                    }
                    self.cpu.step(&mut self.mmu);
                }
                _ => { }
            }
        }

        println!("\nMax address reached in program: {:#06X}", max_pc);
    }
}
