use std::collections::HashMap;
use std::fs::read;
use std::env::current_dir;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use crate::cartridge::{
    cartridge::{Cartridge, CartridgeOption}
};
use crate::console::{
    cpu::Cpu,
    debugger::{DebugAction, Debugger},
    display::Display,
    input::{CallbackAction, Input},
    mmu::{Mmu},
    ppu::Ppu,
    registers::RegIndex
};

const DISPLAY_ENABLED: bool = true;
const SCREEN_PIXEL_WIDTH: u32 = 160;
const SCREEN_PIXEL_HEIGHT: u32 = 144;
const WINDOW_SCALE: u32 = 4;
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
        let debugger = Debugger::new(debug);
        let sdl_context: Sdl = sdl2::init().unwrap();

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
                SCREEN_PIXEL_WIDTH,
                SCREEN_PIXEL_HEIGHT,
                WINDOW_SCALE,
                WINDOW_TITLE,
                &sdl_context
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
        let bootrom_filepath = "src/console/DMG_ROM.bin";
        let bootrom = read(bootrom_filepath).unwrap();
        self.mmu.load_rom(bootrom.as_ref(), 0, bootrom.len());
    }

    fn run_game(&mut self, game: &Cartridge) {
        // TODO load roms correctly, map memory correctly /MBCs
        // self.boot();
        self.mmu.load_rom(&game.data[0x100..], 0x100, 0x8000 - 0x100);
        self.cpu.registers.set_word(RegIndex::PC, 0x100);
        self.main_loop();
    }

    fn boot_empty(&mut self) {
        self.boot();
        self.main_loop();
    }

    fn step(&mut self) -> i16 {
        let status = self.cpu.step(&mut self.mmu);
        if status >= 0 {
            // self.ppu.step();

            if DISPLAY_ENABLED {
                self.display.draw(&mut self.mmu, &mut self.ppu);
            }
        }

        status
    }

    fn main_loop(&mut self) {
        let mut is_paused: bool = false;

        loop {
            match self.input.poll() {
                CallbackAction::ESCAPE => {
                    break;
                }
                CallbackAction::DEBUG(debug_action) => {
                    match debug_action {
                        DebugAction::BREAK => {
                            self.debugger.break_or_cont(
                                Option::from(&mut self.cpu),
                                Option::from(&self.mmu),
                                Option::from(HashMap::from([("Locals", "test value")]))
                            );
                            is_paused = self.debugger.is_active();
                        }
                        DebugAction::PEEK => {
                            self.debugger.peek(
                                Option::from(&mut self.cpu),
                                Option::from(&self.mmu),
                                Option::from(HashMap::from([("Locals", "test value")]))
                            );
                        }
                        DebugAction::STEP
                        | _ => { }
                    }
                }
                _ => { }
            }

            if !is_paused {
                let status = self.step();
                if status < 0 {
                    self.debugger.peek(
                        Option::from(&mut self.cpu),
                        Option::from(&self.mmu),
                        Option::from(HashMap::from([("Locals", "test value")]))
                    );
                    panic!("Console step attempt failed with status {} at address {:#06X}.",
                        status, self.cpu.registers.get_word(RegIndex::PC));
                }
            }
        }
    }
}
