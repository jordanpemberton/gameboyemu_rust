use crate::console::mmu;
use crate::console::mmu::{Caller, Mmu};
use crate::console::register::Register;

// const DIV_SPEED: u16 = 256; // 16384Hz = 256 cpu clocks

#[allow(dead_code)]
pub(crate) struct Timer {
    // Internally, SYSCLK is a 16 bit divider, incremented each "clock tick" (every cpu clock cycle?)
    tima_clocks: u16,
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

        // TODO: stop mode
        if self.is_in_stop_mode {
            return request_interrupt;
        }

        let tac = self.tac.read(mmu, Caller::TIMER);
        let selected_clocks = self.selected_clocks(tac);
        let tac_mask = self.tac_frequency_mask(tac);
        let prev_tac_frequency_bit = (mmu.sysclock & tac_mask) == tac_mask;

        // Increment internal/system clock by cycles, which in turn increments DIV.
        // https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html#relation-between-timer-and-divider-register
        mmu.sysclock = mmu.sysclock.wrapping_add(cycles as u16);
        // if mmu.sysclock >= 0x4000 {
        //     mmu.sysclock = 0;
        // }
        let new_tac_frequency_bit = (mmu.sysclock & tac_mask) == tac_mask;

        // DIV = top 8 bits of sysclock /sysclock shifted >> 6 bits. DIV increments every 256 machine clocks (?)
        let new_div = (mmu.sysclock >> 8) as u8;
        // let new_div = (mmu.sysclock >> 6) as u8;
        if new_div != self.div.read(mmu, Caller::TIMER) {
            self.div.write(mmu, new_div, Caller::TIMER);
        }

        // Increment TIMA according to TAC
        self.tima_clocks = self.tima_clocks.wrapping_add(cycles as u16);
        let tima_incr_is_enabled = (tac & 0b0100) == 0b0100;

        if tima_incr_is_enabled {
            // Increment TIMA at the rate specified by TAC
            let time_to_increment_tima = !prev_tac_frequency_bit && new_tac_frequency_bit;

            // if time_to_increment_tima {
            while self.tima_clocks >= selected_clocks {
                // Increment TIMA
                let (mut new_tima, overflow) = self.tima.read(mmu, Caller::TIMER)
                    .overflowing_add(1);

                // If TIMA overflows, set TIMA to TMA and request interrupt
                if overflow {
                    new_tima = self.tma.read(mmu, Caller::TIMER);
                    request_interrupt = true;
                }

                self.tima.write(mmu, new_tima, Caller::TIMER);

                // Reset /adjust TIMA internal counter
                self.tima_clocks = self.tima_clocks.wrapping_sub(selected_clocks);
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

    #[allow(dead_code)]
    fn selected_clocks(&self, tac: u8) -> u16 {
        match tac & 0b0011 {
            0b00 => 1024, // 256 M * 4
            0b01 => 16,   // 4 M * 4
            0b10 => 64,   // 16 M * 4
            0b11 => 256,  // 64 M * 4
            _ => unreachable!()
        }
    }

    fn tac_frequency_mask(&self, tac: u8) -> u16 {
        let tac_frequency = tac & 0b0011;
        // https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html#relation-between-timer-and-divider-register
        // ______0_ 3_2_1_
        match tac_frequency {
            0b00 => 0b1000_0000, // 0x80, 128
            0b01 => 0b0010_0000, // 0x40, 32
            0b10 => 0b0000_1000, // 0x08, 8
            0b11 => 0b0000_0010, // 0x02, 2
            _ => unreachable!()
        }
    }
}
