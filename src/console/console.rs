use std::collections::HashMap;
use std::fs::read;
use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::cartridge::cartridge::{Cartridge, CartridgeOption};
use crate::console::cpu::Cpu;
use crate::console::debugger::Debugger;
use crate::console::display::Display;
use crate::console::mmu::Mmu;
use crate::console::ppu::Ppu;
use crate::console::cpu_registers::{CpuRegIndex};

const TICKS_PER_FRAME: u16 = 500;
const TICKS_PER_POLL: u16 = 100;

pub(crate) struct Console {
    is_paused: bool,
    tick: u16,
    ticks_per_frame: u16,
    ticks_per_poll: u16,
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
    event_pump: EventPump,
    display: Display,
    debugger: Option<Debugger>,
}

impl Console {
    pub(crate) fn new(window_title: &str, window_scale: u32, display_enabled: bool, debug: bool, cpu_debug_print: bool) -> Console {
        let mut mmu = Mmu::new();
        let cpu = Cpu::new(&mut mmu, cpu_debug_print);
        let ppu = Ppu::new(&mut mmu);
        let sdl_context: Sdl = sdl2::init().unwrap();

        let debugger = if debug {
            Option::from(Debugger::new())
        } else {
            None
        };

        let display = Display::new(
            window_scale,
            window_title,
            &sdl_context,
            display_enabled);

        Console {
            is_paused: false,
            tick: 0,
            ticks_per_frame: TICKS_PER_FRAME,
            ticks_per_poll: TICKS_PER_POLL,
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
        self.mmu.load_rom(&game.data[0..], 0, 0x8000);
        if !skip_boot {
            self.load_bootrom();
        } else {
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

    fn debug_print(&mut self) {
        match self.debugger {
            Some(ref mut debugger) => debugger.print_screen_to_stdout(&self.ppu),
            None => {}
        }
    }

    fn poll(&mut self) -> bool {
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
                            self.is_paused = debugger.is_active();
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
                        None => { }
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    match self.debugger {
                        Some(ref mut debugger) => debugger.print_screen_to_stdout(&self.ppu),
                        None => { }
                    }
                }
                _ => {}
            }
        }

        true
    }

    fn step(&mut self) -> i16 {
        let mut cycles = self.cpu.step(&mut self.mmu);
        cycles += self.cpu.check_interrupts();
        self.ppu.step(cycles as u16, &mut self.cpu.interrupts, &mut self.mmu);
        if self.ppu.lcd_updated && self.tick % self.ticks_per_frame == 0 {
            self.display.draw(&mut self.mmu, &mut self.ppu);
        }
        cycles
    }

    fn main_loop(&mut self) {
        loop {
            if self.tick % self.ticks_per_poll == 0 {
                let exit = !self.poll();
                if exit {
                    self.debug_print();
                    self.debug_peek();
                    break;
                }
            }

            if !self.is_paused {
                let status = self.step();
                if status < 0 {
                    self.debug_print();
                    self.debug_peek();
                    panic!("Console step attempt failed with status {} at address {:#06X}.",
                        status, self.cpu.registers.get_word(CpuRegIndex::PC));
                }
            }

            self.tick = self.tick.wrapping_add(1);
        }
    }
}
