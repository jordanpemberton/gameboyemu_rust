use std::ops::Add;
use std::time::{Duration, SystemTime};
use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::cartridge::cartridge::Cartridge;
use crate::console::cpu::Cpu;
use crate::console::debugger::Debugger;
use crate::console::display::Display;
use crate::console::mmu::Mmu;
use crate::console::ppu::Ppu;
use crate::console::cpu_registers::{CpuRegIndex};
use crate::console::interrupts::InterruptRegBit;
use crate::console::timer::Timer;

const CYCLES_PER_REFRESH: u32 = 69905;

pub(crate) struct Console {
    paused_for_debugger: bool,
    average_cycles_per_update: u128,
    timer: Timer,
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
    event_pump: EventPump,
    display: Display,
    debugger: Option<Debugger>,
}

impl Console {
    pub(crate) fn new(
            window_title: &str,
            window_scale: u32,
            debug: bool,
            cpu_debug_print: bool,
            debug_mode_display: bool,
            cartridge: Option<Cartridge>) -> Console {
        let mut mmu = Mmu::new(cartridge);
        let cpu = Cpu::new(&mut mmu, cpu_debug_print);
        let ppu = Ppu::new(&mut mmu, debug_mode_display);
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
            ppu.lcd.width,
            ppu.lcd.height);

        Console {
            paused_for_debugger: false,
            average_cycles_per_update: 0,
            timer: Timer::default(),
            cpu,
            mmu,
            ppu,
            event_pump: sdl_context.event_pump().unwrap(),
            display,
            debugger,
        }
    }

    pub(crate) fn run(&mut self, skip_boot: bool) {
        if skip_boot {
            self.cpu.registers.set_word(CpuRegIndex::AF, 0x01B0);
            self.cpu.registers.set_word(CpuRegIndex::BC, 0x0013);
            self.cpu.registers.set_word(CpuRegIndex::DE, 0x00D8);
            self.cpu.registers.set_word(CpuRegIndex::HL, 0x014D);
            self.cpu.registers.set_word(CpuRegIndex::SP, 0xFFFE);
            self.cpu.registers.set_word(CpuRegIndex::PC, 0x0100);
            self.mmu.is_booting = false;
        }

        self.main_loop();
    }

    fn debug_peek(&mut self) {
        match self.debugger {
            Some(ref mut debugger) => {
                debugger.peek(
                    Option::from(&self.cpu),
                    Option::from(&self.mmu),
                    Option::from(&self.timer),
                    Option::from(vec![
                        ("Expected cycles per update", CYCLES_PER_REFRESH.to_string().as_str()),
                        ("Average cycles per update", self.average_cycles_per_update.to_string().as_str()),
                    ]));
            }
            None => {}
        }
    }

    #[allow(dead_code)]
    fn debug_print_screen(&mut self) {
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
                                Option::from(&self.timer),
                                Option::from(vec![]));
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
                                Option::from(&self.timer),
                                Option::from(vec![]));
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
        const FFS: u64 = 60;
        let update_duration = Duration::from_millis(1000 / FFS);
        let mut cycles_this_update: u32 = 0;
        let mut total_cycles: u128 = 0;
        let mut total_updates: u128 = 0;
        let mut next_update_time = SystemTime::now().add(update_duration);

        loop {
            while cycles_this_update < CYCLES_PER_REFRESH - 4 && SystemTime::now() < next_update_time {
                if self.paused_for_debugger {
                    cycles_this_update += 4; // keep ticking when paused
                } else {
                    let mut cycles = self.cpu.step(&mut self.mmu);

                    if cycles < 0 {
                        // self.debug_print_screen();
                        self.debug_peek();
                        panic!("Console step attempt failed with status {} at address {:#06X}.",
                            cycles, self.cpu.registers.get_word(CpuRegIndex::PC));
                    }

                    cycles += self.cpu.check_interrupts(&mut self.mmu);

                    self.ppu.step(cycles as u16, &mut self.cpu.interrupts, &mut self.mmu);

                    if self.timer.step(&mut self.mmu, cycles as u8) {
                        self.cpu.interrupts.request(InterruptRegBit::Timer, &mut self.mmu);
                    }

                    cycles_this_update += cycles as u32;
                }
            }

            if SystemTime::now() >= next_update_time {
                self.display.draw(&mut self.ppu);
                if !self.poll() {
                    // self.debug_print_screen();
                    self.debug_peek();
                    break;
                }
                total_cycles += cycles_this_update as u128;
                total_updates += 1;
                cycles_this_update = 0;
                next_update_time = SystemTime::now().add(update_duration);
            }
        }

        self.average_cycles_per_update = total_cycles / total_updates;
        // self.debug_print_screen();
        self.debug_peek();
    }
}
