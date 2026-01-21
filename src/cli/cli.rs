use std::{env, fs, io};
use std::fs::DirEntry;
use std::path::PathBuf;

use crate::cartridge::cartridge::Cartridge;
use crate::console::console::Console;
use crate::console::disassembler;

const BOOTROM_FILEPATH: &str = "./roms/bootrom/dmg.bin";
const ROM_DIR: &str = "./roms";
const DISASSEMBLE_OUTPUT_DIR: &str = "./out";
const SKIP_BOOT_FLAG_STRING: &str = "skipboot";
const DEBUG_FLAG_STRING: &str = "debug";
const PRINT_CPU_FLAG_STRING: &str = "printcpu";


struct EmuArgs {
    skip_boot: bool,
    debug_enabled: bool,
    print_cpu_instrs: bool,
    rom_filepath: String,
}

impl EmuArgs {
    fn new() -> EmuArgs {
        EmuArgs::parse_args()
    }

    fn parse_args() -> EmuArgs {
        let args: Vec<String> = env::args().collect();

        // Defaults
        let mut rom_filepath = String::new();
        let mut skip_boot = false;
        let mut debug_enabled = false;
        let mut print_cpu_instrs = false;

        if args.len() > 1 {
            rom_filepath = args[1].clone();
            skip_boot = args.contains(&String::from(SKIP_BOOT_FLAG_STRING));
            debug_enabled = args.contains(&String::from(DEBUG_FLAG_STRING));
            print_cpu_instrs = args.contains(&String::from(PRINT_CPU_FLAG_STRING));
        }

        EmuArgs {
            rom_filepath,
            skip_boot,
            debug_enabled,
            print_cpu_instrs,
        }
    }
}

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

    let rom_filepath = String::from(curr_path.to_str().unwrap());
    println!("\nSELECTED ROM: {}\n", rom_filepath);
    rom_filepath
}

fn disassemble_rom(filepath: &str, out_path: &str) {
    let cartridge = Cartridge::new(filepath.as_ref());
    let name = *filepath.split('/').collect::<Vec<&str>>()
        .last().unwrap()
        .split('.').collect::<Vec<&str>>()
        .first().unwrap();
    let path = format!("{}/disassemble__{}.txt", out_path, name);
    disassembler::disassemble_to_output_file(&cartridge.data, path.as_str());
}

fn run_rom(args: &EmuArgs) {
    let window_scale = 5;

    if args.rom_filepath.trim().is_empty() {
        disassemble_rom(BOOTROM_FILEPATH, DISASSEMBLE_OUTPUT_DIR);
        let skip_boot = false;
        let mut gamboy = Console::new(
            "GAMBOY",
            window_scale,
            args.debug_enabled,
            args.print_cpu_instrs,
            skip_boot,
            None
        );
        gamboy.run();
    } else {
        disassemble_rom(args.rom_filepath.as_str(), DISASSEMBLE_OUTPUT_DIR);
        let cartridge = Cartridge::new(args.rom_filepath.as_ref());
        let mut gamboy = Console::new(
            "GAMBOY",
            window_scale,
            args.debug_enabled,
            args.print_cpu_instrs,
            args.skip_boot,
            Some(cartridge)
        );
        gamboy.run();
    }
}

pub(crate) fn run() {
    let mut args = EmuArgs::new();

    if String::is_empty(&args.rom_filepath) {
        args.rom_filepath = select_rom(true);
    }

    run_rom(&args);
}
