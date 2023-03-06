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
    pub(crate) active: bool,
}

impl Debugger {
    pub(crate) fn new(enabled: bool) -> Debugger {
        Debugger {
            enabled: enabled,
            active: false,
        }
    }

    pub(crate) fn dump(&mut self, cpu: Option<&mut Cpu>, mmu: Option<&Mmu>, locals: Option<HashMap<&str, &str>>) {
        if let Some(_cpu) = cpu {
            self.dump_cpu_state(_cpu);
        }
        if let Some(_mmu) = mmu {
            self.dump_mmu_state(_mmu);
        }
        if let Some(_locals) = locals {
            self.dump_key_value_pairs(_locals);
        }
    }

    fn dump_key_value_pairs(&self, data: HashMap<&str, &str>) {
        for key in data.keys() {
            println!("{}: {}", key, data.get(key).unwrap());
        }
    }

    fn dump_cpu_state(&self, cpu: &Cpu) {
        self.dump_key_value_pairs(HashMap::from([
            ("Cpu.registers", format!("\n{}", cpu.registers).as_str()),
        ]));
    }

    fn dump_mmu_state(&self, mmu: &Mmu) {
        let vram_forst_8: &[u8] = &mmu.get_memory_buffer(&MemoryType::VRAM)[0..8];
        let mut vram_values_str = String::new();
        for x in vram_forst_8 {
            vram_values_str.push_str(format!("\t{:#06X}", x).as_str());
        }

        self.dump_key_value_pairs(HashMap::from([
            ("VRAM[0..8]", format!("\n{}\n", vram_values_str).as_str()),
        ]));
    }
}
