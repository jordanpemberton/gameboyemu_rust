use std::fmt::{Display, Formatter};
use crate::console::mmu::Mmu;

pub(crate) const DIV_REG_ADDRESS: u16 = 0xFF04;
const TIMA_REG_ADDRESS: u16 = 0xFF05;
const TMA_REG_ADDRESS: u16 = 0xFF06;
const TAC_REG_ADDRESS: u16 = 0xFF07;

#[derive(Default)]
pub(crate) struct Timer {
    div: u8,         // FF04 — DIV: Divider register
    counter: u8,     // FF05 — TIMA: Timer counter
    modulo: u8,      // FF06 — TMA: Timer modulo
    control: u8,     // FF07 — TAC: Timer control
    // Bit  2   - Timer Enable
    // Bits 1-0 - Input Clock Select
    // 00: CPU Clock / 1024 (DMG, SGB2, CGB Single Speed Mode:   4096 Hz, SGB1:   ~4194 Hz, CGB Double Speed Mode:   8192 Hz)
    // 01: CPU Clock / 16   (DMG, SGB2, CGB Single Speed Mode: 262144 Hz, SGB1: ~268400 Hz, CGB Double Speed Mode: 524288 Hz)
    // 10: CPU Clock / 64   (DMG, SGB2, CGB Single Speed Mode:  65536 Hz, SGB1:  ~67110 Hz, CGB Double Speed Mode: 131072 Hz)
    // 11: CPU Clock / 256  (DMG, SGB2, CGB Single Speed Mode:  16384 Hz, SGB1:  ~16780 Hz, CGB Double Speed Mode:  32768 Hz)
}

impl Timer {
    // returns if timer interrupt requested
    pub(crate) fn step(&mut self, mmu: &mut Mmu, cycles: u8) -> bool {
        let mut request_interrupt = false;

        self.refresh_from_mem(mmu);

        self.inc_div(mmu, cycles);

        let clocks = self.selected_clocks();

        if self.is_enabled() && self.div as u16 >= clocks {
            request_interrupt = self.inc_counter(mmu, cycles);
        }

        request_interrupt
    }

    fn refresh_from_mem(&mut self, mmu: &mut Mmu) {
        self.div = mmu.read_8(DIV_REG_ADDRESS);
        self.counter = mmu.read_8(TIMA_REG_ADDRESS);
        self.modulo = mmu.read_8(TMA_REG_ADDRESS);
        self.control = mmu.read_8(TAC_REG_ADDRESS);
    }

    fn is_enabled(&self) -> bool {
        self.control & 0b0100 == 0b0100
    }

    fn selected_clocks(&self) -> u16 {
        match self.control & 0b0011 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!()
        }
    }

    fn inc_div(&mut self, mmu: &mut Mmu, increment_by: u8) {
        // Increment Div (unless STOP instruction was run)
        // Does anything happen when it overflows /wraps?
        self.div = self.div.wrapping_add(increment_by);

        mmu.write_8(DIV_REG_ADDRESS, self.div);
    }

    fn inc_counter(&mut self, mmu: &mut Mmu, increment_by: u8) -> bool {
        // Increment counter by selected clock frequency
        let (counter, request_interrupt) = self.counter.overflowing_add(increment_by);

        // If counter overflows, request interrupt and reset counter = modulo
        self.counter = if request_interrupt {
            self.modulo
        } else {
            counter
        };

        mmu.write_8(TIMA_REG_ADDRESS, self.counter);

        request_interrupt
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
