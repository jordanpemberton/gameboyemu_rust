use crate::console::mmu;
use crate::console::mmu::Mmu;
use crate::console::register::Register;

const DIV_SPEED: u16 = 256; // 16384Hz = 256 cpu clocks

#[allow(dead_code)]
pub(crate) struct Timer {
    div: Register,         // FF04 — DIV: Divider register
    tima: Register,        // FF05 — TIMA: Timer counter
    tma: Register,         // FF06 — TMA: Timer modulo
    tac: Register,         // FF07 — TAC: Timer control
    div_clocks: u16,
    tima_clocks: u16,
    overflow: bool,
    is_in_stop_mode: bool,
}

impl Timer {
    pub(crate) fn new() -> Timer {
        Timer {
            div: Register::new(mmu::DIV_REG),
            tima: Register::new(mmu::TIMA_REG),
            tma: Register::new(mmu::TMA_REG),
            tac: Register::new(mmu::TAC_REG),
            div_clocks: 0,
            tima_clocks: 0,
            overflow: false,
            is_in_stop_mode: false,
        }
    }

    #[allow(unused_variables)]
    pub(crate) fn step(&mut self, mmu: &mut Mmu, cycles: u8) -> bool {
        let mut request_interrupt = false;

        // Increment DIV if not in stop mode
        if !self.is_in_stop_mode {
            let is_time_to_increment_div = self.div_clocks >= DIV_SPEED;
            if is_time_to_increment_div {
                self.div_clocks = self.div_clocks - DIV_SPEED;

                let div = self.div.read(mmu).wrapping_add(1);
                self.div.write_force(mmu, div);  // TEMP Force write DIV reg
            }

            self.div_clocks += cycles as u16;
        }

        // Increment TIMA according to TAC
        let tac = self.tac.read(mmu);
        if tac & 0b0100 == 0b0100 {
            // Increment TIMA at the rate specified by TAC
            let selected_clocks = self.selected_clocks(tac);

            let is_time_to_increment_tima = self.tima_clocks >= selected_clocks;
            if is_time_to_increment_tima {
                self.tima_clocks = self.tima_clocks - selected_clocks;

                let (mut result, overflow) = self.tima.read(mmu).overflowing_add(1);
                // If TIMA overflows, set to TMA and request interrupt
                if overflow {
                    result = self.tma.read(mmu);
                    request_interrupt = true;
                }
                self.tima.write(mmu, result);
            }

            // TODO - TIMA uses DIV /internal clock (which can be reset to 0), see mooneye acceptance/timer/div_write.gb
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
