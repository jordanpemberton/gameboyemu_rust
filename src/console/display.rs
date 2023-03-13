use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::console::mmu::{MemoryType, Mmu};
use crate::console::ppu::Ppu;

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
        self.draw_sdl(mmu, ppu);
    }

    fn draw_sdl(&mut self, mmu: &mut Mmu, ppu: &mut Ppu) {
        // TODO diplay correctly (this just displays raw VRAM mem data)
        let vram = mmu.get_memory_buffer(&MemoryType::VRAM);
        let pixel_buffer = ppu.get_pixel_buffer(vram, 0);

        self.canvas.clear();

        let mut i: usize = 0;
        for row in 0..self.gbpixel_height {
            for column in 0..self.gbpixel_width {
                if i >= pixel_buffer.len() {
                    break
                };
                let color = pixel_buffer[i];
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

        self.canvas.present();
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
