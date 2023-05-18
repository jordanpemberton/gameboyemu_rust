use std::collections::HashMap;
use std::fs::read;
use std::time::{Duration, SystemTime};
use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::itimerspec;

use crate::cartridge::cartridge::{Cartridge, CartridgeOption};
use crate::console::cpu::Cpu;
use crate::console::debugger::Debugger;
use crate::console::display::Display;
use crate::console::mmu::Mmu;
use crate::console::ppu::Ppu;
use crate::console::cpu_registers::{CpuRegIndex};
use crate::console::timer::Timer;

const TICKS_PER_FRAME: u16 = 500;
const TICKS_PER_POLL: u16 = 100;

pub(crate) struct Console {
    paused_for_debugger: bool,
    tick: u16,
    ticks_per_frame: u16,
    ticks_per_poll: u16,
    timer: Timer,
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
            paused_for_debugger: false,
            tick: 0,
            ticks_per_frame: TICKS_PER_FRAME,
            ticks_per_poll: TICKS_PER_POLL,
            timer: Timer::default(),
            cpu,
            mmu,
            ppu,
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
        self.mmu.write(bootrom.as_ref(), 0, bootrom.len());
    }

    fn boot_empty(&mut self) {
        self.load_bootrom();
        self.main_loop();
    }

    fn run_game(&mut self, game: &Cartridge, skip_boot: bool) {
        self.mmu.write(&game.data[0..], 0, 0x8000);
        if !skip_boot {
            self.load_bootrom();
        } else {
            self.cpu.registers.set_word(CpuRegIndex::AF, 0x01B0);
            self.cpu.registers.set_word(CpuRegIndex::BC, 0x0013);
            self.cpu.registers.set_word(CpuRegIndex::DE, 0x00D8);
            self.cpu.registers.set_word(CpuRegIndex::HL, 0x014D);
            self.cpu.registers.set_word(CpuRegIndex::SP, 0xFFFE);
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

    // returns if still active
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
                            self.paused_for_debugger = debugger.is_active();
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

    fn main_loop(&mut self) {
        const CYCLES_PER_UPDATE: i32 = 69905;
        let mut cycles_this_update: i32 = 0;

        const FFS: u128 = 60;
        let update_duration_ms: u128 = 1000 / FFS;
        let mut last_time = SystemTime::now();
        let mut delta_time: Duration = Duration::default();

        loop {
            if delta_time.as_millis() < update_duration_ms {
                let curr_time = SystemTime::now();
                delta_time += curr_time.duration_since(last_time).unwrap();
                last_time = curr_time;
                continue;
            }

            while cycles_this_update < CYCLES_PER_UPDATE {
                if self.paused_for_debugger {
                    cycles_this_update += 4; // keep ticking when paused
                } else {
                    let mut cycles = self.cpu.step(&mut self.mmu);
                    if cycles < 0 {
                        self.debug_print();
                        self.debug_peek();
                        panic!("Console step attempt failed with status {} at address {:#06X}.",
                            cycles, self.cpu.registers.get_word(CpuRegIndex::PC));
                    }

                    cycles += self.cpu.check_interrupts(); // TODO implement interrupts

                    // let timer_interrupt_requested = self.timer.step(&mut self.mmu); // TODO timer

                    self.ppu.step(cycles as u16, &mut self.cpu.interrupts, &mut self.mmu);

                    // TODO interrupts

                    cycles_this_update += cycles as i32;
                }
            }

            self.display.draw(&mut self.mmu, &mut self.ppu);
            if !self.poll() {
                self.debug_print();
                self.debug_peek();
                break;
            }
            cycles_this_update = 0;

            delta_time = Duration::default();
        }
    }
}
