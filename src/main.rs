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

const BOOTROM_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/bootrom/DMG_ROM.bin";
const TEST_ROM_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb";
// "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/cpu_instrs.gb";
const GAME_FILEPATH: &str = "/home/jordan/Games/GameBoy/GB/Tetris.gb";

fn disassemble_roms() {
    let bootrom_cartridge = Cartridge::new(BOOTROM_FILEPATH.as_ref());
    disassembler::disassemble_to_output_file(&bootrom_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/bootrom_disasseble.txt");

    let test_cartridge = Cartridge::new(TEST_ROM_FILEPATH.as_ref());
    disassembler::disassemble_to_output_file(&test_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/testrom_disassemble.txt");

    let game_cartridge = Cartridge::new(GAME_FILEPATH.as_ref());
    disassembler::disassemble_to_output_file(&game_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/game_disassemble.txt");
}

fn main() {
    disassemble_roms();

    let mut gamboy = Console::new(true);

    // gamboy.run(CartridgeOption::NONE, false);
    // gamboy.run(CartridgeOption::SOME(Cartridge::new(TEST_ROM_FILEPATH.as_ref())), true);
    gamboy.run(CartridgeOption::SOME(Cartridge::new(GAME_FILEPATH.as_ref())), true);
}
