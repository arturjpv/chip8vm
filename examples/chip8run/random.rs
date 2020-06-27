use chip8vm;

use rand::prelude::*;

pub struct Random {}

impl chip8vm::Random for Random {
    fn range(&mut self) -> u8 {
        random()
    }
}
