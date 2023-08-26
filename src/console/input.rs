use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::console::mmu::Mmu;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum JoypadInput {
    InputKeyUp,
    InputKeyDown,
    InputKeyLeft,
    InputKeyRight,
    InputKeyStart,
    InputKeySelect,
    InputKeyA,
    InputKeyB,
}

#[derive(Debug)]
pub(crate) enum Callback {
    DebugBreak,
    DebugPeek,
    DebugPrintScreen,
    Exit,
    InputKeyUp,
    InputKeyDown,
    InputKeyLeft,
    InputKeyRight,
    InputKeyStart,
    InputKeySelect,
    InputKeyA,
    InputKeyB,
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

    pub(crate) fn poll(&mut self, mmu: &mut Mmu) -> Vec<Callback> {
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
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    callbacks.push(Callback::InputKeyDown);
                    mmu.input_queue.push(JoypadInput::InputKeyDown);
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    callbacks.push(Callback::InputKeyUp);
                    mmu.input_queue.push(JoypadInput::InputKeyUp);
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    callbacks.push(Callback::InputKeyLeft);
                    mmu.input_queue.push(JoypadInput::InputKeyLeft);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    callbacks.push(Callback::InputKeyRight);
                    mmu.input_queue.push(JoypadInput::InputKeyRight);
                }
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    callbacks.push(Callback::InputKeyStart);
                    mmu.input_queue.push(JoypadInput::InputKeyStart);
                }
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    callbacks.push(Callback::InputKeySelect);
                    mmu.input_queue.push(JoypadInput::InputKeySelect);
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    callbacks.push(Callback::InputKeyB);
                    mmu.input_queue.push(JoypadInput::InputKeyB);
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    callbacks.push(Callback::InputKeyA);
                    mmu.input_queue.push(JoypadInput::InputKeyA);
                }
                _ => { }
            }
        }
        callbacks
    }
}
