#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

extern crate core;

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

pub(crate) fn run(game_filepath: &Path) {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();
    mmu.load_rom_from_file(game_filepath);
    loop {
        cpu.step(&mut mmu);
    }
}

fn main() {
    let game_filepath = "/home/jordan/Games/GameBoy/GB/Pokemon - Red Version (USA, Europe) (SGB Enhanced).gb";
    run(game_filepath.as_ref());
}
