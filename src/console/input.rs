use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::console::mmu;
use crate::console::mmu::Mmu;

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
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    callbacks.push(Callback::InputKeyUp);
                    mmu.input_queue.push(JoypadInput::InputKeyUp);
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    callbacks.push(Callback::InputKeyDown);
                    mmu.input_queue.push(JoypadInput::InputKeyDown);
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    callbacks.push(Callback::InputKeyLeft);
                    mmu.input_queue.push(JoypadInput::InputKeyLeft);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    callbacks.push(Callback::InputKeyRight);
                    mmu.input_queue.push(JoypadInput::InputKeyRight);
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    callbacks.push(Callback::InputKeyStart);
                    mmu.input_queue.push(JoypadInput::InputKeyStart);
                }
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    callbacks.push(Callback::InputKeySelect);
                    mmu.input_queue.push(JoypadInput::InputKeySelect);
                }
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    callbacks.push(Callback::InputKeyA);
                    mmu.input_queue.push(JoypadInput::InputKeyA);
                }
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    callbacks.push(Callback::InputKeyB);
                    mmu.input_queue.push(JoypadInput::InputKeyB);
                }
                _ => { }
            }
        }
        callbacks
    }

    pub(crate) fn read_joypad_input(input: &JoypadInput, reg_value: u8) -> u8 {
        let select_action_is_enabled = (reg_value & (1 << 4)) >> 4 == 1;
        let directional_is_enabled = (reg_value & (1 << 5)) >> 5 == 1;

        let input_bit_value = if select_action_is_enabled {
            1 << (match input {
                JoypadInput::InputKeyStart => 3,
                JoypadInput::InputKeySelect => 2,
                JoypadInput::InputKeyB => 1,
                JoypadInput::InputKeyA | _ => 0,
            })
        } else if directional_is_enabled {
            1 << (match input {
                JoypadInput::InputKeyDown => 3,
                JoypadInput::InputKeyUp => 2,
                JoypadInput::InputKeyLeft => 1,
                JoypadInput::InputKeyRight | _ => 0,
            })
        } else {
            0
        };

        !input_bit_value
    }
}
