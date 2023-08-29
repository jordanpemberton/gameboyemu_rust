use std::fmt::{Display, Formatter};
use crate::console::mmu;
use crate::console::mmu::Mmu;

#[allow(dead_code)]
#[derive(Default)]
pub(crate) struct Timer {
    div: u8,         // FF04 — DIV: Divider register
    counter: u8,     // FF05 — TIMA: Timer counter
    modulo: u8,      // FF06 — TMA: Timer modulo
    control: u8,     // FF07 — TAC: Timer control
    overflow: bool,
    // Bit  2   - Timer Enable
    // Bits 1-0 - Input Clock Select
    // 00: CPU Clock / 1024 (DMG, SGB2, CGB Single Speed Mode:   4096 Hz, SGB1:   ~4194 Hz, CGB Double Speed Mode:   8192 Hz)
    // 01: CPU Clock / 16   (DMG, SGB2, CGB Single Speed Mode: 262144 Hz, SGB1: ~268400 Hz, CGB Double Speed Mode: 524288 Hz)
    // 10: CPU Clock / 64   (DMG, SGB2, CGB Single Speed Mode:  65536 Hz, SGB1:  ~67110 Hz, CGB Double Speed Mode: 131072 Hz)
    // 11: CPU Clock / 256  (DMG, SGB2, CGB Single Speed Mode:  16384 Hz, SGB1:  ~16780 Hz, CGB Double Speed Mode:  32768 Hz)
    tim_clocks: usize,
    div_clocks: usize,
}

impl Timer {
    // returns if timer interrupt requested
    #[allow(dead_code)]
    pub(crate) fn step(&mut self, mmu: &mut Mmu, cycles: u8) -> bool {
        let mut request_interrupt = false;
        self.refresh_from_mem(mmu);
        self.inc_div(mmu, cycles); // TODO fix
        let clocks = self.selected_clocks();
        if self.is_enabled() && self.div as u16 >= clocks {
            request_interrupt = self.inc_counter(mmu, cycles);
        }
        request_interrupt
    }

    #[allow(unused_variables)]
    pub(crate) fn step_2(&mut self, mmu: &mut Mmu, cycles: u8) -> bool {
        let mut request_interrupt = false;
        self.refresh_from_mem(mmu);
        if self.div_clocks < (cycles as usize) {
            self.div = self.div.wrapping_add(1);
            let rem = (cycles as usize) - self.div_clocks;
            self.div_clocks = 256 - rem;
        } else {
            self.div_clocks -= cycles as usize
        }
        if self.control & 0x04 == 0 {
            return request_interrupt;
        }
        if self.tim_clocks < cycles as usize {
            let mut rem = (cycles as usize) - self.tim_clocks;
            loop {
                let (tim, of) = self.counter.overflowing_add(1);
                self.counter = tim;
                if of {
                    self.counter = self.modulo;
                    request_interrupt = true;
                }
                self.tim_clocks = match self.control & 0x3 { // reset
                    0x0 => 1024, // 4096Hz = 1024 cpu clocks
                    0x1 => 16,   // 262144Hz = 16 cpu clocks
                    0x2 => 64,   // 65536Hz = 64 cpu clocks
                    0x3 => 256,  // 16384Hz = 256 cpu clocks
                    _ => unreachable!(),
                };
                if rem <= self.tim_clocks {
                    self.tim_clocks -= rem;
                    break;
                }
                rem -= self.tim_clocks;
            }
        } else {
            self.tim_clocks -= cycles as usize;
        }

        mmu.write_8(mmu::DIV_REG, self.div);
        mmu.write_8(mmu::TIMA_REG, self.counter);
        request_interrupt
    }

    #[allow(dead_code)]
    pub(crate) fn step_3(&mut self, mmu: &mut Mmu, cycles: u8) -> bool {
        let mut request_interrupt = false;
        self.refresh_from_mem(mmu);

        if self.overflow {
            self.inc_div(mmu, cycles);
            self.counter = self.modulo;
            request_interrupt = true;
            self.overflow = false;
        } else if self.is_enabled() && (self.div as u16) < self.selected_clocks() { // ?
            self.inc_div(mmu, cycles);
            if self.div as u16 >= self.selected_clocks() {
                (self.counter, self.overflow) = self.counter.overflowing_add(cycles);
            }
        } else {
            self.inc_div(mmu, cycles);
        }

        mmu.write_8(mmu::DIV_REG, self.div);
        mmu.write_8(mmu::TIMA_REG, self.counter);

        request_interrupt
    }

    fn refresh_from_mem(&mut self, mmu: &mut Mmu) {
        self.div = mmu.read_8(mmu::DIV_REG);
        self.counter = mmu.read_8(mmu::TIMA_REG);
        self.modulo = mmu.read_8(mmu::TMA_REG);
        self.control = mmu.read_8(mmu::TAC_REG);
    }

    fn is_enabled(&self) -> bool {
        self.control & 0b0100 == 0b0100
    }

    fn selected_clocks(&self) -> u16 {
        match self.control & 0b0011 {
            0b00 => 1 << 10, // 1024,   // vs 1<<7, 128?
            0b01 => 1 << 4,  // 16,     // vs 1<<1, 1?
            0b10 => 1 << 6,  // 64,     // vs 1<<3, 8?
            0b11 => 1 << 8,  // 256,    // vs 1<<5, 32?
            _ => unreachable!()
        }
    }

    // TODO failing mooneye test acceptance/div_timing
    #[allow(dead_code)]
    fn inc_div(&mut self, mmu: &mut Mmu, increment_by: u8) {
        // Increment Div (unless STOP instruction was run)
        // Does anything happen when it overflows /wraps?
        self.div = self.div.wrapping_add(increment_by);

        mmu.write_8(mmu::DIV_REG, self.div);
    }

    #[allow(dead_code)]
    fn inc_counter(&mut self, mmu: &mut Mmu, increment_by: u8) -> bool {
        // Increment counter by selected clock frequency
        let (counter, overflow) = self.counter.overflowing_add(increment_by);

        // If counter overflows, request interrupt and reset counter = modulo
        self.counter = if overflow {
            self.modulo
        } else {
            counter
        };

        mmu.write_8(mmu::TIMA_REG, self.counter);

        overflow
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,
            "\tDIV:            {:#04X}\n\
            \tTIMA (counter): {:#04X}\n\
            \tTMA (modulo):   {:#04X}\n\
            \tTAC (control):  {:#04X}",
            self.div,
            self.counter,
            self.modulo,
            self.control,
        )
    }
}
