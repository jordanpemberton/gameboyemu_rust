use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::console::mmu::Mmu;

#[allow(dead_code)]
#[derive(Debug, Eq, Hash, PartialEq)]
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
                    mmu.active_input.insert(JoypadInput::InputKeyDown);
                }
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    mmu.active_input.remove(&JoypadInput::InputKeyDown);
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    callbacks.push(Callback::InputKeyUp);
                    mmu.active_input.insert(JoypadInput::InputKeyUp);
                }
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    mmu.active_input.remove(&JoypadInput::InputKeyUp);
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    callbacks.push(Callback::InputKeyLeft);
                    mmu.active_input.insert(JoypadInput::InputKeyLeft);
                }
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    mmu.active_input.remove(&JoypadInput::InputKeyLeft);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    callbacks.push(Callback::InputKeyRight);
                    mmu.active_input.insert(JoypadInput::InputKeyRight);
                }
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    mmu.active_input.remove(&JoypadInput::InputKeyRight);
                }
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    callbacks.push(Callback::InputKeyStart);
                    mmu.active_input.insert(JoypadInput::InputKeyStart);
                }
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
                    mmu.active_input.remove(&JoypadInput::InputKeyStart);
                }
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    callbacks.push(Callback::InputKeySelect);
                    mmu.active_input.insert(JoypadInput::InputKeySelect);
                }
                Event::KeyUp { keycode: Some(Keycode::X), .. } => {
                    mmu.active_input.remove(&JoypadInput::InputKeySelect);
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    callbacks.push(Callback::InputKeyB);
                    mmu.active_input.insert(JoypadInput::InputKeyB);
                }
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    mmu.active_input.remove(&JoypadInput::InputKeyB);
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    callbacks.push(Callback::InputKeyA);
                    mmu.active_input.insert(JoypadInput::InputKeyA);
                }
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    mmu.active_input.remove(&JoypadInput::InputKeyA);
                }
                _ => { }
            }
        }
        callbacks
    }
}
