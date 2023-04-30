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
use crate::console::cpu_registers::{CpuRegIndex};

pub(crate) struct Console {
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
    display: Display,
    input: Input,
    debugger: Option<Debugger>,
    is_paused: bool,
}

impl Console {
    pub(crate) fn new(window_title: &str, window_scale: u32, display_enabled: bool, debug: bool) -> Console {
        let debugger = if debug {
            Option::from(Debugger::new())
        } else {
            None
        };

        let sdl_context: Sdl = sdl2::init().unwrap();

        let mut mmu = Mmu::new();
        let cpu = Cpu::new(&mut mmu);
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
            is_paused: false,
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

    fn load_bootrom(&mut self) {
        let bootrom_filepath = "src/console/DMG_ROM.bin";
        let bootrom = read(bootrom_filepath).unwrap();
        self.mmu.load_rom(bootrom.as_ref(), 0, bootrom.len());
    }

    fn boot_empty(&mut self) {
        self.load_bootrom();
        self.main_loop();
    }

    fn run_game(&mut self, game: &Cartridge, skip_boot: bool) {
        if !skip_boot {
            self.load_bootrom();
            self.mmu.load_rom(&game.data[0x0100..], 0x0100, 0x8000 - 0x100);
        } else {
            self.mmu.load_rom(&game.data[0..], 0, 0x8000);
            self.cpu.registers.set_word(CpuRegIndex::PC, 0x0100);
        }

        self.main_loop();
    }

    fn debug_callback_handler(&mut self, debug_action: DebugAction) {
        if self.debugger.is_some() {
            let debugger = self.debugger.as_mut().unwrap();

            match debug_action {
                DebugAction::BREAK => {
                    debugger.break_or_cont(
                        Option::from(&mut self.cpu),
                        Option::from(&self.mmu),
                        Option::from(&self.ppu),
                        Option::from(HashMap::from([]))
                    );
                    self.is_paused = debugger.is_active();
                }
                DebugAction::PEEK => {
                    debugger.peek(
                        Option::from(&mut self.cpu),
                        Option::from(&self.mmu),
                        Option::from(&self.ppu),
                        Option::from(HashMap::from([]))
                    );
                }
                DebugAction::STEP => {
                    panic!("Unimplemented");
                }
                | _ => {}
            };
        }
    }

    fn debug_peek(&mut self) {
        if self.debugger.is_some() {
            let debugger = self.debugger.as_mut().unwrap();

            debugger.peek(
                Option::from(&mut self.cpu),
                Option::from(&self.mmu),
                Option::from(&self.ppu),
                Option::from(HashMap::from([]))
            );
        }
    }

    fn main_loop(&mut self) {
        loop {
            match self.input.poll() {
                CallbackAction::ESCAPE => {
                    self.debug_peek();
                    break;
                }
                CallbackAction::DEBUG(debug_action) => {
                    self.debug_callback_handler(debug_action);
                }
                _ => {
                    // default = step
                }
            }

            if !self.is_paused {
                let status = self.step();

                if status < 0 {
                    self.debug_peek();
                    panic!("Console step attempt failed with status {} at address {:#06X}.",
                        status, self.cpu.registers.get_word(CpuRegIndex::PC));
                }
            }
        }
    }

    fn step(&mut self) -> i16 {
        let mut cycles: i16 = self.cpu.step(&mut self.mmu);
        cycles += self.cpu.check_interrupts();

        self.ppu.step(cycles as u16, &mut self.cpu.interrupts, &mut self.mmu);
        if self.ppu.lcd_updated {
            self.display.draw(&mut self.mmu, &mut self.ppu);
        }

        cycles
    }
}
