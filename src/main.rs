#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
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
use crate::console::cpu_registers::{CpuRegIndex, CpuRegisters};

const BOOTROM_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/bootrom/DMG_ROM.bin";

const GAME_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/games/Tetris.gb";
// const GAME_FILEPATH: &str = "/home/jordan/Games/GameBoy/GB/Pokemon - Red Version (USA, Europe) (SGB Enhanced).gb";
// const GAME_FILEPATH: &str = "/home/jordan/Games/GameBoy/GB/Pokemon - Blue Version (USA, Europe) (SGB Enhanced).gb";

const TEST_ROM_DIR: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/";
const TEST_ROMS: [&str; 12] = [
    "cpu_instrs.gb",
    "individual/01-special.gb",
    "individual/02-interrupts.gb",
    "individual/03-op sp,hl.gb",
    "individual/04-op r,imm.gb",
    "individual/05-op rp.gb",
    "individual/06-ld r,r.gb",
    "individual/07-jr,jp,call,ret,rst.gb",
    "individual/08-misc instrs.gb",
    "individual/09-op r,r.gb",
    "individual/10-bit ops.gb",
    "individual/11-op a,(hl).gb",
];
const TEST_ROM: usize = 0;

fn disassemble_roms() {
    let bootrom_cartridge = Cartridge::new(BOOTROM_FILEPATH.as_ref());
    disassembler::disassemble_to_output_file(&bootrom_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/bootrom_disasseble.txt");

    for test_rom in TEST_ROMS {
        let filepath = format!("{}{}", TEST_ROM_DIR, test_rom);
        let test_cartridge = Cartridge::new(filepath.as_ref());

        let mut parts = test_rom.split('.').collect::<Vec<&str>>();
        let mut name = parts.first().unwrap();
        parts = name.split('/').collect::<Vec<&str>>();
        name = parts.last().unwrap();

        let path = format!("/home/jordan/RustProjs/GameBoyEmu/out/testrom_disassemble__{}.txt",
           *name);
        disassembler::disassemble_to_output_file(&test_cartridge.data, path.as_str());
    }

    let game_cartridge = Cartridge::new(GAME_FILEPATH.as_ref());
    disassembler::disassemble_to_output_file(&game_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/game_disassemble.txt");
}

fn main() {
    disassemble_roms();

    CpuRegisters::test();

    let mut gamboy = Console::new("GAMBOY",4, true, true);

    gamboy.run(CartridgeOption::NONE, false);

    // gamboy.run(CartridgeOption::SOME(Cartridge::new(GAME_FILEPATH.as_ref())), true);

    let test_filepath = format!("{}{}", TEST_ROM_DIR, TEST_ROMS[TEST_ROM]);
    // gamboy.run(CartridgeOption::SOME(Cartridge::new(test_filepath.as_str().as_ref())), true);
}
