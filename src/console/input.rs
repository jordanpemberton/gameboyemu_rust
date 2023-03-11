use sdl2::{
    EventPump,
    event::Event,
    Sdl,
    keyboard::Keycode
};
use crate::console::{
    debugger::DebugAction
};

pub(crate) enum CallbackAction {
    DEBUG(DebugAction),
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
                }
                Event::KeyDown { keycode: Some(Keycode::B ), .. } => {
                    CallbackAction::DEBUG(DebugAction::BREAK)
                }
                Event::KeyDown { keycode: Some(Keycode::P ), .. } => {
                    CallbackAction::DEBUG(DebugAction::PEEK)
                }
                Event::KeyDown { keycode: Some(Keycode::N ), .. } => {
                    CallbackAction::DEBUG(DebugAction::STEP)
                }
                _ => {
                    CallbackAction::STEP
                }
            }
        }
        CallbackAction::PANIC
    }
}
