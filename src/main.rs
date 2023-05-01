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

const BOOTROM_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/bootrom/dmg.bin";

const GAME_FILEPATH: &str =
    // "/home/jordan/Games/GameBoy/GB/Tetris.gb"
    "/home/jordan/Games/GameBoy/GB/Dr. Mario (Game Boy Prototype)/Dr. Mario (Prototype).gb"
    // "/home/jordan/Games/GameBoy/GB/Kirby's Dream Land (USA, Europe).gb" // hits Invalid opcode F4 at 0x4C4A?
;

const BLARGG_TEST_ROM_DIR: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/";
const BLARGG_TEST_ROMS: [&str; 12] = [
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
    "individual/11-op a,(hl).gb"
];

const MEALY_TEST_ROM_DIR: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/mealybug/mealybug-tearoom-tests/";
const MEALY_TEST_ROMS: [&str; 12] = [
    "m2_win_en_toggle.gb",
    "m3_bgp_change.gb",
    "m3_bgp_change_sprites.gb",   // invalid instructions
    "m3_lcdc_bg_en_change.gb",
    "m3_lcdc_bg_en_change2.gb",
    "m3_lcdc_bg_map_change.gb",
    "m3_lcdc_bg_map_change2.gb",
    "m3_lcdc_obj_en_change.gb",
    "m3_lcdc_obj_en_change_variant.gb",
    "m3_lcdc_obj_size_change.gb",
    "m3_lcdc_obj_size_change_scx.gb",
    "m3_scy_change.gb",
];

const TEST_IS_BLARGG: bool = false;
const TEST_ROM: usize = 0;

fn disassemble_roms() {
    let bootrom_cartridge = Cartridge::new(BOOTROM_FILEPATH.as_ref());
    disassembler::disassemble_to_output_file(&bootrom_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/bootrom_disasseble.txt");

    let test_roms_dir = if TEST_IS_BLARGG { BLARGG_TEST_ROM_DIR } else { MEALY_TEST_ROM_DIR };
    let test_roms = if TEST_IS_BLARGG { BLARGG_TEST_ROMS } else { MEALY_TEST_ROMS };
    for test_rom in test_roms {
        let filepath = format!("{}{}", test_roms_dir, test_rom);
        let test_cartridge = Cartridge::new(filepath.as_ref());

        let name = *test_rom.split('/').collect::<Vec<&str>>()
            .last().unwrap()
            .split('.').collect::<Vec<&str>>()
            .first().unwrap();

        let path = format!("/home/jordan/RustProjs/GameBoyEmu/out/testrom_disassemble__{}.txt", name);
        disassembler::disassemble_to_output_file(&test_cartridge.data, path.as_str());
    }

    let game_cartridge = Cartridge::new(GAME_FILEPATH.as_ref());

    let name = *GAME_FILEPATH.split('/').collect::<Vec<&str>>()
        .last().unwrap()
        .split('.').collect::<Vec<&str>>()
        .first().unwrap();

    let path = format!("/home/jordan/RustProjs/GameBoyEmu/out/game_disassemble__{}.txt", name);
    disassembler::disassemble_to_output_file(&game_cartridge.data, path.as_str());
}

fn main() {
    disassemble_roms();

    CpuRegisters::test();

    let mut gamboy = Console::new("GAMBOY",5, true, true);

    // gamboy.run(CartridgeOption::NONE, false);

    // gamboy.run(CartridgeOption::SOME(Cartridge::new(GAME_FILEPATH.as_ref())), true);

    let test_roms_dir = if TEST_IS_BLARGG { BLARGG_TEST_ROM_DIR } else { MEALY_TEST_ROM_DIR };
    let test_roms = if TEST_IS_BLARGG { BLARGG_TEST_ROMS } else { MEALY_TEST_ROMS };
    let test_filepath = format!("{}{}", test_roms_dir, test_roms[TEST_ROM]);
    gamboy.run(CartridgeOption::SOME(Cartridge::new(test_filepath.as_str().as_ref())), true);
}
