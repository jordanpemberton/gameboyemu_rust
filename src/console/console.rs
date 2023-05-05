use std::collections::HashMap;
use std::fs::read;
use std::env::current_dir;
use std::path::Path;

use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::cartridge::cartridge::{Cartridge, CartridgeOption};

use crate::console::cpu::{Cpu};
use crate::console::debugger::{DebugAction, Debugger};
use crate::console::display::{Display};
use crate::console::interrupts::{Interrupts};
use crate::console::mmu::{Mmu};
use crate::console::ppu::{LCD_PIXEL_WIDTH, LCD_PIXEL_HEIGHT, Ppu};
use crate::console::cpu_registers::{CpuRegIndex};

pub(crate) struct Console {
    is_paused: bool,
    clocks: u16,
    polling_speed: u16,
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
    event_pump: EventPump,
    display: Display,
    debugger: Option<Debugger>,
}

impl Console {
    pub(crate) fn new(window_title: &str, window_scale: u32, display_enabled: bool, debug: bool) -> Console {
        let mut mmu = Mmu::new();
        let cpu = Cpu::new(&mut mmu);
        let ppu = Ppu::new(&mut mmu);
        let sdl_context: Sdl = sdl2::init().unwrap();

        let debugger = if debug {
            Option::from(Debugger::new())
        } else {
            None
        };

        let display = Display::new(
            LCD_PIXEL_WIDTH as u32,
            LCD_PIXEL_HEIGHT as u32,
            window_scale,
            window_title,
            &sdl_context,
            display_enabled);

        Console {
            is_paused: false,
            clocks: 0,
            polling_speed: 1000,
            cpu: cpu,
            mmu: mmu,
            ppu: ppu,
            event_pump: sdl_context.event_pump().unwrap(),
            display,
            debugger,
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

    fn debug_peek(&mut self) {
        match self.debugger {
            Some(ref mut debugger) => {
                debugger.peek(
                    Option::from(&self.cpu),
                    Option::from(&self.mmu),
                    Option::from(&self.ppu),
                    Option::from(HashMap::from([])));
            }
            None => {}
        }
    }

    fn poll(&mut self) -> bool {
        if self.clocks >= self.polling_speed {
            self.clocks = 0;

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        return false;
                    }
                    Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                        match self.debugger {
                            Some(ref mut debugger) => {
                                debugger.break_or_cont(
                                    Option::from(&self.cpu),
                                    Option::from(&self.mmu),
                                    Option::from(&self.ppu),
                                    Option::from(HashMap::from([])));
                            }
                            None => {}
                        }
                    }
                    Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                        match self.debugger {
                            Some(ref mut debugger) => {
                                debugger.peek(
                                    Option::from(&self.cpu),
                                    Option::from(&self.mmu),
                                    Option::from(&self.ppu),
                                    Option::from(HashMap::from([])));
                            }
                            None => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        true
    }

    fn step(&mut self) -> i16 {
        let mut cycles = self.cpu.step(&mut self.mmu);
        if cycles >= 0 {
            cycles += self.cpu.check_interrupts();
            self.clocks += cycles as u16;

            self.ppu.step(cycles as u16, &mut self.cpu.interrupts, &mut self.mmu);
            if self.ppu.lcd_updated {
                self.display.draw(&mut self.mmu, &mut self.ppu);
            }
        }

        cycles
    }

    fn main_loop(&mut self) {
        loop {
            if !self.is_paused {
                let exit = !self.poll();
                if exit {
                    self.debug_peek();
                    break;
                }

                let status = self.step();
                if status < 0 {
                    self.debug_peek();
                    panic!("Console step attempt failed with status {} at address {:#06X}.",
                        status, self.cpu.registers.get_word(CpuRegIndex::PC));
                }
            }
        }
    }
}
