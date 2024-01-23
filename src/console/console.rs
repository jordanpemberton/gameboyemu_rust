use std::thread::sleep;
use std::time::{Duration, Instant};
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
use crate::console::mmu;
use crate::console::timer::Timer;

const CYCLES_PER_FRAME: u64 = 69905;
const FRAMES_PER_SECOND: u64 = 60;

pub(crate) struct Console {
    cycles: i16,
    skip_boot: bool,
    timer: Timer,
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
    input: Input,
    display: Display,
    debugger: Option<Debugger>,
    // perf
    total_cycles: u128,
    total_frames: u128,
    total_runtime: u128,
}

impl Console {
    pub(crate) fn new(
            window_title: &str,
            window_scale: u32,
            debug: bool,
            cpu_debug_print: bool,
            skip_boot: bool,
            cartridge: Option<Cartridge>) -> Console {
        let timer = Timer::new();
        let mmu = Mmu::new(cartridge, skip_boot);
        let cpu = Cpu::new(cpu_debug_print);
        let ppu = Ppu::new();
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
            cycles: 0,
            skip_boot,
            timer,
            cpu,
            mmu,
            ppu,
            input,
            display,
            debugger,
            total_cycles: 0,
            total_frames: 0,
            total_runtime: 0,
        }
    }

    pub(crate) fn run(&mut self) {
        if self.skip_boot {
            self.cpu.registers.set_word(CpuRegIndex::AF, 0x01B0);
            self.cpu.registers.set_word(CpuRegIndex::BC, 0x0013);
            self.cpu.registers.set_word(CpuRegIndex::DE, 0x00D8);
            self.cpu.registers.set_word(CpuRegIndex::HL, 0x014D);
            self.cpu.registers.set_word(CpuRegIndex::SP, 0xFFFE);
            self.cpu.registers.set_word(CpuRegIndex::PC, 0x0100);
            self.mmu.write_8(mmu::LCD_CONTROL_REG, 0x91); // Enable LCD
            self.mmu.is_booting = false;
        }

        self.main_loop();

        // self.debug_print_screen();
        self.debug_peek();
    }

    fn debug_peek(&mut self) {
        match self.debugger {
            Some(ref mut debugger) => {
                debugger.peek(
                    Option::from(&mut self.cpu),
                    Option::from(&mut self.mmu),
                    Option::from(&mut self.timer),
                    Option::from(vec![
                        ("Total cycles", self.total_cycles.to_string().as_str()),
                        ("Total frames", self.total_frames.to_string().as_str()),
                        ("Total runtime (secs)", self.total_runtime.to_string().as_str()),
                        ("Average cycles per frame", (self.total_cycles / self.total_frames).to_string().as_str()),
                        ("Expected cycles per frame", CYCLES_PER_FRAME.to_string().as_str()),
                        ("Average frames per second", (self.total_frames /
                            if self.total_runtime > 0 { self.total_runtime } else { 1 } ).to_string().as_str()),
                        ("Expected frames per second", FRAMES_PER_SECOND.to_string().as_str()),
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
                                Option::from(&mut self.cpu),
                                Option::from(&mut self.mmu),
                                Option::from(&mut self.timer),
                                Option::from(vec![]));
                        }
                        None => {}
                    }
                }
                Callback::DebugPeek => {
                    match self.debugger {
                        Some(ref mut debugger) => {
                            debugger.peek(
                                Option::from(&mut self.cpu),
                                Option::from(&mut self.mmu),
                                Option::from(&mut self.timer),
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

    fn main_tick(&mut self) -> u16 {
        // DEBUG
        if self.debugger.is_some() && self.debugger.as_mut().unwrap().active {
            return 4;
        }

        // TIMER (using previous cycles delta)
        if self.timer.step(&mut self.mmu, self.cycles as u8) {
            self.cpu.interrupts.request(InterruptRegBit::Timer, &mut self.mmu);
        }

        // PPU - OAM DMA transfer
        self.ppu.oam_dma(&mut self.mmu);

        // PPU - step
        self.ppu.step(self.cycles as u16, &mut self.cpu.interrupts, &mut self.mmu);

        // CPU - step (execute instruction)
        self.cycles = self.cpu.step(&mut self.mmu);

        if self.cycles < 0 {
            // self.debug_print_screen();
            self.debug_peek();
            panic!("Console step attempt failed with status {} at address {:#06X}.",
                self.cycles, self.cpu.registers.get_word(CpuRegIndex::PC));
        }

        // CPU - interrupts
        self.cycles += self.cpu.handle_interrupts(&mut self.mmu);

        self.cycles as u16
    }

    fn main_loop(&mut self) {
        let mut is_running = true;
        let start_time = Instant::now();
        let mut cycles_this_frame = 0;

        while is_running {
            if cycles_this_frame >= CYCLES_PER_FRAME {
                self.display.draw(&mut self.ppu);
                is_running = self.input_polling();

                self.total_frames += 1;
                self.total_cycles += cycles_this_frame as u128;
                cycles_this_frame = 0;
            } else {
                cycles_this_frame += self.main_tick() as u64;
            }
        }

        self.total_runtime = start_time.elapsed().as_secs() as u128;
    }

    // TODO Correct speed /framerate
    fn _main_loop(&mut self) {
        let frame_duration = Duration::from_millis(1000 / FRAMES_PER_SECOND); // Why is it always so slow?
        let start_time = Instant::now();
        let mut cycles_this_frame: u64 = 0;
        let mut frame_start_time = Instant::now();
        let sleep_duration = Duration::from_nanos(100);

        'mainloop: loop {
            if self.debugger.is_some() && self.debugger.as_mut().unwrap().active {
                // PAUSED FOR DEBUGGER
                cycles_this_frame += 4; // this allows input polling
            } else if frame_start_time.elapsed() < frame_duration {
                if cycles_this_frame < CYCLES_PER_FRAME {
                    // MAIN TICK
                    cycles_this_frame += self.main_tick() as u64;
                } else {
                    // WAIT
                    sleep(sleep_duration);
                }
            } else {
                // DRAW + POLL
                self.display.draw(&mut self.ppu);
                self.total_frames += 1;

                if !self.input_polling() {
                    break 'mainloop;
                }

                self.total_cycles += cycles_this_frame as u128;
                frame_start_time = Instant::now();
                cycles_this_frame = 0;
            }
        }

        let runtime = start_time.elapsed();
        self.total_runtime = runtime.as_secs() as u128;
    }
}
