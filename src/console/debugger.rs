use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::console::{
    cpu::Cpu
};
use crate::console::mmu::{Endianness, Mmu};
use crate::console::ppu::{LCD_PIXEL_HEIGHT, LCD_PIXEL_WIDTH, Ppu};

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
    pub(crate) fn new() -> Debugger {
        Debugger {
            enabled: true,
            active: false,
        }
    }

    pub(crate) fn is_active(&self) -> bool {
        self.enabled && self.active
    }

    pub(crate) fn break_or_cont(&mut self, cpu: Option<&mut Cpu>, mmu: Option<&Mmu>, ppu: Option<&Ppu>, locals: Option<HashMap<&str, &str>>) {
        if self.enabled {
            self.active = !self.active;
            if self.active {
                self.dump(cpu, mmu, ppu, locals);
            }
        }
    }

    pub(crate) fn peek(&mut self, cpu: Option<&mut Cpu>, mmu: Option<&Mmu>, ppu: Option<&Ppu>, locals: Option<HashMap<&str, &str>>) {
        if self.enabled {
            self.dump(cpu, mmu, ppu, locals);
        }
    }

    pub(crate) fn step(&mut self, cpu: Option<&mut Cpu>, mmu: Option<&Mmu>, ppu: Option<&Ppu>, locals: Option<HashMap<&str, &str>>) {
        // todo
    }

    fn dump(&mut self, cpu: Option<&mut Cpu>, mmu: Option<&Mmu>, ppu: Option<&Ppu>, locals: Option<HashMap<&str, &str>>) {
        if let Some(_cpu) = cpu {
            self.dump_cpu_state(_cpu);
        }
        if let Some(_mmu) = mmu {
            self.dump_mmu_state(_mmu);
        }
        if let Some(_ppu) = ppu {
            self.draw_screen_stdout(_ppu);
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
        let vram: &[u8] = &mmu.read_buffer(0x8000, 0xA000, Endianness::BIG);
        let rows: usize = 2; // for all: (vram.len() as u16 / cols as u16) as usize;

        let mut i = 0;
        for row in 0..rows {
            vram_values_str.push_str(format!("\n{:#06X} :", i + 0x8000).as_str());

            for col in 0..cols {
                if vram[i] > 0 {
                    vram_values_str.push_str(format!("\t{:#04X}", vram[i]).as_str());
                } else {
                    vram_values_str.push_str("\t____");
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

    fn draw_screen_stdout(&mut self, ppu: &Ppu) {
        for x in 0..LCD_PIXEL_WIDTH + 2 {
            print!("_");
        }
        println!();
        for y in 0..LCD_PIXEL_HEIGHT {
            print!("|");
            for x in 0..LCD_PIXEL_WIDTH {
                let color = ppu.lcd.data[y as usize][x as usize];
                match color % 4 {
                    3 => print!("@"),
                    2 => print!("+"),
                    1 => print!("."),
                    _ => print!(" "),
                }
            }
            print!("|");
            println!();
        }
        for x in 0..LCD_PIXEL_WIDTH + 2 {
            print!("_");
        }
        println!();
    }
}
