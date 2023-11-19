use std::fmt::{Display, Formatter};
use crate::console::mmu;
use crate::console::mmu::Mmu;

const DIV_SPEED: u16 = 256; // 16384Hz = 256 cpu clocks

#[allow(dead_code)]
#[derive(Default)]
pub(crate) struct Timer {
    div: u8,         // FF04 — DIV: Divider register
    tima: u8,        // FF05 — TIMA: Timer counter
    tma: u8,         // FF06 — TMA: Timer modulo
    tac: u8,         // FF07 — TAC: Timer control
    div_clocks: u16,
    tima_clocks: u16,
    overflow: bool,
    is_in_stop_mode: bool,
}

impl Timer {
    #[allow(unused_variables)]
    pub(crate) fn step(&mut self, mmu: &mut Mmu, cycles: u8) -> bool {
        let mut request_interrupt = false;
        self.refresh_from_mem(mmu);

        if !self.is_in_stop_mode {
            let is_time_to_increment_div = self.div_clocks >= DIV_SPEED;
            if is_time_to_increment_div {
                self.div_clocks = self.div_clocks - DIV_SPEED;
                self.div = self.div.wrapping_add(1);
            }
            self.div_clocks += cycles as u16;
        }

        if self.tac & 0b0100 == 0b0100 {
            // Increment TIMA at the rate specified by TAC
            let is_time_to_increment_tima = self.tima_clocks >= self.selected_clocks();
            if is_time_to_increment_tima {
                self.tima_clocks = self.tima_clocks - self.selected_clocks();
                let (result, overflow) = self.tima.overflowing_add(1);
                self.tima = result;
                if overflow {
                    self.tima = self.tma;
                    request_interrupt = true;
                }
            }
            // TODO - TIMA uses DIV /internal clock (which can be reset to 0), see mooneye acceptance/timer/div_write.gb
            self.tima_clocks += cycles as u16;
        }

        self.update_mem(mmu);

        request_interrupt
    }

    fn refresh_from_mem(&mut self, mmu: &mut Mmu) {
        self.div = mmu.read_8(mmu::DIV_REG);
        self.tima = mmu.read_8(mmu::TIMA_REG);
        self.tma = mmu.read_8(mmu::TMA_REG);
        self.tac = mmu.read_8(mmu::TAC_REG);
    }

    fn update_mem(&mut self, mmu: &mut Mmu) {
        mmu.write_8_force(mmu::DIV_REG, self.div);
        mmu.write_8(mmu::TIMA_REG, self.tima);
        mmu.write_8(mmu::TMA_REG, self.tma);
        mmu.write_8(mmu::TAC_REG, self.tac);
    }

    fn selected_clocks(&self) -> u16 {
        match self.tac & 0b0011 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!()
        }
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,
            "\tDIV:             {:#04X}\n\
             \tTIMA (counter):  {:#04X}\n\
             \tTMA (modulo):    {:#04X}\n\
             \tTAC (control):   {:#04X}\n\
             \tis_in_stop_mode: {}",
            self.div,
            self.tima,
            self.tma,
            self.tac,
            self.is_in_stop_mode
        )
    }
}
