use crate::console::gpu::{Gpu, VRAM_BEGIN, VRAM_END};

pub(crate) struct MemoryBus {
    gpu: Gpu
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            gpu: Gpu::default(),
        }
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.read_vram(address - VRAM_BEGIN)
            }
            _ => panic!("Cannot read unsupported area of memory.")
        }
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, value)
            }
            _ => panic!("Cannot write to unsupported area of memory.")
        }
    }
}
