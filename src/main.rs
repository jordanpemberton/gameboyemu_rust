#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

mod console;
mod cartridge;

use crate::console::cpu::Cpu;
use crate::console::mmu::Mmu;

use std::fs::{File};
use std::io::{Read};
use std::path::{Path};

pub(crate) fn load_rom(path: &Path) -> Vec<u8> {
    let mut f = File::open(path).expect("Failed to open file");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("Failed to read file");
    buf
}

pub(crate) fn run(_rom: &[u8]) {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();
    loop {
        cpu.step(&mut mmu);
    }
}

fn main() {
    let file_path = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/individual/06-ld r,r.gb";
    let rom = load_rom(Path::new(&file_path));
    run(&rom);
}
