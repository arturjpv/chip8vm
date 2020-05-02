use ggez::{graphics, Context, GameResult};

use chip8vm;

#[derive(PartialEq, Copy, Clone)]
pub enum PixelState {
    ON,
    OFF,
}

pub struct Screen {
    display: Vec<graphics::Mesh>,
    pixels: [PixelState; chip8vm::SCREEN_WIDTH * chip8vm::SCREEN_HEIGHT],
}

impl Screen {
    pub fn new(context: &mut Context, scale: u16) -> GameResult<Screen> {
        let mut display = Vec::new();

        for y in 0..32 {
            for x in 0..64 {
                let rect = ggez::graphics::Rect::new(
                    (x * scale) as f32,
                    (y * scale) as f32,
                    scale as f32,
                    scale as f32,
                );
                let pixel = graphics::Mesh::new_rectangle(
                    context,
                    graphics::DrawMode::fill(),
                    rect,
                    graphics::WHITE,
                )?;
                display.push(pixel)
            }
        }

        Ok(Screen {
            display,
            pixels: [PixelState::OFF; chip8vm::SCREEN_WIDTH * chip8vm::SCREEN_HEIGHT],
        })
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        for (i, pixel) in self.display.iter().enumerate() {
            if self.is_on(i) {
                graphics::draw(context, pixel, graphics::DrawParam::default())?;
            }
        }

        Ok(())
    }

    fn is_on(&self, i: usize) -> bool {
        if self.pixels[i] == PixelState::ON {
            true
        } else {
            false
        }
    }
}

impl chip8vm::Screen for Screen {
    fn clear(&mut self) {
        self.pixels.iter_mut().map(|x| *x = PixelState::OFF).count();
    }

    fn draw(&mut self, x: u8, y: u8) -> bool {
        let position = (y as usize * chip8vm::SCREEN_WIDTH) + x as usize;
        if self.pixels[position] == PixelState::ON {
            self.pixels[position] = PixelState::OFF;
            true
        } else {
            self.pixels[position] = PixelState::ON;
            false
        }
    }
}
