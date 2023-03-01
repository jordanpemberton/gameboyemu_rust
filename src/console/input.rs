use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub(crate) enum CallbackAction {
    ESCAPE,
    PANIC,
    STEP,
}

pub(crate) struct Input {
    event_pump: EventPump,
}

impl Input {
    pub(crate) fn new(sdl_context: &Sdl) -> Input {
        let event_pump: EventPump = sdl_context.event_pump().unwrap();

        Input {
            event_pump: event_pump,
        }
    }

    pub(crate) fn poll(&mut self) -> CallbackAction {
        for event in self.event_pump.poll_iter() {
            return match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    CallbackAction::ESCAPE
                },
                _ => {
                    CallbackAction::STEP
                }
            }
        }
        CallbackAction::PANIC
    }
}
