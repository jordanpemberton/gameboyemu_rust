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
    tima_clocks: u16,
    div_clocks: u8,
}

impl Timer {
    #[allow(unused_variables)]
    pub(crate) fn step(&mut self, mmu: &mut Mmu, cycles: u8) -> bool { // passes all blargg CPU tests
        let mut request_interrupt = false;
        self.refresh_from_mem(mmu);

        // Increment DIV
        if self.div_clocks < cycles {
            self.div = self.div.wrapping_add(1);
            let remainder = cycles - self.div_clocks;
            self.div_clocks = (0u8).wrapping_sub(remainder);
        } else {
            self.div_clocks -= cycles;
        }
        mmu.write_8(mmu::DIV_REG, self.div);

        if self.is_enabled() {
            if self.tima_clocks < cycles as u16 {
                let mut remainder = cycles as u16 - self.tima_clocks;

                // Increment counter (TIMA)
                loop {
                    request_interrupt |= self.inc_counter(mmu, 1);
                    self.tima_clocks = self.selected_clocks();

                    if remainder <= self.tima_clocks {
                        self.tima_clocks -= remainder;
                        break;
                    } else {
                        remainder -= self.tima_clocks;
                    }
                }
            } else {
                self.tima_clocks -= cycles as u16;
            }
        }

        request_interrupt
    }

    #[allow(dead_code)]
    pub(crate) fn step_2(&mut self, mmu: &mut Mmu, cycles: u8) -> bool {  // Fails blargg CPU test 2 (#5)
        let mut request_interrupt = false;
        self.refresh_from_mem(mmu);
        self.inc_div(mmu, cycles); // TODO fix
        let clocks = self.selected_clocks();
        if self.is_enabled() && self.div as u16 >= clocks {
            request_interrupt = self.inc_counter(mmu, cycles);
        }
        request_interrupt
    }

    #[allow(dead_code)]
    pub(crate) fn step_3(&mut self, mmu: &mut Mmu, cycles: u8) -> bool {  // Fails blargg CPU test 2 (#5)
        let mut request_interrupt = false;
        self.refresh_from_mem(mmu);

        if self.overflow {
            self.inc_div(mmu, cycles);
            self.counter = self.modulo;
            request_interrupt = true;
            self.overflow = false;
        } else if self.is_enabled() && (self.div as u16) < self.selected_clocks() { // ?
            self.inc_div(mmu, cycles);
            if self.div as u16 >= self.selected_clocks() { // TODO, dead branch
                (self.counter, self.overflow) = self.counter.overflowing_add(cycles);
            }
        } else {
            self.inc_div(mmu, cycles);
        }

        mmu.write_8(mmu::TIMA_REG, self.counter);

        request_interrupt
    }

    fn refresh_from_mem(&mut self, mmu: &mut Mmu) {
        self.div = mmu.read_8(mmu::DIV_REG);
        self.counter = mmu.read_8(mmu::TIMA_REG);
        self.modulo = mmu.read_8(mmu::TMA_REG);
        self.control = mmu.read_8(mmu::TAC_REG);
    }

    // Control (FF07) Bit 2 = Timer Enable
    fn is_enabled(&self) -> bool {
        self.control & 0b0100 == 0b0100
    }

    // Control (FF07) Bits 1-0 = Input Clock Select
    // 00: CPU Clock / 1024 (DMG, SGB2, CGB Single Speed Mode:   4096 Hz, SGB1:   ~4194 Hz, CGB Double Speed Mode:   8192 Hz)
    // 01: CPU Clock / 16   (DMG, SGB2, CGB Single Speed Mode: 262144 Hz, SGB1: ~268400 Hz, CGB Double Speed Mode: 524288 Hz)
    // 10: CPU Clock / 64   (DMG, SGB2, CGB Single Speed Mode:  65536 Hz, SGB1:  ~67110 Hz, CGB Double Speed Mode: 131072 Hz)
    // 11: CPU Clock / 256  (DMG, SGB2, CGB Single Speed Mode:  16384 Hz, SGB1:  ~16780 Hz, CGB Double Speed Mode:  32768 Hz)
    fn selected_clocks(&self) -> u16 {
        match self.control & 0b0011 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!()
        }
    }

    // TODO failing mooneye test acceptance/div_timing
    #[allow(dead_code)]
    fn inc_div(&mut self, mmu: &mut Mmu, increment_by: u8) {
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
