use ggez;
use ggez::conf::WindowMode;
use ggez::event::{quit, KeyCode};
use ggez::graphics::set_window_title;
use ggez::{event, graphics, input, Context, ContextBuilder, GameResult};
use std::fs;
use std::path::Path;

mod cli;
mod random;
mod keypad;
mod screen;

use chip8vm::{chip::Chip, PROGRAM_SIZE};
use random::*;
use keypad::*;
use screen::*;

const TICKS_X_FRAME: u32 = 10;

struct Chip8Run {
    chip: Chip,
    random: Random,
    screen: Screen,
    keypad: Keypad,
}

impl Chip8Run {
    fn new(context: &mut Context, scale: u16) -> GameResult<Chip8Run> {
        let chip = Chip::default();
        let random = Random {};
        let screen = Screen::new(context, scale)?;
        let keypad = Keypad::new();
        let chip8 = Chip8Run {
            chip,
            random,
            screen,
            keypad,
        };
        Ok(chip8)
    }

    fn load_program(&mut self, program_path: String) {
        let program = fs::read(program_path).expect("Unable to load program.");
        let mut program_code: [u8; PROGRAM_SIZE] = [0; PROGRAM_SIZE];
        for (i, data) in program.iter().enumerate() {
            program_code[i] = *data;
        }
        self.chip.load_program(program_code);
    }
}

impl event::EventHandler for Chip8Run {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // get input
        if input::keyboard::is_key_pressed(context, KeyCode::Escape) {
            quit(context);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::Key1) {
            self.keypad.key_pressed(0x1);
        } else {
            self.keypad.key_released(0x1);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::Key2) {
            self.keypad.key_pressed(0x2);
        } else {
            self.keypad.key_released(0x2);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::Key3) {
            self.keypad.key_pressed(0x3);
        } else {
            self.keypad.key_released(0x3);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::Key4) {
            self.keypad.key_pressed(0xC);
        } else {
            self.keypad.key_released(0xC);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::Q) {
            self.keypad.key_pressed(0x4);
        } else {
            self.keypad.key_released(0x4);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::W) {
            self.keypad.key_pressed(0x5);
        } else {
            self.keypad.key_released(0x5);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::E) {
            self.keypad.key_pressed(0x6);
        } else {
            self.keypad.key_released(0x6);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::R) {
            self.keypad.key_pressed(0xD);
        } else {
            self.keypad.key_released(0xD);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::A) {
            self.keypad.key_pressed(0x7);
        } else {
            self.keypad.key_released(0x7);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::S) {
            self.keypad.key_pressed(0x8);
        } else {
            self.keypad.key_released(0x8);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::D) {
            self.keypad.key_pressed(0x9);
        } else {
            self.keypad.key_released(0x9);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::F) {
            self.keypad.key_pressed(0xE);
        } else {
            self.keypad.key_released(0xE);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::Z) {
            self.keypad.key_pressed(0xA);
        } else {
            self.keypad.key_released(0xA);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::X) {
            self.keypad.key_pressed(0x0);
        } else {
            self.keypad.key_released(0x0);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::C) {
            self.keypad.key_pressed(0xB);
        } else {
            self.keypad.key_released(0xB);
        }

        if input::keyboard::is_key_pressed(context, KeyCode::V) {
            self.keypad.key_pressed(0xF);
        } else {
            self.keypad.key_released(0xF);
        }

        // emulate cpu
        for _i in 0..TICKS_X_FRAME {
            if !self.chip.tick(&self.random, &mut self.screen, &self.keypad) {
                quit(context);
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        //println!("delta = {}ns", timer::delta(ctx).subsec_nanos());

        // emulate timers
        self.chip.tick_timers();

        // draw screen
        graphics::clear(context, graphics::BLACK);
        self.screen.draw(context)?;
        graphics::present(context)
    }
}

fn main() -> GameResult {
    let options = cli::get_options();

    if options.scale > 30 {
        panic!("scale parameter can not be greater than 30.");
    }

    let window = WindowMode {
        width: (64 * options.scale) as f32,
        height: (32 * options.scale) as f32,
        ..WindowMode::default()
    };

    let builder = ContextBuilder::new("chip8run", "chip8vm");
    let (context, event_loop) = &mut builder.window_mode(window).build()?;

    let path = Path::new(options.program_path.as_str());
    set_window_title(context, path.file_stem().unwrap().to_str().unwrap());

    let chip8 = &mut Chip8Run::new(context, options.scale)?;
    chip8.load_program(options.program_path);

    event::run(context, event_loop, chip8)
}
