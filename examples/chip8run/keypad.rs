use chip8vm;

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    PRESSED,
    RELEASED,
}

pub struct Keypad {
    keys: [KeyState; chip8vm::KEYPAD_NUM_KEYS],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [KeyState::RELEASED; chip8vm::KEYPAD_NUM_KEYS],
        }
    }

    pub fn key_pressed(&mut self, keycode: u8) {
        self.keys[keycode as usize] = KeyState::PRESSED;
    }

    pub fn key_released(&mut self, keycode: u8) {
        self.keys[keycode as usize] = KeyState::RELEASED;
    }
}

impl chip8vm::Keypad for Keypad {
    fn is_pressed(&self, keycode: u8) -> bool {
        match self.keys[keycode as usize] {
            KeyState::PRESSED => true,
            KeyState::RELEASED => false,
        }
    }

    fn pressed_key(&self) -> Option<u8> {
        for (keycode, state) in self.keys.iter().enumerate() {
            match state {
                KeyState::PRESSED => {
                    return Some(keycode as u8);
                }
                KeyState::RELEASED => continue,
            }
        }

        None
    }
}
