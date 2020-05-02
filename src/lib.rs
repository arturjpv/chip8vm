#![no_std]
//! The chip8vm module contains the chip::Chip main module of the Chip8 VM and all the
//! interface traits that need to be implemented, so the execution of the programs can take
//! place.
//!
//! Trait interfaces:
//! * Random - Numerical random range interface to provide a random range implementation to
//! the Chip8 VM.
//! * Screen - Screen interface to provide screen capabilities to the Chip 8 VM.
//! * Keypad - Kaypad interface to provide input events to the Chip8 VM.

pub mod chip;

mod font;
mod specs;

use specs::*;

/// Constant defining Chip8 VM program size.
pub const PROGRAM_SIZE: usize = PROG_END - PROG_START;


/// Random trait used by Chip 8 VM to get random numbers.
pub trait Random {
    /// This function returns a random number between 0 and 255.
    fn range(&self) -> u8;
}


/// Constant defining Chip8 VM screen width.
pub const SCREEN_WIDTH: usize = 64;
/// Constant defining Chip8 VM screen height.
pub const SCREEN_HEIGHT: usize = 32;

/// Screen trait used to control de application screen from the Chip8 VM.
pub trait Screen {
    /// This function requests the application to clear the screen display.
    fn clear(&mut self);

    /// This function request the application to draw a pixel in the (x, y) coordinates
    /// of the display. The draw operation must be done simulating a Xor operation and
    /// the return of the function must acknowledge if the status of the previous pixel
    /// is altered. This is understood by the Chip8 VM as a collision:
    /// * If the pixel is already on, turn it off and return true.
    /// * If the pixel is off, turn it on and return false.
    ///
    /// # Parameters
    /// * x - Coordinate x of the pixel to draw.
    /// * y - Coordinate y of the pixel to draw.
    ///
    /// # Return
    /// * true - If a collision is detected.
    /// * false - If there is no collision.
    fn draw(&mut self, x: u8, y: u8) -> bool;
}


/// Constant defining Chip8 VM keypad number of keys.
pub const KEYPAD_NUM_KEYS: usize = 16;

/// Keypad trait used to control de application keypad from the Chip8 VM.
pub trait Keypad {
    /// This function asks the application if a specific key is pressed.
    ///
    /// # Parameters
    /// * keycode - The keycode to check. From 0x0 to 0xF.
    ///
    /// # Return
    /// * true - If the specific key is pressed.
    /// * false - If the specific key is not pressed.
    fn is_pressed(&self, keycode: u8) -> bool;

    /// This function requests for a key press.
    ///
    /// In the original implementation of Chip8 this function was blocking. In the current
    /// implementation of the Chip8 VM, this function is non-blocking. If a key is not
    /// pressed, the Chip8 VM itself will avoid advancing the instruction pointer. This will
    /// make Chip8 to continue asking for a key until is pressed.
    ///
    /// # Return
    /// * Optional u8 - Optional return of a keycode. From 0x0 to 0xF.
    fn pressed_key(&self) -> Option<u8>;
}
