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

const GAME_FILEPATH: &str =
    "/home/jordan/RustProjs/GameBoyEmu/roms/games/Tetris.gb"
    // "/home/jordan/Games/GameBoy/GB/Pokemon - Red Version (USA, Europe) (SGB Enhanced).gb"
    // "/home/jordan/Games/GameBoy/GB/Pokemon - Blue Version (USA, Europe) (SGB Enhanced).gb"
;

const BLARGG_TEST_ROM_DIR: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/";
// const MEALY_TEST_ROM_DIR: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/mealybug/mealybug-tearoom-tests/";

const TEST_ROMS: [(&str, &str); 12] = [
    (BLARGG_TEST_ROM_DIR, "cpu_instrs.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/01-special.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/02-interrupts.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/03-op sp,hl.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/04-op r,imm.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/05-op rp.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/06-ld r,r.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/07-jr,jp,call,ret,rst.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/08-misc instrs.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/09-op r,r.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/10-bit ops.gb"),
    (BLARGG_TEST_ROM_DIR, "individual/11-op a,(hl).gb"),

    // (MEALY_TEST_ROM_DIR, "m2_win_en_toggle.gb"),        // line 152 out of range for hblank
    // (MEALY_TEST_ROM_DIR, "m3_bgp_change.gb"),           // line 152 out of range for hblank
    // (MEALY_TEST_ROM_DIR, "m3_bgp_change_sprites.gb"),   // invalid instructions
    // (MEALY_TEST_ROM_DIR, "m3_lcdc_bg_en_change.gb"),    // line 152 out of range for hblank
    // (MEALY_TEST_ROM_DIR, "m3_lcdc_bg_en_change2.gb"),
    // (MEALY_TEST_ROM_DIR, "m3_lcdc_bg_map_change.gb"),   // line 152 out of range for hblank
    // (MEALY_TEST_ROM_DIR, "m3_lcdc_bg_map_change2.gb"),
    // (MEALY_TEST_ROM_DIR, "m3_lcdc_obj_en_change.gb"),           // line 152 out of range for hblank
    // (MEALY_TEST_ROM_DIR, "m3_lcdc_obj_en_change_variant.gb"),   // line 152 out of range for hblank
    // (MEALY_TEST_ROM_DIR, "m3_lcdc_obj_size_change.gb"),         // line 152 out of range for hblank
    // (MEALY_TEST_ROM_DIR, "m3_lcdc_obj_size_change_scx.gb"),     // line 152 out of range for hblank
];

const TEST_ROM: usize = 11;

fn disassemble_roms() {
    let bootrom_cartridge = Cartridge::new(BOOTROM_FILEPATH.as_ref());
    disassembler::disassemble_to_output_file(&bootrom_cartridge.data, "/home/jordan/RustProjs/GameBoyEmu/out/bootrom_disasseble.txt");

    for test_rom in TEST_ROMS {
        let filepath = format!("{}{}", test_rom.0, test_rom.1);
        let test_cartridge = Cartridge::new(filepath.as_ref());

        let mut parts = test_rom.1.split('.').collect::<Vec<&str>>();
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

    gamboy.run(CartridgeOption::SOME(Cartridge::new(GAME_FILEPATH.as_ref())), true);

    let test_filepath = format!("{}{}", TEST_ROMS[TEST_ROM].0, TEST_ROMS[TEST_ROM].1);
    // gamboy.run(CartridgeOption::SOME(Cartridge::new(test_filepath.as_str().as_ref())), true);
}
