use crate::console::mmu;
use crate::console::mmu::{Caller, Mmu};
use crate::console::register::Register;

// const DIV_SPEED: u16 = 256; // 16384Hz = 256 cpu clocks

#[allow(dead_code)]
pub(crate) struct Timer {
    // Internally, SYSCLK is a 16 bit divider, incremented each "clock tick" (every cpu clock cycle?)
    tima_clocks: u16,
    overflow: bool,
    is_in_stop_mode: bool, // TODO implement

    // The DIV IO register only exposes the upper 8 bits of SYSCLK,
    // so its exposed value increases every 256 cycles.
    div: Register,         // FF04 — DIV: Divider register
    // TIMA uses the internal value of the SYSCLK divider (?),
    // increments every 16, 64, 256 or 1024 clock cycles depending on the frequency set in the TAC register.
    tima: Register,        // FF05 — TIMA: Timer counter
    tma: Register,         // FF06 — TMA: Timer modulo
    tac: Register,         // FF07 — TAC: Timer control
}

impl Timer {
    pub(crate) fn new() -> Timer {
        Timer {
            tima_clocks: 0,
            overflow: false,
            is_in_stop_mode: false,
            div: Register::new(mmu::DIV_REG),
            tima: Register::new(mmu::TIMA_REG),
            tma: Register::new(mmu::TMA_REG),
            tac: Register::new(mmu::TAC_REG),
        }
    }

    #[allow(unused_variables)]
    pub(crate) fn step(&mut self, mmu: &mut Mmu, cycles: u8) -> bool {
        let mut request_interrupt = false;

        // If internal clock was reset, reset tima_clocks also
        if mmu.sysclock == 0 {
            self.tima_clocks = 0;
        }

        // TODO: stop mode
        if self.is_in_stop_mode {
            return request_interrupt;
        }

        // Increment internal/system clock /DIV
        mmu.sysclock = mmu.sysclock.wrapping_add(cycles as u16);
        self.div.write(mmu, (mmu.sysclock >> 8) as u8, Caller::TIMER); // DIV = botttom 8 or sysclock, increments every 256 clocks

        // Increment TIMA according to TAC
        let tac = self.tac.read(mmu, Caller::TIMER);
        let tima_incr_is_enabled = tac & 0b0100 == 0b0100;

        if tima_incr_is_enabled {
            self.tima_clocks += cycles as u16;

            // Increment TIMA at the rate specified by TAC
            let selected_clocks = self.selected_clocks(tac);

            while self.tima_clocks >= selected_clocks {
                let (mut new_tima, overflow) = self.tima.read(mmu, Caller::TIMER).overflowing_add(1);

                // If TIMA overflows, set TIMA to TMA and request interrupt
                if overflow {
                    request_interrupt = true;
                    new_tima = self.tma.read(mmu, Caller::TIMER);
                }

                self.tima.write(mmu, new_tima, mmu::Caller::TIMER);
                self.tima_clocks = self.tima_clocks - selected_clocks;
            }
        }

        request_interrupt
    }

    pub(crate) fn as_str(&mut self, mmu: &mut Mmu) -> String {
        format!(
            "\tDIV:             {:#04X}\n\
             \tTIMA (counter):  {:#04X}\n\
             \tTMA (modulo):    {:#04X}\n\
             \tTAC (control):   {:#04X}\n\
             \tis_in_stop_mode: {}",
            self.div.read(mmu, Caller::TIMER),
            self.tima.read(mmu, Caller::TIMER),
            self.tma.read(mmu, Caller::TIMER),
            self.tac.read(mmu, Caller::TIMER),
            self.is_in_stop_mode,
        )
    }

    fn selected_clocks(&self, tac: u8) -> u16 {
        match tac & 0b0011 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!()
        }
    }
}
