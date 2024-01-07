use crate::console::mmu;
use crate::console::mmu::Mmu;
use crate::console::register::Register;

// const DIV_SPEED: u16 = 256; // 16384Hz = 256 cpu clocks

#[allow(dead_code)]
pub(crate) struct Timer {
    // Internally, SYSCLK is a 16 bit divider, incremented each clock tick (every 1 or 4 cycles?).
    tima_clocks: u16,
    overflow: bool,
    is_in_stop_mode: bool,

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

        // If internal clock was reset, reset tima_clocks
        if mmu.sysclock == 0 {
            self.tima_clocks = 0;
        }

        // Increment internal clock /DIV if not in stop mode
        if !self.is_in_stop_mode {
            mmu.sysclock = mmu.sysclock.wrapping_add(cycles as u16);
            self.div.write_force(mmu, (mmu.sysclock >> 4) as u8); // DIV increments every 256 clocks
        }

        // Increment TIMA according to TAC
        let tac = self.tac.read(mmu);
        let tima_incr_is_enabled = tac & 0b0100 == 0b0100;
        if tima_incr_is_enabled {
            // Increment TIMA at the rate specified by TAC
            let selected_clocks = self.selected_clocks(tac);

            let is_time_to_increment_tima = self.tima_clocks >= selected_clocks;
            if is_time_to_increment_tima {
                // Reset tima_clocks
                self.tima_clocks = self.tima_clocks - selected_clocks;

                let (mut result, overflow) = self.tima.read(mmu).overflowing_add(1);
                // If TIMA overflows, set TIMA to TMA and request interrupt
                if overflow {
                    result = self.tma.read(mmu);
                    request_interrupt = true;
                }

                self.tima.write(mmu, result);
            }

            self.tima_clocks += cycles as u16;
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
            self.div.read(mmu),
            self.tima.read(mmu),
            self.tma.read(mmu),
            self.tac.read(mmu),
            self.is_in_stop_mode
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
