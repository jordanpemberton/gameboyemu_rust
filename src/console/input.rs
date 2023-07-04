use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub(crate) enum Callback {
    DebugBreak,
    DebugPeek,
    DebugPrintScreen,
    Exit,
}

pub(crate) struct Input {
    event_pump: EventPump,
}

impl Input {
    pub(crate) fn new(sdl_context: &mut Sdl) -> Input {
        Input {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    pub(crate) fn poll(&mut self) -> Vec<Callback> {
        let mut callbacks: Vec<Callback> = vec![];
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    callbacks.push(Callback::Exit);
                }
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    callbacks.push(Callback::DebugBreak);
                }
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    callbacks.push(Callback::DebugPeek);
                }
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    callbacks.push(Callback::DebugPrintScreen);
                }
                _ => { }
            }
        }
        callbacks
    }
}
