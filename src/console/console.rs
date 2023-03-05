use std::fs::read;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    Sdl
};

use crate::cartridge::{
    cartridge::{Cartridge, CartridgeOption}
};
use crate::console::{
    cpu::Cpu,
    debugger::{DebugAction, Debugger},
    display::Display,
    input::{CallbackAction, Input},
    mmu::{MemoryType, Mmu},
    ppu::Ppu,
    registers::RegIndex
};

const SCREEN_PIXEL_WIDTH: u32 = 300;
const SCREEN_PIXEL_HEIGHT: u32 = 500;
const WINDOW_SCALE: u32 = 10;
const WINDOW_TITLE: &str = "_GAMBOY_";

pub(crate) struct Console {
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
    display: Display,
    input: Input,
    debugger: Debugger,
}

impl Console {
    pub(crate) fn new(debug: bool) -> Console {
        let sdl_context: Sdl = sdl2::init().unwrap();
        let debugger = Debugger::new(debug);

        Console {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
            ppu: Ppu::new([
                [0,1,2,3],
                [0,1,2,3],
                [0,1,2,3],
                [0,1,2,3],
            ]),
            display: Display::new(
                &sdl_context,
                SCREEN_PIXEL_WIDTH,
                SCREEN_PIXEL_HEIGHT,
                WINDOW_SCALE,
                WINDOW_TITLE
            ),
            input: Input::new(&sdl_context),
            debugger: debugger,
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

    fn step(&mut self) {
        self.cpu.step(&mut self.mmu, &mut self.debugger);

        // self.ppu.step();

        let vram = self.mmu.get_memory_buffer(&MemoryType::VRAM);
        let pixel_buffer = self.ppu.get_pixel_buffer(vram, 0);
        self.display.draw_screen(pixel_buffer);
    }

    fn main_loop(&mut self) {
        loop {
            let action = self.input.poll();
            match action {
                CallbackAction::ESCAPE => {
                    break;
                }
                CallbackAction::DEBUG(debug_action) => {
                    match debug_action {
                        DebugAction::STEP => {
                            self.debugger.toggle_stepping();
                            self.step();
                        }
                        _ => { }
                    }
                }
                CallbackAction::STEP => {
                    self.step();
                }
                _ => { }
            }
        }
    }
}
