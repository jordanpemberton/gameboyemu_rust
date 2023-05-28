use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::console::ppu::{Lcd, LCD_PIXEL_HEIGHT, LCD_PIXEL_WIDTH, Ppu};

const COLORS: [Color; 12] = [
    Color::RGB(0, 0, 0),
    Color::RGB(255, 255, 0),
    Color::RGB(255, 0, 255),
    Color::RGB(0, 255, 255),

    Color::RGB(0, 0, 0),
    Color::RGB(128, 255, 0),
    Color::RGB(128, 0, 255),
    Color::RGB(0, 128, 255),

    Color::RGB(0, 0, 0),
    Color::RGB(64, 122, 0),
    Color::RGB(64, 0, 128),
    Color::RGB(0, 64, 128),
];

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

pub(crate) struct Display {
    enabled: bool,
    canvas: WindowCanvas,
    pixels: [[Rect; LCD_PIXEL_WIDTH]; LCD_PIXEL_HEIGHT],
}

impl Display {
    pub(crate) fn new(
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
            canvas:
                create_sdl_canvas(
                    sdl_context,
                    LCD_PIXEL_WIDTH as u32 * window_scale,
                    LCD_PIXEL_HEIGHT as u32 * window_scale,
                    window_title
                ),
            pixels,
        }
    }

    pub(crate) fn draw(&mut self, ppu: &mut Ppu) {
        if !self.enabled {
            return;
        }

        self.canvas.clear();

        self.draw_screen(&ppu.lcd);

        self.canvas.present();
        self.canvas.set_draw_color(COLORS[0]);
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
