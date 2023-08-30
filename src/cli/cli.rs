use std::{env, fs, io};
use std::fs::DirEntry;
use std::path::PathBuf;

use crate::cartridge::cartridge::Cartridge;
use crate::console::console::Console;
use crate::console::disassembler;
use crate::console::cpu_registers::CpuRegisters;

const BOOTROM_FILEPATH: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/bootrom/dmg.bin";
const ROM_DIR: &str = "/home/jordan/RustProjs/GameBoyEmu/roms/";

struct EmuArgs {
    skip_boot: bool,
    debug_enabled: bool,
    cpu_instr_log: bool,
    lcd_debug: bool,
    rom_filepath: String,
}

impl EmuArgs {
    fn new() -> EmuArgs {
        // EmuArgs::parse_args_v1()
        // EmuArgs::parse_args_v2()
        EmuArgs::parse_args_v3()
    }

    #[allow(dead_code)]
    fn parse_args_v1() -> EmuArgs {
        let args: Vec<String> = env::args().collect();
        EmuArgs {
            rom_filepath: if args.len() > 1 { args[1].clone() } else { String::new() },
            skip_boot: if args.len() > 2 { args[2].clone().parse().unwrap() } else { 1 } > 0,
            debug_enabled: if args.len() > 3 { args[3].clone().parse().unwrap() } else { 1 } > 0,
            cpu_instr_log: if args.len() > 4 { args[4].clone().parse().unwrap() } else { 0 } > 0,
            lcd_debug: if args.len() > 5 { args[5].clone().parse().unwrap() } else { 0 } > 0,
        }
    }

    #[allow(dead_code)]
    fn parse_args_v2() -> EmuArgs {
        let mut args = env::args().skip(1);
        let mut skip_boot = true;
        let mut debug_enabled = true;
        let mut cpu_instr_log = false;
        let mut lcd_debug = false;
        let mut rom_filepath = String::new();

        let num_args = 5;
        let mut i = 0;
        while i < num_args {
            if let Some(arg) = args.next() {
                match &arg[..] {
                    "--noboot" => {
                        if let Some(arg) = args.next() {
                            skip_boot = arg.parse::<u8>().unwrap() > 0;
                        }
                    }
                    "--debug" => {
                        if let Some(arg) = args.next() {
                            debug_enabled = arg.parse::<u8>().unwrap() > 0;
                        }
                    }
                    "--cpulog" => {
                        if let Some(arg) = args.next() {
                            cpu_instr_log = arg.parse::<u8>().unwrap() > 0;
                        }
                    }
                    "--lcddebug" => {
                        if let Some(arg) = args.next() {
                            lcd_debug = arg.parse::<u8>().unwrap() > 0;
                        }
                    }
                    "--rom" => {
                        if let Some(arg) = args.next() {
                            rom_filepath = arg;
                        }
                    }
                    _ => {
                        match i {
                            0 => { rom_filepath = arg; }
                            1 => { debug_enabled = arg.parse::<u8>().unwrap() > 0; },
                            2 => { cpu_instr_log = arg.parse::<u8>().unwrap() > 0; },
                            3 => { lcd_debug = arg.parse::<u8>().unwrap() > 0; }
                            _ => { skip_boot = arg.parse::<u8>().unwrap() > 0; },
                        }
                    }
                }
                i += 1;
            } else {
                break;
            }
        }

        EmuArgs {
            skip_boot,
            debug_enabled,
            cpu_instr_log,
            lcd_debug,
            rom_filepath,
        }
    }

    #[allow(dead_code)]
    fn parse_args_v3() -> EmuArgs {
        let mut args = env::args().skip(1);
        let mut skip_boot = true;
        let mut debug_enabled = true;
        let mut cpu_instr_log = false;
        let mut lcd_debug = false;
        let mut rom_filepath = String::new();

        while let Some(arg) = args.next() {
            match &arg[..] {
                "--noboot" => {
                    if let Some(arg) = args.next() {
                        skip_boot = arg.parse::<u8>().unwrap() > 0;
                    }
                },
                "--debug" => {
                    if let Some(arg) = args.next() {
                        debug_enabled = arg.parse::<u8>().unwrap() > 0;
                    }
                },
                "--cpulog" => {
                    if let Some(arg) = args.next() {
                        cpu_instr_log = arg.parse::<u8>().unwrap() > 0;
                    }
                },
                "--lcddebug" => {
                    if let Some(arg) = args.next() {
                        lcd_debug = arg.parse::<u8>().unwrap() > 0;
                    }
                },
                "--rom" => {
                    if let Some(arg) = args.next() {
                        rom_filepath = arg;
                    }
                },
                _ => {
                    if arg.starts_with('-') {
                        println!("Unknown argument {}", arg);
                    } else {
                        println!("Unknown positional argument {}", arg);
                    }
                }
            }
        }

        EmuArgs {
            skip_boot,
            debug_enabled,
            cpu_instr_log,
            lcd_debug,
            rom_filepath,
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

fn disassemble_rom(filepath: &str) {
    let cartridge = Cartridge::new(filepath.as_ref());
    let name = *filepath.split('/').collect::<Vec<&str>>()
        .last().unwrap()
        .split('.').collect::<Vec<&str>>()
        .first().unwrap();
    let path = format!("/home/jordan/RustProjs/GameBoyEmu/out/disassemble__{}.txt", name);
    disassembler::disassemble_to_output_file(&cartridge.data, path.as_str());
}

fn run_rom(args: &EmuArgs) { //rom_filepath: &str, skip_boot: bool, debug_enabled: bool, cpu_instr_log: bool, lcd_debug: bool) {
    let window_scale = if args.lcd_debug { 4 } else { 7 };

    if args.rom_filepath.trim().is_empty() {
        disassemble_rom(BOOTROM_FILEPATH);
        let mut gamboy = Console::new(
            "GAMBOY",
            window_scale,
            args.debug_enabled,
            args.cpu_instr_log,
            args.lcd_debug,
            None
        );
        gamboy.run(false);
    } else {
        disassemble_rom(args.rom_filepath.as_str());
        let cartridge = Cartridge::new(args.rom_filepath.as_ref());
        let mut gamboy = Console::new(
            "GAMBOY",
            window_scale,
            args.debug_enabled,
            args.cpu_instr_log,
            args.lcd_debug,
            Some(cartridge)
        );
        gamboy.run(args.skip_boot);
    }
}

pub(crate) fn run() {
    CpuRegisters::test();

    let mut args = EmuArgs::new();

    if String::is_empty(&args.rom_filepath) {
        args.rom_filepath = select_rom(true);
    }

    run_rom(&args);
}
