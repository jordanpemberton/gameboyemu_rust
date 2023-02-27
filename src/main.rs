#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

extern crate core;

mod console;
mod cartridge;

use crate::cartridge::cartridge::Cartridge;
use crate::console::cpu::Cpu;
use crate::console::mmu::Mmu;
use crate::console::registers::{RegIndex};

use std::fs::{read};

fn run(game: Option<Cartridge>) {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();

    if let Some(mut cartridge) = game {
        run_game(&mut cpu, &mut mmu, &mut cartridge);
    } else {
        boot_empty(&mut cpu, &mut mmu);
    }
}

fn run_game(cpu: &mut Cpu, mmu: &mut Mmu, game: &Cartridge) {
    // TODO load roms correctly, map memory correctly /MBCs
    let bootrom_filepath = "/home/jordan/RustProjs/GameBoyEmu/src/console/DMG_ROM.bin";
    let bootrom = read(bootrom_filepath).unwrap();
    mmu.load_rom(bootrom.as_ref(), 0, bootrom.len());

    mmu.load_rom(&game.data[0x100..], 0x100, 0x8000 - 0x100);
    cpu.registers.set_word(RegIndex::PC, 0x100);

    loop {
        cpu.step(mmu);
    }
}

fn boot_empty(cpu: &mut Cpu, mmu: &mut Mmu) {
    let bootrom_filepath = "/home/jordan/RustProjs/GameBoyEmu/src/console/DMG_ROM.bin";
    let bootrom = read(bootrom_filepath).unwrap();
    mmu.load_rom(bootrom.as_ref(), 0, bootrom.len());

    loop {
        cpu.step(mmu);
    }
}

fn main() {
    let game_filepath = "/home/jordan/Games/GameBoy/GB/Tetris.gb";
    let game_cartridge = Cartridge::new(game_filepath.as_ref());
    let test_rom_filepath = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/cpu_instrs.gb";
    let test_cartridge = Cartridge::new(test_rom_filepath.as_ref());

    // run(None);
    // run(Option::from(test_cartridge));
    run(Option::from(game_cartridge));
}
