#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod cartridge;
mod console;

use crate::cartridge::cartridge::{Cartridge, CartridgeOption};
use crate::console::console::Console;
use crate::console::cpu::Cpu;
use crate::console::disassembler;
use crate::console::mmu::Mmu;
use crate::console::registers::{RegIndex};

fn main() {
    let debug = true;

    let bootrom_filepath = "/home/jordan/RustProjs/GameBoyEmu/roms/bootrom/DMG_ROM.bin";
    let bootrom_cartridge = Cartridge::new(bootrom_filepath.as_ref());
    disassembler::disassemble_to_output_file(&bootrom_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/DMG_ROM.txt");

    let game_filepath = "/home/jordan/Games/GameBoy/GB/Tetris.gb";
    let game_cartridge = Cartridge::new(game_filepath.as_ref());
    disassembler::disassemble_to_output_file(&game_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/Tetris.txt");

    let test_rom_filepath = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/cpu_instrs.gb";
    let test_cartridge = Cartridge::new(test_rom_filepath.as_ref());
    disassembler::disassemble_to_output_file(&test_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/cpu_instrs.txt");

    let mut gamboy = Console::new(debug);

    // gamboy.run(CartridgeOption::NONE);
    // gamboy.run(CartridgeOption::SOME(test_cartridge));
    // gamboy.run(CartridgeOption::SOME(game_cartridge));
}
