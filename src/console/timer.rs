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

    pub(crate) fn is_tac_enabled(tac: u8) -> bool {
        (tac & 0b0100) == 0b0100
    }

    pub(crate) fn is_counter_bit_flipped(internal_counter: u16, tac: u8) -> bool {
        let bit_mask = Timer::counter_bit(tac);
        (internal_counter & bit_mask) == bit_mask
    }

    #[allow(unused_variables)]
    pub(crate) fn step(&mut self, mmu: &mut Mmu, cycles: u16) -> bool {
        let mut request_interrupt = false;

        // TODO: stop mode
        if self.is_in_stop_mode {
            return false;
        }

        // Check TAC
        let tac = self.tac.read(mmu, Caller::TIMER);
        let tima_incr_is_enabled = Timer::is_tac_enabled(tac);

        // Using TAC, check sysclock counter bit (before clock is incremented)
        let prev_counter_bit = Timer::is_counter_bit_flipped(mmu.sysclock, tac);

        // Increment internal/system clock by cycles, which in turn increments DIV.
        // https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html#relation-between-timer-and-divider-register
        mmu.sysclock = mmu.sysclock.wrapping_add(cycles);

        // DIV = top 8 (?) bits of the sysclock, ie sysclock shifted >> 8(?) bits.
        // DIV increments every 256 machine clocks(?)
        let curr_div = self.div.read(mmu, Caller::TIMER);
        let new_div = (mmu.sysclock >> 8) as u8;
        if new_div != curr_div {
            self.div.write(mmu, new_div, Caller::TIMER);
        }

        // If TIMA overflowed (on last tick), set TIMA to TMA and request interrupt.
        // When TIMA overflows, the value from TMA is copied, and the timer flag is set in IF, but one M-cycle later.
        // (This means that TIMA is equal to $00 for the M-cycle after it overflows.)
        // (This only happens when TIMA overflows from incrementing, it cannot be made to happen by manually writing to TIMA.)
        if self.tima_overflow {
            let tma = self.tma.read(mmu, Caller::TIMER);
            self.tima.write(mmu, tma, Caller::TIMER);
            self.tima_overflow = false;
            request_interrupt = true;
        }
        // Else increment TIMA according to TAC and DIV/sysclock
        else if tima_incr_is_enabled {
            let new_counter_bit = Timer::is_counter_bit_flipped(mmu.sysclock, tac);

            let increment_tima =
                // Option 1: different than before -- Passes all blarrg/cpu_instrs tests!
                // but fails blargg/mem_timing/individual/02-write_timing.gb
                prev_counter_bit != new_counter_bit;

                // Option 2: was flipped last tick but now isn't
                // passes more of blargg/mem_timing/individual/02-write_timing.gb??,
                // but fails blarrg/cpu_instrs/individual/02-interrupts.gb!! >:(
                // prev_counter_bit && !new_counter_bit;

            if increment_tima {
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

    // https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html#relation-between-timer-and-divider-register
    // https://www.reddit.com/r/EmuDev/comments/pbmu8r/gameboy_writing_to_the_div_location_reset_its/
    // Used to mask bits of the sysclock:
    fn counter_bit(tac: u8) -> u16 {
        match tac & 0b11 {
            0b00 => 1 << 9,
            0b11 => 1 << 7,
            0b10 => 1 << 5,
            0b01 => 1 << 3,
            _ => unreachable!()
        }
    }
}
