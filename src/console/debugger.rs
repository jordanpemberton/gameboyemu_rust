use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::console::{
    cpu::Cpu
};
use crate::console::mmu::{MemoryType, Mmu};

pub(crate) enum DebugAction {
    BREAK,
    PEEK,
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

    pub(crate) fn is_active(&self) -> bool {
        self.enabled && self.active
    }

    pub(crate) fn break_or_cont(&mut self, cpu: Option<&mut Cpu>, mmu: Option<&Mmu>, locals: Option<HashMap<&str, &str>>) {
        if self.enabled {
            self.active = !self.active;
            if self.active {
                self.dump(cpu, mmu, locals);
            }
        }
    }

    pub(crate) fn peek(&mut self, cpu: Option<&mut Cpu>, mmu: Option<&Mmu>, locals: Option<HashMap<&str, &str>>) {
        if self.enabled {
            self.dump(cpu, mmu, locals);
        }
    }

    pub(crate) fn step(&mut self, cpu: Option<&mut Cpu>, mmu: Option<&Mmu>, locals: Option<HashMap<&str, &str>>) {
        // todo
    }

    fn dump(&mut self, cpu: Option<&mut Cpu>, mmu: Option<&Mmu>, locals: Option<HashMap<&str, &str>>) {
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
        let cols: usize = 16;

        let mut vram_values_str = String::new();
        let vram: &[u8] = &mmu.get_memory_buffer(&MemoryType::VRAM);
        let rows: usize = 16; // vram.len() / cols;

        let mut i = 0;
        for row in 0..rows {
            vram_values_str.push_str(format!("\n{:#06X} :", i).as_str());

            for col in 0..cols {
                if vram[i] > 0 {
                    vram_values_str.push_str(format!("\t{:#06X}", vram[i]).as_str());
                } else {
                    vram_values_str.push_str("\t______");
                }
                i += 1;
            }
            vram_values_str.push_str("\n");
        }

        self.dump_key_value_pairs(HashMap::from([
            (
                format!("VRAM[0..{}]", (rows * cols)).as_str(),
                format!("\n{}\n", vram_values_str).as_str()
            )
        ]));
    }
}
