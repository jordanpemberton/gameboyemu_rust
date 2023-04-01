use std::collections::HashMap;
use std::fs::read;
use std::env::current_dir;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use crate::cartridge::cartridge::{Cartridge, CartridgeOption};

use crate::console::cpu::{Cpu};
use crate::console::debugger::{DebugAction, Debugger};
use crate::console::display::{Display};
use crate::console::input::{CallbackAction, Input};
use crate::console::interrupts::{Interrupts};
use crate::console::mmu::{Mmu};
use crate::console::ppu::{LCD_PIXEL_WIDTH, LCD_PIXEL_HEIGHT, Ppu};
use crate::console::registers::{RegIndex};

pub(crate) struct Console {
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
    display: Display,
    input: Input,
    debugger: Debugger,
}

impl Console {
    pub(crate) fn new(window_title: &str, window_scale: u32, display_enabled: bool, debug: bool) -> Console {
        let debugger = Debugger::new(debug);
        let sdl_context: Sdl = sdl2::init().unwrap();

        let cpu = Cpu::new();
        let mut mmu = Mmu::new();
        let ppu = Ppu::new(&mut mmu);

        Console {
            cpu: cpu,
            mmu: mmu,
            ppu: ppu,
            display: Display::new(
                LCD_PIXEL_WIDTH,
                LCD_PIXEL_HEIGHT,
                window_scale,
                window_title,
                &sdl_context,
                display_enabled
            ),
            input: Input::new(&sdl_context),
            debugger: debugger,
        }
    }

    pub(crate) fn run(&mut self, cartridge: CartridgeOption, skip_boot: bool) {
        match cartridge {
            CartridgeOption::NONE => {
                if !skip_boot {
                    self.boot_empty();
                }
            }
            CartridgeOption::SOME(cartridge) => {
                self.run_game(&cartridge, skip_boot);
            }
        }
    }

    fn boot(&mut self) {
        // TODO load roms correctly, map memory correctly /MBCs
        let bootrom_filepath = "src/console/DMG_ROM.bin";
        let bootrom = read(bootrom_filepath).unwrap();
        self.mmu.load_rom(bootrom.as_ref(), 0, bootrom.len());
    }

    fn run_game(&mut self, game: &Cartridge, skip_boot: bool) {
        if !skip_boot {
            self.boot();
            self.mmu.load_rom(&game.data[0x0100..], 0x0100, 0x8000 - 0x100);
        } else {
            self.mmu.load_rom(&game.data[0..], 0, 0x8000);
            self.cpu.registers.set_word(RegIndex::PC, 0x0100);
        }

        self.main_loop();
    }

    fn boot_empty(&mut self) {
        self.boot();
        self.main_loop();
    }

    fn step(&mut self) -> i16 {
        let mut cycles: i16 = self.cpu.step(&mut self.mmu);
        cycles += self.cpu.check_interrupt();

        // TODO
        let interrupts = Interrupts::new();

        self.ppu.step(&interrupts, &mut self.mmu);
        self.display.draw(&mut self.mmu, &mut self.ppu);

        cycles
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
