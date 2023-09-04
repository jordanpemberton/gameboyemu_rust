use std::ops::Add;
use std::time::{Duration, SystemTime};
use sdl2::Sdl;

use crate::cartridge::cartridge::Cartridge;
use crate::console::cpu::Cpu;
use crate::console::debugger::Debugger;
use crate::console::display::Display;
use crate::console::input::{Callback, Input};
use crate::console::mmu::Mmu;
use crate::console::ppu::Ppu;
use crate::console::cpu_registers::{CpuRegIndex};
use crate::console::interrupts::InterruptRegBit;
use crate::console::timer::Timer;

const CYCLES_PER_FRAME: u32 = 69905;
const FRAMES_PER_SECOND: u32 = 30;

pub(crate) struct Console {
    average_cycles_per_frame: u128,
    timer: Timer,
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
    input: Input,
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
        let mut sdl_context: Sdl = sdl2::init().unwrap();
        let input = Input::new(&mut sdl_context);

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
            average_cycles_per_frame: 0,
            timer: Timer::default(),
            cpu,
            mmu,
            ppu,
            input,
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

        let frame_duration = Duration::from_millis(1000 / FRAMES_PER_SECOND as u64);

        let (total_cycles, total_frames) = self.main_loop(frame_duration);

        self.average_cycles_per_frame = total_cycles / total_frames;
        // self.debug_print_screen();
        self.debug_peek();
    }

    fn debug_peek(&mut self) {
        match self.debugger {
            Some(ref mut debugger) => {
                debugger.peek(
                    Option::from(&self.cpu),
                    Option::from(&mut self.mmu),
                    Option::from(&self.timer),
                    Option::from(vec![
                        ("Expected cycles per frame", CYCLES_PER_FRAME.to_string().as_str()),
                        ("Average cycles per frame", self.average_cycles_per_frame.to_string().as_str()),
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

    fn input_polling(&mut self) -> bool {
        let callbacks = self.input.poll(&mut self.mmu);

        for callback in callbacks {
            match callback {
                Callback::DebugBreak => {
                    match self.debugger {
                        Some(ref mut debugger) => {
                            debugger.break_or_cont(
                                Option::from(&self.cpu),
                                Option::from(&mut self.mmu),
                                Option::from(&self.timer),
                                Option::from(vec![]));
                        }
                        None => {}
                    }
                }
                Callback::DebugPeek => {
                    match self.debugger {
                        Some(ref mut debugger) => {
                            debugger.peek(
                                Option::from(&self.cpu),
                                Option::from(&mut self.mmu),
                                Option::from(&self.timer),
                                Option::from(vec![]));
                        }
                        None => {}
                    }
                }
                Callback::DebugPrintScreen => {
                    match self.debugger {
                        Some(ref mut debugger) => debugger.print_screen_to_stdout(&self.ppu),
                        None => {}
                    }
                }
                Callback::Exit => {
                    // self.debug_print_screen();
                    self.debug_peek();
                    return false;
                }
                Callback::InputKeyStart
                | Callback::InputKeySelect
                | Callback::InputKeyA
                | Callback::InputKeyB
                | Callback::InputKeyUp
                | Callback::InputKeyDown
                | Callback::InputKeyLeft
                | Callback::InputKeyRight => {
                    self.cpu.interrupts.request(InterruptRegBit::Joypad, &mut self.mmu); // TODO where does this happen?
                }
            }
        }
        true
    }

    fn main_loop(&mut self, frame_duration: Duration) -> (u128, u128) {
        let mut cycles_this_frame: u32 = 0;
        let mut total_cycles: u128 = 0;
        let mut total_frames: u128 = 0;
        let mut last_time = SystemTime::now();
        let mut next_refresh_time = last_time.add(frame_duration);

        'mainloop: loop {
            // how to avoid calling every tick?
            last_time = SystemTime::now();

            if last_time >= next_refresh_time {
                // Time to draw and poll input
                self.display.draw(&mut self.ppu);

                if !self.input_polling() {
                    break 'mainloop;
                }

                total_cycles += cycles_this_frame as u128;
                total_frames += 1;
                cycles_this_frame = 0;
                next_refresh_time = last_time.add(frame_duration);
            } else if cycles_this_frame >= (CYCLES_PER_FRAME - 4) {
                // Wait till next update
                // sleep?
            } else if self.debugger.is_some() && self.debugger.as_mut().unwrap().active {
                // Currently paused for debugger but keep ticking
                cycles_this_frame += 4;
            } else {
                // TICK (CPU, Interrupts, PPU, Timer)
                let cycles = self.cpu.step(&mut self.mmu);

                if cycles < 0 {
                    // self.debug_print_screen();
                    self.debug_peek();
                    panic!("Console step attempt failed with status {} at address {:#06X}.",
                        cycles, self.cpu.registers.get_word(CpuRegIndex::PC));
                }

                // cycles += // TODO fix cycle adjustment?
                self.cpu.handle_interrupts(&mut self.mmu);

                self.ppu.step(cycles as u16, &mut self.cpu.interrupts, &mut self.mmu);

                if self.timer.step_2(&mut self.mmu, cycles as u8) {
                    self.cpu.interrupts.request(InterruptRegBit::Timer, &mut self.mmu);
                }

                cycles_this_frame += cycles as u32;
            }
        }

        (total_cycles, total_frames)
    }
}
