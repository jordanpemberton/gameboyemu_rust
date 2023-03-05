use std::collections::HashMap;
use sdl2::{
    event::Event,
    EventPump,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    Sdl,
    video::Window
};

pub(crate) struct Display {
    gbpixel_width: u32,
    gbpixel_height: u32,
    gbpixel_size: u32,
    canvas: WindowCanvas,
}

impl Display {
    pub(crate) fn new(
        sdl_context: &Sdl,
        pixel_width: u32,
        pixel_height: u32,
        window_scale: u32,
        window_title: &str) -> Display {
        Display {
            gbpixel_width: pixel_width,
            gbpixel_height: pixel_height,
            gbpixel_size: window_scale,
            canvas: create_canvas(sdl_context, pixel_width, pixel_height, window_title),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    pub(crate) fn draw_screen(&mut self, pixel_buffer: Vec<Color>) {
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
                    (row * self.gbpixel_size) as i32,
                    (column * self.gbpixel_size) as i32,
                    self.gbpixel_size,
                    self.gbpixel_size);
                self.canvas.fill_rect(gbpixel).unwrap();
            }
        }

        self.canvas.present();
    }
}

fn create_canvas(sdl_context: &Sdl, window_width: u32, window_height: u32, window_title: &str) -> WindowCanvas {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(window_title, window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    window.into_canvas()
        .present_vsync()
        .build()
        .unwrap()
}
