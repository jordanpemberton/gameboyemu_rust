#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs::{read};

mod console;
mod cartridge;

use crate::cartridge::cartridge::{Cartridge, CartridgeOption};
use crate::console::console::Console;
use crate::console::cpu::Cpu;
use crate::console::mmu::Mmu;
use crate::console::registers::{RegIndex};

fn main() {
    let game_filepath = "/home/jordan/Games/GameBoy/GB/Tetris.gb";
    let game_cartridge = Cartridge::new(game_filepath.as_ref());
    let test_rom_filepath = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/cpu_instrs.gb";
    let test_cartridge = Cartridge::new(test_rom_filepath.as_ref());

    let mut gamboy = Console::new();
    // gamboy.run(CartridgeOption::NONE);
    // gamboy.run(CartridgeOption::SOME(test_cartridge));
    gamboy.run(CartridgeOption::SOME(game_cartridge));
}
