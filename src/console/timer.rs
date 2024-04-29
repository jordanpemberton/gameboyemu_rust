use crate::console::mmu;
use crate::console::mmu::{Caller, Mmu};
use crate::console::register::Register;

// const DIV_SPEED: u16 = 256; // 16_384Hz = 256 cpu clocks

#[allow(dead_code)]
pub(crate) struct Timer {
    is_in_stop_mode: bool, // TODO implement
    tima_overflow: bool,
    // The DIV IO register only exposes the upper 8 bits of system 16bit counter,
    // so its exposed value increases every 256 cycles.
    div: Register,         // FF04 — DIV: Divider register
    // TIMA uses the internal value of the system clock 16bit counter,
    // increments every 16, 64, 256 or 1024 clock cycles depending on the frequency set in the TAC register.
    tima: Register,        // FF05 — TIMA: Timer counter
    tma: Register,         // FF06 — TMA: Timer modulo
    tac: Register,         // FF07 — TAC: Timer control
}

impl Timer {
    pub(crate) fn new() -> Timer {
        Timer {
            is_in_stop_mode: false,
            tima_overflow: false,
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
            return false;
        }

        // Check TAC
        let tac = self.tac.read(mmu, Caller::TIMER);
        let tima_incr_is_enabled = (tac & 0b0100) == 0b0100;
        let tac_frequency = self.tac_frequency_mask(tac);
        let prev_tac_frequency_bit = self.tac_counter_bit(mmu.sysclock, tac);

        // Increment internal/system clock by cycles, which in turn increments DIV.
        // https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html#relation-between-timer-and-divider-register
        mmu.sysclock = mmu.sysclock.wrapping_add(cycles as u16);

        // DIV = top 8 bits of sysclock /sysclock shifted >> 8 bits. DIV increments every 256 machine clocks.
        let new_div = (mmu.sysclock >> 8) as u8; // why does mooneye use >>6?
        if new_div != self.div.read(mmu, Caller::TIMER) {
            self.div.write(mmu, new_div, Caller::TIMER);
        }

        // If TIMA overflowed, set TIMA to TMA and request interrupt.
        if self.tima_overflow {
            let tma = self.tma.read(mmu, Caller::TIMER);
            self.tima.write(mmu, tma, Caller::TIMER);
            self.tima_overflow = false;
            request_interrupt = true;
        }

        // Else increment TIMA according to TAC and DIV/sysclock
        else if tima_incr_is_enabled {
            let tac_frequency = self.tac_frequency_mask(tac);
            let new_tac_frequency_bit = self.tac_counter_bit(mmu.sysclock, tac);
            let increment_tima = prev_tac_frequency_bit != new_tac_frequency_bit; // TAC bit was just changed
            // let increment_tima = prev_tac_frequency_bit && !new_tac_frequency_bit; // TAC bit was just cleared
            // let increment_tima = !prev_tac_frequency_bit && new_tac_frequency_bit; // TAC bit was just set

            if increment_tima {
            // while self.tima_clocks >= tac_frequency { // vs counting cycles
                let mut tima = self.tima.read(mmu, Caller::TIMER);
                (tima, self.tima_overflow) = tima.overflowing_add(1);
                self.tima.write(mmu, tima, Caller::TIMER);
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
            0b00 => 1 << 10, // 1024, // = 256 M * 4
            0b01 => 1 << 4,  // 16,   // = 4 M * 4
            0b10 => 1 << 6,  // 64,   // = 16 M * 4
            0b11 => 1 << 8,  // 256,  // = 64 M * 4
            _ => unreachable!()
        }
    }

    #[allow(dead_code)]
    fn tac_frequency_mask(&self, tac: u8) -> u16 {
        let tac_frequency = tac & 0b0011;
        // https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html#relation-between-timer-and-divider-register
        // https://www.reddit.com/r/EmuDev/comments/pbmu8r/gameboy_writing_to_the_div_location_reset_its/
        // bits 3, 5, 7, 9 of sysclock (DIV):
        // ____ __0_ 3_2_ 1___
        match tac_frequency {
            0b00 => 1 << 9, // 0b0010_0000_0000, // 0x200, 512
            0b01 => 1 << 3, // 0b0000_1000,      // 0x08,  8
            0b10 => 1 << 5, // 0b0010_0000,      // 0x20,  32
            0b11 => 1 << 7, // 0b1000_0000,      // 0x80,  128
            _ => unreachable!()
        }
    }

    #[allow(dead_code)]
    // from mooneye
    fn tac_counter_bit(&self, internal_counter: u16, tac: u8) -> bool {
        // let mask = self.tac_counter_mask_16b(tac);
        let mask = self.tac_frequency_mask(tac);
        (internal_counter & mask) == mask
    }

    #[allow(dead_code)]
    fn tac_counter_mask_14b(&self, tac: u8) -> u16 {
        match tac & 0b11 {
            0b00 => 1 << 7,       // 0x80, 128
            0b01 => 1 << 1,       // 0x02, 2
            0b10 => 1 << 3,       // 0x08, 8
            0b11 => 1 << 5,       // 0x20, 32
            _ => unreachable!()
        }
    }

    #[allow(dead_code)]
    fn tac_counter_mask_16b(&self, tac: u8) -> u16 {
        match tac & 0b11 {
            0b00 => 1 << 8,       // 0x80, 128
            0b01 => 1 << 2,       // 0x02, 2
            0b10 => 1 << 4,       // 0x08, 8
            0b11 => 1 << 6,       // 0x20, 32
            _ => unreachable!()
        }
    }
}
