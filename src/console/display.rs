use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::console::mmu::{Endianness, Mmu};
use crate::console::ppu::{Lcd, LCD_PIXEL_HEIGHT, LCD_PIXEL_WIDTH, Ppu};

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

// How do these get drawn correctly?
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

const COLORS: [Color; 4] = [
    Color::RGB(0, 0, 0),
    Color::RGB(255, 255, 0),
    Color::RGB(255, 0, 255),
    Color::RGB(0, 255, 255),
];

pub(crate) struct Display {
    enabled: bool,
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
            sdl_context: &Sdl,
            enabled: bool) -> Display {
        Display {
            enabled: enabled,
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
        if !self.enabled {
            return;
        }

        self.canvas.clear();

        self.draw_screen(&ppu.background);
        // self.draw_screen(&ppu.window);
        // self.draw_screen(&ppu.sprites);

        self.canvas.present();
        self.canvas.set_draw_color(COLORS[0]);
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

    fn draw_scanline(&mut self, y: u32, scanline: [Color; LCD_PIXEL_WIDTH as usize]) {
        for x in 0..LCD_PIXEL_WIDTH {
            let color = scanline[x as usize];
            self.canvas.set_draw_color(color);

            let pixel_rect = Rect::new(
                (x * self.gbpixel_size) as i32,
                (y * self.gbpixel_size) as i32,
                self.gbpixel_size,
                self.gbpixel_size);

            self.canvas.fill_rect(pixel_rect).unwrap();
        }
    }

    fn draw_screen(&mut self, lcd: &Lcd) {
        for y in 0..LCD_PIXEL_HEIGHT {
            let colors: [Color; LCD_PIXEL_WIDTH as usize] = lcd.data[y as usize].map(|pixel| COLORS[pixel as usize]);
            self.draw_scanline(y, colors);
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
