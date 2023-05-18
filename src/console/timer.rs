use crate::console::mmu::Mmu;

pub(crate) const DIV_REG_ADDRESS: u16 = 0xFF04;
const TIMA_REG_ADDRESS: u16 = 0xFF05;
const TMA_REG_ADDRESS: u16 = 0xFF06;
const TAC_REG_ADDRESS: u16 = 0xFF07;

#[derive(Default)]
pub(crate) struct Timer {
    pub(crate) div: u8,         // FF04 — DIV: Divider register
    pub(crate) counter: u8,     // FF05 — TIMA: Timer counter
    pub(crate) modulo: u8,      // FF06 — TMA: Timer modulo
    pub(crate) control: u8,     // FF07 — TAC: Timer control
    // Bit  2   - Timer Enable
    // Bits 1-0 - Input Clock Select
    // 00: CPU Clock / 1024 (DMG, SGB2, CGB Single Speed Mode:   4096 Hz, SGB1:   ~4194 Hz, CGB Double Speed Mode:   8192 Hz)
    // 01: CPU Clock / 16   (DMG, SGB2, CGB Single Speed Mode: 262144 Hz, SGB1: ~268400 Hz, CGB Double Speed Mode: 524288 Hz)
    // 10: CPU Clock / 64   (DMG, SGB2, CGB Single Speed Mode:  65536 Hz, SGB1:  ~67110 Hz, CGB Double Speed Mode: 131072 Hz)
    // 11: CPU Clock / 256  (DMG, SGB2, CGB Single Speed Mode:  16384 Hz, SGB1:  ~16780 Hz, CGB Double Speed Mode:  32768 Hz)
}

impl Timer {
    pub(crate) fn step(&mut self, mmu: &mut Mmu) -> bool {
        let mut request_interrupt = false;
        self.refresh_from_mem(mmu);

        // Increment Div (unless STOP instruction was run)
        // Does anything happen when it overflows /wraps?
        self.div = self.div.wrapping_add(1);

        if self.is_enabled() {
            // Increment counter by selected clock frequency
            let clocks = self.selected_clocks();
            (self.counter, request_interrupt) = self.counter.overflowing_add(clocks as u8); // How is TIMA incremented by u16 values...?
            // If counter overflows, request interrupt and reset counter = modulo
            if request_interrupt {
                self.counter = self.modulo;
            }
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
        self.control & 0b0000_0100 == 0b0000_0100
    }

    fn selected_clocks(&self) -> u16 {
        match self.control & 0b0000_0011 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 | _ => 256,
        }
    }
}
