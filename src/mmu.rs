const RAM_SIZE: usize = 65536;

pub(crate) struct Mmu {
    ram: [u8; RAM_SIZE]
}

impl Mmu {
    pub(crate) fn new() -> Mmu {
        Mmu {
            ram: [0; RAM_SIZE],
        }
    }

    pub(crate) fn read_byte(&self, address: usize) -> u8 {
        self.ram[address]
    }

    pub(crate) fn read_word(&self, address: usize) -> u16 {
        (self.ram[address] as u16) << 8 | self.ram[address + 1] as u16
    }

    fn load_byte(&mut self, address: usize, value: u8) {
        self.ram[address] = value;
    }

    fn load_word(&mut self, address: usize, value: u16) {
        let lsb = ((value & 0xFF00) >> 8) as u8;
        let msb = (value & 0x00FF) as u8;
        self.ram[address] = lsb;
        self.ram[address + 1] = msb;
    }
}
