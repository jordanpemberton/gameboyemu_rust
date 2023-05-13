use std::{fs, io};
use std::fs::DirEntry;
use std::path::PathBuf;

use crate::cartridge::cartridge::{Cartridge, CartridgeOption};
use crate::console::console::Console;
use crate::console::disassembler;
use crate::console::cpu_registers::CpuRegisters;

const BOOTROM_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/bootrom/dmg.bin";
const ROM_DIR: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/";

fn select_rom(is_contained: bool) -> String {
    let root_path = PathBuf::from(ROM_DIR);
    let mut curr_path = PathBuf::from(ROM_DIR);
    let mut selection = -1;

    while curr_path.is_dir() {
        println!("\n0: {:}", if is_contained && curr_path == root_path {
                String::from("N/A")
            } else {
                format!("↑ {}/", curr_path.parent().unwrap().file_name().unwrap().to_string_lossy())
            });

        let mut paths: Vec<DirEntry> = fs::read_dir(curr_path.as_path()).unwrap()
            .map(|e| e.unwrap()).collect();
        paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        let mut child_paths = vec!();
        for path in paths.iter() {
            if path.path().is_dir() {
                child_paths.push(path);
            }
        }
        for path in paths.iter() {
            if !path.path().is_dir() && !path.path().is_symlink() && !path.file_name().to_string_lossy().contains(".zip") {
                child_paths.push(path);
            }
        }

        for (i, child_path) in child_paths.iter().enumerate() {
            println!("{}: {:}{}",
                i + 1,
                child_path.file_name().to_string_lossy(),
                if child_path.path().is_dir() { "/" } else { "" });
        }

        let mut buffer = String::new();
        while selection < 0 {
            buffer.clear();
            io::stdin().read_line(&mut buffer).unwrap();
            selection = buffer.trim().parse::<i32>().expect("Please input the number of the path you'd like to select.");
        }

        if selection == 0 {
            if is_contained && curr_path == root_path {
                println!("Nope, not allowed.");
            } else {
                curr_path = curr_path.parent().unwrap().to_path_buf();
            }
        } else if 0 < selection && selection <= child_paths.len() as i32 {
            curr_path = PathBuf::from(child_paths.get(selection as usize - 1).unwrap().path());
        }
        selection = -1;
    }

    String::from(curr_path.to_str().unwrap())
}

fn disassemble_rom(filepath: &str) {
    let cartridge = Cartridge::new(filepath.as_ref());
    let name = *filepath.split('/').collect::<Vec<&str>>()
        .last().unwrap()
        .split('.').collect::<Vec<&str>>()
        .first().unwrap();
    let path = format!("/home/jordan/RustProjs/GameBoyEmu/out/disassemble__{}.txt", name);
    disassembler::disassemble_to_output_file(&cartridge.data, path.as_str());
}

fn run_rom(rom_filepath: &str, display_enabled: bool, debug_enabled: bool, skip_boot: bool, cpu_debug_print: bool) {
    let mut gamboy = Console::new("GAMBOY", 6, display_enabled, debug_enabled, cpu_debug_print);

    if rom_filepath.is_empty() {
        disassemble_rom(BOOTROM_FILEPATH);
        gamboy.run(CartridgeOption::NONE, false);
    } else {
        disassemble_rom(rom_filepath);
        gamboy.run(CartridgeOption::SOME(Cartridge::new(rom_filepath.as_ref())), skip_boot);
    }
}

pub(crate) fn run(args: Vec<String>, is_contained: bool) {
    let display_enabled = if args.len() > 1 { args[1].clone().parse().unwrap() } else { 1 } > 0;
    let debug_enabled = if args.len() > 2 { args[2].clone().parse().unwrap() } else { 1 } > 0;
    let skip_boot = if args.len() > 3 { args[3].clone().parse().unwrap() } else { 0 } > 0;
    let cpu_debug_print = if args.len() > 4 { args[4].clone().parse().unwrap() } else { 1 } > 0;

    CpuRegisters::test();

    let selection = select_rom(is_contained);
    println!("\nSELECTED ROM: {}\n", selection);

    run_rom(selection.as_str(), display_enabled, debug_enabled, skip_boot, cpu_debug_print);
}
