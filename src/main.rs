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
use crate::console::cpu_registers::{CpuRegIndex, CpuRegisters};

const BOOTROM_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/bootrom/DMG_ROM.bin";

const TEST_ROM_DIR: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/";
const TEST_ROMS: [&str; 1] = [
    "cpu_instrs.gb",

    // "individual/01-special.gb",
    // "individual/02-interrupts.gb",
    // "individual/03-op sp,hl.gb",
    // "individual/04-op r,imm.gb",
    // "individual/05-op rp.gb",
    // "individual/06-ld r,r.gb",
    // "individual/07-jr,jp,call,ret,rst.gb",
    // "individual/08-misc instrs.gb",
    // "individual/09-op r,r.gb",
    // "individual/10-bit ops.gb",
    // "individual/11-op a,(hl).gb",
];

const GAME_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/games/Tetris.gb";

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

    // gamboy.run(CartridgeOption::NONE, false);

    // gamboy.run(CartridgeOption::SOME(Cartridge::new(GAME_FILEPATH.as_ref())), true);

    for test_rom in TEST_ROMS {
        let filepath = format!("{}{}", TEST_ROM_DIR, test_rom);
        println!("============ BEGIN TEST {} ============", test_rom);
        gamboy.run(CartridgeOption::SOME(Cartridge::new(filepath.as_str().as_ref())), true);
        println!("============ END TEST {} ============\n\n\n", test_rom);
    }
}
