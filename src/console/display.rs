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

pub(crate) struct Display {
    canvas: WindowCanvas,
}

impl Display {
    pub(crate) fn new(sdl_context: &Sdl, window_width: u32, window_height: u32, window_title: &str) -> Display {
        let canvas = create_canvas(sdl_context, window_width, window_height, window_title);
        Display {
            canvas: canvas,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.canvas.clear();
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
