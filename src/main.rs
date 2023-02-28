#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod console;
mod cartridge;

use crate::cartridge::cartridge::Cartridge;
use crate::console::cpu::Cpu;
use crate::console::mmu::Mmu;
use crate::console::registers::{RegIndex};

use sdl2::{
    event::Event,
    EventPump,
    keyboard::Keycode,
    // pixels::Color,
    // rect::Rect,
    // render::Canvas,
    render::WindowCanvas,
    Sdl,
    // video::Window
};

use std::fs::{read};

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 250;

fn setup_canvas(sdl_context: &Sdl, game_path: &str) -> WindowCanvas {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(game_path, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    canvas.clear();
    canvas.present();
    canvas
}

fn run(game: Option<Cartridge>) {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();

    if let Some(mut cartridge) = game {
        run_game(&mut cpu, &mut mmu, &mut cartridge);
    } else {
        boot_empty(&mut cpu, &mut mmu);
    }
}

fn run_game(cpu: &mut Cpu, mmu: &mut Mmu, game: &Cartridge) {
    // TODO load roms correctly, map memory correctly /MBCs
    let bootrom_filepath = "/home/jordan/RustProjs/GameBoyEmu/src/console/DMG_ROM.bin";
    let bootrom = read(bootrom_filepath).unwrap();
    mmu.load_rom(bootrom.as_ref(), 0, bootrom.len());

    mmu.load_rom(&game.data[0x100..], 0x100, 0x8000 - 0x100);
    // cpu.registers.set_word(RegIndex::PC, 0x100);

    let sdl_context: Sdl = sdl2::init().unwrap();
    let mut event_pump: EventPump = sdl_context.event_pump().unwrap();
    let mut canvas: WindowCanvas = setup_canvas(&sdl_context, "Game Name");

    main_loop(cpu, mmu, &mut event_pump, &mut canvas);
}

fn boot_empty(cpu: &mut Cpu, mmu: &mut Mmu) {
    let bootrom_filepath = "/home/jordan/RustProjs/GameBoyEmu/src/console/DMG_ROM.bin";
    let bootrom = read(bootrom_filepath).unwrap();
    mmu.load_rom(bootrom.as_ref(), 0, bootrom.len());

    let sdl_context: Sdl = sdl2::init().unwrap();
    let mut event_pump: EventPump = sdl_context.event_pump().unwrap();
    let mut canvas: WindowCanvas = setup_canvas(&sdl_context, " ");

    main_loop(cpu, mmu, &mut event_pump, &mut canvas);
}

fn main_loop(cpu: &mut Cpu, mmu: &mut Mmu, event_pump: &mut EventPump, canvas: &mut WindowCanvas) {
    let mut max_pc: u16 = 0; // for debugging

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {
                    break 'mainloop;
                },
                _ => {
                    let pc = cpu.registers.get_word(RegIndex::PC);
                    if pc > max_pc {
                        max_pc = pc;
                    }
                    cpu.step(mmu);
                }
            }
        }
    }

    println!("\nMax address reached in program: {:#06X}", max_pc);
}

fn main() {
    let game_filepath = "/home/jordan/Games/GameBoy/GB/Tetris.gb";
    let game_cartridge = Cartridge::new(game_filepath.as_ref());
    let test_rom_filepath = "/home/jordan/RustProjs/GameBoyEmu/roms/test_roms/blargg/cpu_instrs/cpu_instrs.gb";
    let test_cartridge = Cartridge::new(test_rom_filepath.as_ref());

    // run(None);
    // run(Option::from(test_cartridge));
    run(Option::from(game_cartridge));
}
