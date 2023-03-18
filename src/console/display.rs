use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::console::mmu::{Endianness, Mmu};
use crate::console::ppu::Ppu;

const GB_CONSOLE_SPRITE: [u8; 16] = [
    0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
    0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C,
];

const NINTENDO_LOGO_DATA: [u8; 16 * 3] = [
    0xCE, 0xED, 0x66, 0x66,     0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83,     0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F,     0x88, 0x89, 0x00, 0x0E,

    0xDC, 0xCC, 0x6E, 0xE6,     0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63,     0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F,     0xBB, 0xB9, 0x33, 0x3E,
];

const R_DATA: [u8; 16] = [
    0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0,0,0,0, 0,0,0,0,
];

const NINTENDO_TILES: [u8; 16 * 6] = [
    // N
    0xC6, 0, 0xE6, 0, 0xE6, 0, 0xD6, 0,
    0xD6, 0, 0xCE, 0, 0xCE, 0, 0xC6, 0,
    // in
    0xC0, 0, 0xC0, 0, 0x00, 0, 0xDB, 0,
    0xDD, 0, 0xD9, 0, 0xD9, 0, 0xD9, 0,
    // te
    0x00, 0, 0x30, 0, 0x78, 0, 0x33, 0,
    0xB6, 0, 0xB7, 0, 0xB6, 0, 0xB3, 0,
    // n
    0x00, 0, 0x00, 0, 0x00, 0, 0xCD, 0,
    0x6E, 0, 0xEC, 0, 0x0C, 0, 0xEC, 0,
    // d
    0x01, 0, 0x01, 0, 0x01, 0, 0x8F, 0,
    0xD9, 0, 0xD9, 0, 0xD9, 0, 0xCF, 0,
    // o
    0x80, 0, 0x80, 0, 0x80, 0, 0x9E, 0,
    0xB3, 0, 0xB3, 0, 0xB3, 0, 0x9E, 0,
];

const SQUARE_SPRITE: [u8; 16] = [
    0b1111_1111, 0b1111_1111,
    0b1000_0001, 0b1000_0001,
    0b1011_1101, 0b1000_0001,
    0b1010_0101, 0b1001_1001,
    0b1010_0101, 0b1001_1001,
    0b1011_1101, 0b1000_0001,
    0b1000_0001, 0b1000_0001,
    0b1111_1111, 0b1111_1111,
];

const COLORS: [Color; 4] = [
    Color::RGB(0, 0, 0),
    Color::RGB(255, 255, 0),
    Color::RGB(255, 0, 255),
    Color::RGB(0, 255, 255),
];

pub(crate) struct Display {
    gbpixel_width: u32,
    gbpixel_height: u32,
    gbpixel_size: u32,
    canvas: WindowCanvas,
}

impl Display {
    pub(crate) fn new(
            pixel_width: u32,
            pixel_height: u32,
            window_scale: u32,
            window_title: &str,
            sdl_context: &Sdl) -> Display {
        Display {
            gbpixel_width: pixel_width,
            gbpixel_height: pixel_height,
            gbpixel_size: window_scale,
            canvas:
                create_sdl_canvas(
                    sdl_context,
                    pixel_width * window_scale,
                    pixel_height * window_scale,
                    window_title
                ),
        }
    }

    pub(crate) fn draw(&mut self, mmu: &mut Mmu, ppu: &mut Ppu) {
        // TODO Read data from memory for bg, tiles, window, etc.

        self.canvas.clear();

        // Drawing raw tiles stored in vraw
        let tile_bytes: Vec<u8> = mmu.read_buffer(0x8000, 0x9800, Endianness::BIG);

        let tiles = ppu.read_tiles(tile_bytes.as_ref());
        let mut x = 0;
        let mut y = 0;
        for tile in tiles {
            self.draw_tile(x, y, tile);
            x += 8;
            if x > self.gbpixel_width as usize {
                x = 0;
                y += 8;
            }
        }

        self.canvas.present();
    }

    fn draw_tile(&mut self, x: usize, y: usize, tile: [[u8; 8]; 8]) {
        for i in 0..8 {
            for j in 0..8 {
                let color = COLORS[tile[i][j] as usize];
                self.canvas.set_draw_color(color);

                let gbpixel = Rect::new(
                    ((x + j) as u32 * self.gbpixel_size) as i32,
                    ((y + i) as u32 * self.gbpixel_size) as i32,
                    self.gbpixel_size,
                    self.gbpixel_size);

                self.canvas.fill_rect(gbpixel).unwrap();
            }
        }
    }

    fn draw_sdl(&mut self, buffer: Vec<Color>) {
        let mut i: usize = 0;
        for row in 0..self.gbpixel_height {
            for column in 0..self.gbpixel_width {
                if i >= buffer.len() {
                    break
                };
                let color = buffer[i];
                i += 1;

                self.canvas.set_draw_color(color);

                let gbpixel = Rect::new(
                    (column * self.gbpixel_size) as i32,
                    (row * self.gbpixel_size) as i32,
                    self.gbpixel_size,
                    self.gbpixel_size);

                self.canvas.fill_rect(gbpixel).unwrap();
            }
        }
    }

    fn draw_to_stdout(&mut self, pixel_buffer: &[u8]) {
        let mut s: String = String::new();

        let mut i: usize = 0;
        for row in 0..self.gbpixel_height {
            for column in 0..self.gbpixel_width {
                if i >= pixel_buffer.len() {
                    break
                };
                let color = pixel_buffer[i];
                i += 1;
                s.push(if color > 0 { 'X' } else { '.' });
            }
            s.push('\n');
        }
        println!("{}", s);
    }
}

fn create_sdl_canvas(sdl_context: &Sdl, window_width: u32, window_height: u32, window_title: &str) -> WindowCanvas {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(window_title, window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    window.into_canvas()
        .build()
        .unwrap()
}
