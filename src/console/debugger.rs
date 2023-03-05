use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::console::{
    cpu::Cpu
};
use crate::console::mmu::{MemoryType, Mmu};

pub(crate) enum DebugAction {
    CONT,
    DUMP,
    STEP,
}

pub(crate) struct Debugger {
    pub(crate) enabled: bool,
    pub(crate) stepping: bool,
}

impl Debugger {
    pub(crate) fn new(enabled: bool) -> Debugger {
        Debugger {
            enabled: enabled,
            stepping: false,
        }
    }

    pub(crate) fn toggle_stepping(&mut self) {
        self.stepping = self.enabled && !self.stepping;
    }

    pub(crate) fn dump(&self, data: HashMap<&str, &str>) {
        if self.enabled && self.stepping {
            for key in data.keys() {
                println!("{}: {}", key, data.get(key).unwrap());
            }
        }
    }

    pub(crate) fn dump_cpu(&self, cpu: &Cpu) {
        self.dump(HashMap::from([
            ("Cpu.registers", format!("\n{}", cpu.registers).as_str()),
        ]));
    }

    pub(crate) fn dump_mmu(&self, mmu: &Mmu) {
        let vram_forst_8: &[u8] = &mmu.get_memory_buffer(&MemoryType::VRAM)[0..8];
        let mut vram_values_str = String::new();
        for x in vram_forst_8 {
            vram_values_str.push_str(format!("\t{:#06X}", x).as_str());
        }

        self.dump(HashMap::from([
            ("VRAM[0..8]", format!("\n{}\n", vram_values_str).as_str()),
        ]));
    }
}
