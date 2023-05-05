use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use sdl2::sys::rand;

use crate::console::mmu::{Endianness, Mmu};
use crate::console::ppu::{Lcd, LCD_PIXEL_HEIGHT, LCD_PIXEL_WIDTH, Ppu};

const COLORS: [Color; 8] = [
    Color::RGB(0, 0, 0),
    Color::RGB(255, 255, 0),
    Color::RGB(255, 0, 255),
    Color::RGB(0, 255, 255),

    Color::RGB(0, 0, 0),
    Color::RGB(122, 255, 0),
    Color::RGB(122, 0, 255),
    Color::RGB(0, 122, 255),
];

pub(crate) struct Display {
    enabled: bool,
    gbpixel_width: u32,
    gbpixel_height: u32,
    gbpixel_size: u32,
    canvas: WindowCanvas,
    pixels: [[Rect; LCD_PIXEL_WIDTH]; LCD_PIXEL_HEIGHT],
}

impl Display {
    pub(crate) fn new(
            pixel_width: u32,
            pixel_height: u32,
            window_scale: u32,
            window_title: &str,
            sdl_context: &Sdl,
            enabled: bool) -> Display {
        let mut pixels = [[Rect::new(0,0,0,0); LCD_PIXEL_WIDTH]; LCD_PIXEL_HEIGHT];
        for y in 0..LCD_PIXEL_HEIGHT as u32 {
            for x in 0..LCD_PIXEL_WIDTH as u32 {
                pixels[y as usize][x as usize] = Rect::new(
                    (x * window_scale) as i32,
                    (y * window_scale) as i32,
                    window_scale,
                    window_scale);
            }
        }

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
            pixels,
        }
    }

    pub(crate) fn draw(&mut self, mmu: &mut Mmu, ppu: &mut Ppu) {
        if !self.enabled {
            return;
        }

        self.canvas.clear();

        self.draw_screen(&ppu.lcd);

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
        for x in 0..LCD_PIXEL_WIDTH as u32 {
            let color = scanline[x as usize];
            self.canvas.set_draw_color(color);
            self.canvas.fill_rect(self.pixels[y as usize][x as usize]).unwrap();
        }
    }

    fn draw_screen(&mut self, lcd: &Lcd) {
        for y in 0..LCD_PIXEL_HEIGHT {
            let row = lcd.data[y as usize];
            let colors: [Color; LCD_PIXEL_WIDTH] = row.map(|pixel| COLORS[pixel as usize]);
            self.draw_scanline(y as u32, colors);
        }
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
