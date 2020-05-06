use crate::font::*;
use crate::specs::*;
use crate::{Random, Screen, Keypad, SCREEN_WIDTH, SCREEN_HEIGHT, PROGRAM_SIZE};

/// Chip structure that contains the needed state for the Chip8 VM to work.
///
/// Sizes of the fields are defined in the specs.rs module.
pub struct Chip {
    /// Set of registers for the Chip8 VM.
    registers: [u8; NUM_RESGISTERS],

    /// Address register.
    i: u16,

    /// Available memory for the Chip8 VM.
    memory: [u8; MEM_SIZE],

    /// Instruction pointer.
    ip: usize,

    /// Available memory for the stack of the Chip8 VM.
    stack: [u16; STACK_SIZE],

    /// Stack pointer.
    sp: usize,

    /// Available timers fot the Chip8 VM.
    timers: [u8; NUM_TIMERS],
}

/// Default implementation for Chip structure.
///
/// Initializes a Chip structure with default values and sizes defined in the specs.rs module.
impl Default for Chip {

    /// default function for fields initialization.
    fn default() -> Self {
        Chip {
            registers: [0; NUM_RESGISTERS],
            i: 0,
            memory: [0; MEM_SIZE],
            ip: PROG_START,
            stack: [0; STACK_SIZE],
            sp: 0,
            timers: [0; NUM_TIMERS],
        }
    }
}

/// The Chip implementation contains the required functions to decode the Chip8 opcodes and
/// perform it's associated actions.
impl Chip {
    /// Loads the provided program array in the chip memory, starting at PROG_START offset.
    ///
    /// It also initialize the reserved memory for the font, with the font set defined in the Font
    /// struct.
    ///
    /// A program should be loaded before execution of the VM starts.
    ///
    /// # Arguments
    ///
    /// * `program` - array with the program data.
    ///
    /// # Example
    ///
    /// ```
    /// // Declare use of chip8vm::chip module.
    /// use chip8vm::chip::*;
    /// use chip8vm::PROGRAM_SIZE;
    ///
    /// // Create a Chip object with default values.
    /// let mut chip = Chip::default();
    /// // Create an empty program array.
    /// let mut program: [u8; PROGRAM_SIZE] = [0; PROGRAM_SIZE];
    ///
    /// //... Fill program array with Chip8 opcodes.
    ///
    /// // Load program into Chip8 VM.
    /// chip.load_program(program);
    /// ```
    pub fn load_program(&mut self, program: [u8; PROGRAM_SIZE]) {
        // Load font
        self.memory[..(CHARACTERS * CHARACTER_SIZE)].copy_from_slice(&Font::default().set);

        // Load program
        self.memory[PROG_START..(PROG_START + program.len())].copy_from_slice(&program);
    }

    /// Decreases the value of the Chip8 VM timers by 1.
    ///
    /// The two internal timers:
    /// * Delay timer - Used for synchronization purposes in programs.
    /// * Sound timer - Used for producing sound. Whenever the value is greater that 0 sound is
    /// produced.
    ///
    /// This method must be called at a 60Hz frequency.
    pub fn tick_timers(&mut self) {
        if self.timers[DELAY_TIMER] > 0 {
            self.timers[DELAY_TIMER] -= 1;
        }

        if self.timers[SOUND_TIMER] > 0 {
            self.timers[SOUND_TIMER] -= 1;
        }
    }

    /// Decodes and executes the current instruction pointed by the instruction pointer.
    /// Dependencies like screen, keypad and random are injected as parameters.
    ///
    /// # Parameters
    /// * rand -
    /// * screen -
    /// * keypad -
    ///
    /// This method should be called at a frequency around 600Hz.
    pub fn tick(&mut self, random: &impl Random, screen: &mut impl Screen, keypad: &impl Keypad) -> bool {
        let op_high = self.memory[self.ip] as u16;
        let op_low = self.memory[self.ip + 1] as u16;
        let opcode = op_high << 8 | op_low;

        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let option = (opcode & 0x000F) as u8;
        let address = opcode & 0x0FFF;

        self.ip += 2;

        let mut execute = true;
        match opcode {
            0x0000 => execute = false,
            0x00E0 => self.cls(screen),
            0x00EE => self.ret(),
            0x1000..=0x1FFF => self.jmp(address),
            0x2000..=0x2FFF => self.call(address),
            0x3000..=0x3FFF => self.se_vx_byte(x, op_low as u8),
            0x4000..=0x4FFF => self.sne_vx_byte(x, op_low as u8),
            0x5000..=0x5FFF => self.sne_vx_vy(x, y),
            0x6000..=0x6FFF => self.ld_vx_byte(x, op_low as u8),
            0x7000..=0x7FFF => self.add_vx_byte(x, op_low as u8),
            0x8000..=0x8FFF => match option {
                0x0 => self.ld_vx_vy(x, y),
                0x1 => self.or_vx_vy(x, y),
                0x2 => self.and_vx_vy(x, y),
                0x3 => self.xor_vx_vy(x, y),
                0x4 => self.add_vx_vy(x, y),
                0x5 => self.sub_vx_vy(x, y),
                0x6 => self.shr_vx(x),
                0x7 => self.subn_vx_vy(x, y),
                0xE => self.shl_vx(x),
                _ => execute = false,
            },
            0x9000..=0x9FFF => self.sne_vx_vy(x, y),
            0xA000..=0xAFFF => self.ld_i_addr(address),
            0xB000..=0xBFFF => self.jmp_v0_addr(address),
            0xC000..=0xCFFF => self.rnd_vx_byte(random, x, op_low as u8),
            0xD000..=0xDFFF => self.draw_vx_vy_nibble(screen, x, y, option),
            0xE000..=0xEFFF => match op_low {
                0x9E => self.skp_vx(keypad, x),
                0xA1 => self.sknp_vx(keypad, x),
                _ => execute = false,
            },
            0xF000..=0xFFFF => match op_low {
                0x07 => self.ld_vx_dt(x),
                0x0A => self.ld_vx_k(keypad, x),
                0x15 => self.ld_dt_vx(x),
                0x18 => self.ld_st_vx(x),
                0x1E => self.add_i_vx(x),
                0x29 => self.ld_f_vx(x),
                0x33 => self.ld_b_vx(x),
                0x55 => self.ld_vi_vx(x),
                0x65 => self.ld_vx_vi(x),
                _ => execute = false,
            },
            _ => execute = false,
        }

        execute
    }

    fn cls(&mut self, screen: &mut impl Screen) {
        screen.clear();
    }

    /// Returns from a subroutine call.
    ///
    /// Decreases the stack pointer and retrieves the address stored at stack pointer. The
    /// instruction pointer is set to the stack pointer retrieved address.
    fn ret(&mut self) {
        self.sp -= 1;
        self.ip = self.stack[self.sp] as usize;
    }

    /// Jumps to the specified address.
    ///
    /// Sets the instruction pointer to the specific address.
    ///
    /// # Parameters
    /// * address - Memory address to set the instruction pointer to.
    fn jmp(&mut self, address: u16) {
        self.ip = address as usize;
    }

    /// Call the subroutine stored at the specified address.
    ///
    /// Stores the current instruction pointer in the stack, increases the stack pointer and
    /// sets the instruction pointer to the specific address.
    ///
    /// # Parameters
    /// * address - Memory address of the subroutine.
    fn call(&mut self, address: u16) {
        self.stack[self.sp] = self.ip as u16;
        self.sp += 1;
        self.ip = address as usize;
    }

    /// Skips the next instruction if the value stored in register Vx is equal to the encoded value
    /// in the opcode.
    ///
    /// # Parameters
    /// * x - Register number.
    /// * value - Encoded value to compare.
    fn se_vx_byte(&mut self, x: u8, value: u8) {
        if self.registers[x as usize] == value {
            self.ip += 2;
        }
    }

    /// Skips the next instruction if the value stored in register Vx is not equal to the opcode
    /// encoded value.
    ///
    /// # Parameters
    /// * x - Register number.
    /// * value - Value to compare.
    fn sne_vx_byte(&mut self, x: u8, value: u8) {
        if self.registers[x as usize] != value {
            self.ip += 2;
        }
    }

    /// Stores the opcode encoded value into register Vx.
    ///
    /// # Parameters
    /// * x - Register number to store the value.
    /// * value - Value to store.
    fn ld_vx_byte(&mut self, x: u8, value: u8) {
        self.registers[x as usize] = value;
    }

    /// Adds the opcode encoded value to the register Vx.
    ///
    /// # Parameters
    /// * x - Register number to add the value.
    /// * value - Value to add.
    fn add_vx_byte(&mut self, x: u8, value: u8) {
        let vx = self.registers[x as usize];
        self.registers[x as usize] = vx.overflowing_add(value).0;
    }

    /// Stores the content of register Vy in register Vx.
    ///
    /// # Parameters
    /// * x - Register number to store the value.
    /// * y - Register number that contains the value to store.
    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    /// Performs *Or* operation between Vx and Vy, storing the result in Vx.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    /// * y - Register number foy Vy.
    fn or_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    /// Performs *And* operation between Vx and Vy, storing the result in Vx.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    /// * y - Register number foy Vy.
    fn and_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] &= self.registers[y as usize];
    }

    /// Performs *Xor* operation between Vx and Vy, storing the result in Vx.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    /// * y - Register number foy Vy.
    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    /// Performs *Add* operation between Vx and Vy, storing the result in Vx.
    /// Register VF is set to 1 if the addition causes a carry (overflow).
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    /// * y - Register number foy Vy.
    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];
        let result = vx.overflowing_add(vy);
        self.registers[x as usize] = result.0;
        self.registers[FLAG] = if result.1 { 1 } else { 0 };
    }

    /// Performs *Sub* operation to Vx, subtracting Vy and storing the result in Vx.
    /// Register VF is set to 1 if the subtraction does NOT cause borrow (overflow_sub).
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    /// * y - Register number foy Vy.
    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];
        let result = vx.overflowing_sub(vy);
        self.registers[x as usize] = result.0;
        self.registers[FLAG] = if result.1 { 0 } else { 1 };
    }

    /// Performs right swift operation to Vx.
    /// Register VF is set to 1 if the least significant bit is 1.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn shr_vx(&mut self, x: u8) {
        self.registers[FLAG] = self.registers[x as usize] & 0x01;
        self.registers[x as usize] >>= 1;
    }

    /// Performs *Sub* operation to Vy, subtracting Vx and storing the result in Vx.
    /// Register VF is set to 1 if the subtraction does NOT cause borrow (overflow_sub).
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    /// * y - Register number foy Vy.
    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];
        let result = vy.overflowing_sub(vx);
        self.registers[x as usize] = result.0;
        self.registers[FLAG] = if result.1 { 0 } else { 1 };
    }

    /// Performs left swift operation to Vx.
    /// Register VF is set to 1 if the most significant bit is 1.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn shl_vx(&mut self, x: u8) {
        self.registers[FLAG] = self.registers[x as usize] >> 7;
        self.registers[x as usize] <<= 1;
    }

    /// Skips the next instruction if Vx is different from Vy.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    /// * y - Register number for Vy.
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] != self.registers[y as usize] {
            self.ip += 2;
        }
    }

    /// Loads the opcode encoded address to the register I
    ///
    /// # Parameters
    /// * address - Address that will be loaded to register I
    fn ld_i_addr(&mut self, address: u16) {
        self.i = address
    }

    /// Jumps to a calculated location, setting the instruction pointer to the
    /// result of the operation *address* + V0.
    ///
    /// # Parameters
    /// * address - Address to add to register VO.
    fn jmp_v0_addr(&mut self, address: u16) {
        self.ip = (address + self.registers[0 as usize] as u16) as usize;
    }

    fn rnd_vx_byte(&mut self, random: &impl Random, x: u8, mask: u8) {
        self.registers[x as usize] = random.range() & mask;
    }

    fn draw_vx_vy_nibble(&mut self, screen: &mut impl Screen, x: u8, y: u8, lines: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];
        let mut collision = false;

        for line in 0..lines {
            let data = self.memory[(self.i + line as u16) as usize];
            for displace in 0..8 {
                let bit = (data >> displace) & 0x1;
                if bit == 1 {
                    let new_x = vx.overflowing_add(7 - displace).0 % SCREEN_WIDTH as u8;
                    let new_y = vy.overflowing_add(line).0 % SCREEN_HEIGHT as u8;

                    collision |= screen.draw(new_x, new_y);
                }
            }
        }

        self.registers[FLAG] = if collision { 1 } else { 0 };
    }

    fn skp_vx(&mut self, keypad: &impl Keypad, x: u8) {
        let vx = self.registers[x as usize];
        if keypad.is_pressed(vx) {
            self.ip += 2;
        }
    }

    fn sknp_vx(&mut self, keypad: &impl Keypad, x: u8) {
        let vx = self.registers[x as usize];
        if !keypad.is_pressed(vx) {
            self.ip += 2;
        }
    }

    /// Set Vx register to delay timer value.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn ld_vx_dt(&mut self, x: u8) {
        self.registers[x as usize] = self.timers[DELAY_TIMER];
    }

    fn ld_vx_k(&mut self, keypad: &impl Keypad, x: u8) {
        let key = keypad.pressed_key();
        match key {
            None => {
                self.ip -= 2;
            }
            Some(key) => self.registers[x as usize] = key,
        }
    }

    /// Set delay timer to register Vx.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn ld_dt_vx(&mut self, x: u8) {
        self.timers[DELAY_TIMER] = self.registers[x as usize];
    }

    /// Set sound timer to register Vx.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn ld_st_vx(&mut self, x: u8) {
        self.timers[SOUND_TIMER] = self.registers[x as usize];
    }

    /// Add Vx register value to I. Store the result in I.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn add_i_vx(&mut self, x: u8) {
        let vx = self.registers[x as usize] as u16;
        self.i += vx;
    }

    /// Set I to the location of the Vx font sprite.
    /// Each character has CHARACTER_SIZE length.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn ld_f_vx(&mut self, x: u8) {
        self.i = (self.registers[x as usize] * CHARACTER_SIZE as u8) as u16;
    }

    /// Stores BCD representation of the value contained in register Vx.
    /// Storing the result in I, I + 1 and I + 2.
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn ld_b_vx(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        self.memory[self.i as usize] = vx / 100;
        self.memory[(self.i + 1) as usize] = (vx % 100) / 10;
        self.memory[(self.i + 2) as usize] = vx % 10;
    }

    /// Stores V0 to Vx in memory, starting at address I.
    /// I is set to I + x + 1
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn ld_vi_vx(&mut self, x: u8) {
        for register in 0..=x {
            self.memory[(self.i + register as u16) as usize] = self.registers[register as usize];
        }
        self.i += (x + 1) as u16;
    }

    /// Fills V0 to Vx with values from memory starting at address I.
    /// I is set to I + x + 1
    ///
    /// # Parameters
    /// * x - Register number for Vx.
    fn ld_vx_vi(&mut self, x: u8) {
        for register in 0..=x {
            self.registers[register as usize] = self.memory[(self.i + register as u16) as usize];
        }
        self.i += (x + 1) as u16;
    }
}


//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use crate::PROGRAM_SIZE;

    struct TestRandom {}

    impl Random for TestRandom {
        fn range(&self) -> u8 {
            1
        }
    }

    struct TestScreen {}

    impl Screen for TestScreen {
        fn clear(&mut self) {}
        fn draw(&mut self, _x: u8, _y: u8) -> bool { true }
    }

    struct TestKeypad {}

    impl Keypad for TestKeypad {
        fn is_pressed(&self, _keycode: u8) -> bool { true }
        fn pressed_key(&self) -> Option<u8> { None }
    }

    fn prepare_vm(test_program: [u8; 10])-> (Chip, TestRandom, TestScreen, TestKeypad){
        let mut chip = Chip::default();
        let random = TestRandom{};
        let screen = TestScreen{};
        let keypad = TestKeypad{};

        let mut program = [0; PROGRAM_SIZE];
        program[0..test_program.len()].copy_from_slice(&test_program);

        chip.load_program(program);
        (chip, random, screen, keypad)
    }

    #[test]
    fn load_font() {
        let program_code: [u8; 10] = [0; 10];
        let (chip, _random, _screen, _keypad) = prepare_vm(program_code);

        let mut loaded = true;
        let font = Font::default();
        for i in 0..font.set.len() {
            if chip.memory[i] != font.set[i] {
                loaded = false;
                break;
            }
        }

        assert!(loaded);
    }

    #[test]
    fn load_program() {
        let program_code: [u8; 10] = [0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        let (chip, _random, _screen, _keypad) = prepare_vm(program_code);

        let mut loaded = true;
        for i in 0..program_code.len() {
            if chip.memory[PROG_START + i] != program_code[i] {
                loaded = false;
                break;
            }
        }

        assert!(loaded);
    }

    #[test]
    fn tick_timers_delay_not_zero() {
        let mut chip = Chip::default();
        chip.timers[DELAY_TIMER] = 3;

        chip.tick_timers();

        assert_eq!(chip.timers[DELAY_TIMER], 2);
    }

    #[test]
    fn tick_timers_delay_zero() {
        let mut chip = Chip::default();
        chip.timers[DELAY_TIMER] = 0;

        chip.tick_timers();

        assert_eq!(chip.timers[DELAY_TIMER], 0);
    }

    #[test]
    fn tick_timers_sound_not_zero() {
        let mut chip = Chip::default();
        chip.timers[SOUND_TIMER] = 3;

        chip.tick_timers();

        assert_eq!(chip.timers[SOUND_TIMER], 2);
    }

    #[test]
    fn tick_timers_sound_zero() {
        let mut chip = Chip::default();
        chip.timers[SOUND_TIMER] = 0;

        chip.tick_timers();

        assert_eq!(chip.timers[SOUND_TIMER], 0);
    }

    #[test]
    fn tick_opcode_0x00() {
        let program_code: [u8; 10] = [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let (mut chip, random, mut screen, keypad) = prepare_vm(program_code);

        let result = chip.tick(&random, &mut screen, &keypad);

        assert_eq!(result, false);
        assert_eq!(chip.ip, PROG_START + 2);
    }

    #[test]
    fn opcode_ret() {
        let mut chip = Chip::default();

        chip.stack[0] = 0x210;
        chip.sp = 1;

        chip.ret();

        assert_eq!(chip.ip, 0x210);
        assert_eq!(chip.sp, 0);
    }

    #[test]
    fn opcode_jmp() {
        let mut chip = Chip::default();

        chip.jmp(0x205);

        assert_eq!(chip.ip, 0x205);
    }

    #[test]
    fn opcode_call() {
        let mut chip = Chip::default();

        chip.call(0x205);

        assert_eq!(chip.ip, 0x205);
        assert_eq!(chip.sp, 1);
        assert_eq!(chip.stack[0], PROG_START as u16);
    }

    #[test]
    fn opcode_se_vx_byte_without_skip() {
        let mut chip = Chip::default();

        chip.registers[1] = 10;
        chip.se_vx_byte(1, 20);

        assert_eq!(chip.ip, PROG_START);
    }

    #[test]
    fn opcode_se_vx_byte_with_skip() {
        let mut chip = Chip::default();

        chip.registers[1] = 10;
        chip.se_vx_byte(1, 10);

        assert_eq!(chip.ip, PROG_START + 2);
    }

    #[test]
    fn opcode_sne_vx_byte_without_skip() {
        let mut chip = Chip::default();

        chip.registers[1] = 10;
        chip.sne_vx_byte(1, 10);

        assert_eq!(chip.ip, PROG_START);
    }

    #[test]
    fn opcode_sne_vx_byte_with_skip() {
        let mut chip = Chip::default();

        chip.registers[1] = 10;
        chip.sne_vx_byte(1, 20);

        assert_eq!(chip.ip, PROG_START + 2);
    }

    #[test]
    fn opcode_ld_vx_byte() {
        let mut chip = Chip::default();

        chip.ld_vx_byte(1, 20);

        assert_eq!(chip.registers[1], 20);
    }

    #[test]
    fn opcode_add_vx_byte() {
        let mut chip = Chip::default();

        chip.registers[1] = 10;
        chip.add_vx_byte(1, 2);

        assert_eq!(chip.registers[1], 12);
    }

    #[test]
    fn opcode_ld_vx_vy() {
        let mut chip = Chip::default();

        chip.registers[2] = 10;
        chip.ld_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 10);
    }

    #[test]
    fn opcode_or_vx_vy() {
        let mut chip = Chip::default();

        chip.registers[1] = 9;
        chip.registers[2] = 17;
        chip.or_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 25);
    }

    #[test]
    fn opcode_and_vx_vy() {
        let mut chip = Chip::default();

        chip.registers[1] = 9;
        chip.registers[2] = 17;
        chip.and_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 1);
    }

    #[test]
    fn opcode_xor_vx_vy() {
        let mut chip = Chip::default();

        chip.registers[1] = 9;
        chip.registers[2] = 17;
        chip.xor_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 24);
    }

    #[test]
    fn opcode_add_vx_vy_without_carry() {
        let mut chip = Chip::default();

        chip.registers[1] = 9;
        chip.registers[2] = 17;
        chip.add_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 26);
        assert_eq!(chip.registers[FLAG], 0);
    }

    #[test]
    fn opcode_add_vx_vy_with_carry() {
        let mut chip = Chip::default();

        chip.registers[1] = 240;
        chip.registers[2] = 40;
        chip.add_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 24);
        assert_eq!(chip.registers[FLAG], 1);
    }

    #[test]
    fn opcode_sub_vx_vy_without_borrow() {
        let mut chip = Chip::default();

        chip.registers[1] = 40;
        chip.registers[2] = 20;
        chip.sub_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 20);
        assert_eq!(chip.registers[FLAG], 1);
    }

    #[test]
    fn opcode_sub_vx_vy_with_borrow() {
        let mut chip = Chip::default();

        chip.registers[1] = 20;
        chip.registers[2] = 40;
        chip.sub_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 236);
        assert_eq!(chip.registers[FLAG], 0);
    }

    #[test]
    fn opcode_shr_vx_vy_without_bit() {
        let mut chip = Chip::default();

        chip.registers[1] = 8;
        chip.shr_vx(1);

        assert_eq!(chip.registers[1], 4);
        assert_eq!(chip.registers[FLAG], 0);
    }

    #[test]
    fn opcode_shr_vx_vy_with_bit() {
        let mut chip = Chip::default();

        chip.registers[1] = 9;
        chip.shr_vx(1);

        assert_eq!(chip.registers[1], 4);
        assert_eq!(chip.registers[FLAG], 1);
    }

    #[test]
    fn opcode_subn_vx_vy_without_borrow() {
        let mut chip = Chip::default();

        chip.registers[1] = 20;
        chip.registers[2] = 40;
        chip.subn_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 20);
        assert_eq!(chip.registers[FLAG], 1);
    }

    #[test]
    fn opcode_subn_vx_vy_with_borrow() {
        let mut chip = Chip::default();

        chip.registers[1] = 40;
        chip.registers[2] = 20;
        chip.subn_vx_vy(1, 2);

        assert_eq!(chip.registers[1], 236);
        assert_eq!(chip.registers[FLAG], 0);
    }

    #[test]
    fn opcode_shl_vx_vy_without_bit() {
        let mut chip = Chip::default();

        chip.registers[1] = 8;
        chip.shl_vx(1);

        assert_eq!(chip.registers[1], 16);
        assert_eq!(chip.registers[FLAG], 0);
    }

    #[test]
    fn opcode_shl_vx_vy_with_bit() {
        let mut chip = Chip::default();

        chip.registers[1] = 136;
        chip.shl_vx(1);

        assert_eq!(chip.registers[1], 16);
        assert_eq!(chip.registers[FLAG], 1);
    }

    #[test]
    fn opcode_sne_vx_vy_with_skip() {
        let mut chip = Chip::default();

        chip.registers[1] = 2;
        chip.registers[2] = 4;
        chip.sne_vx_vy(1, 2);

        assert_eq!(chip.ip, PROG_START + 2);
    }

    #[test]
    fn opcode_ld_i_addr() {
        let mut chip = Chip::default();

        chip.ld_i_addr(0x205);

        assert_eq!(chip.i, 0x205);
    }

    #[test]
    fn opcode_jmp_v0_addr() {
        let mut chip = Chip::default();

        chip.registers[0] = 2;
        chip.jmp_v0_addr(0x205);

        assert_eq!(chip.ip, 0x207);
    }

    #[test]
    fn opcode_ld_vx_dt() {
        let mut chip = Chip::default();

        chip.timers[DELAY_TIMER] = 6;
        chip.ld_vx_dt(2);

        assert_eq!(chip.registers[2], 6);
    }

    #[test]
    fn opcode_ld_dt_vx() {
        let mut chip = Chip::default();

        chip.registers[2] = 6;
        chip.ld_dt_vx(2);

        assert_eq!(chip.timers[DELAY_TIMER], 6);
    }

    #[test]
    fn opcode_ld_st_vx() {
        let mut chip = Chip::default();

        chip.registers[2] = 6;
        chip.ld_st_vx(2);

        assert_eq!(chip.timers[SOUND_TIMER], 6);
    }

    #[test]
    fn opcode_add_i_vx() {
        let mut chip = Chip::default();

        chip.registers[2] = 6;
        chip.ld_st_vx(2);

        assert_eq!(chip.timers[SOUND_TIMER], 6);
    }

    #[test]
    fn opcode_ld_f_vx() {
        let mut chip = Chip::default();

        chip.registers[2] = 6;
        chip.ld_f_vx(2);

        assert_eq!(chip.i, (chip.registers[2] * CHARACTER_SIZE as u8) as u16);
    }

    #[test]
    fn opcode_ld_b_vx() {
         let mut chip = Chip::default();

         chip.registers[2] = 128;
         chip.ld_b_vx(2);

         assert_eq!(chip.memory[chip.i as usize], 1);
         assert_eq!(chip.memory[(chip.i + 1) as usize], 2);
         assert_eq!(chip.memory[(chip.i + 2) as usize], 8);
    }

    #[test]
    fn opcode_ld_vi_vx() {
        let mut chip = Chip::default();

        chip.registers[0] = 32;
        chip.registers[6] = 64;
        chip.registers[FLAG] = 128;
        chip.i = 0x200;
        chip.ld_vi_vx(FLAG as u8);

        assert_eq!(chip.memory[0x200], 32);
        assert_eq!(chip.memory[0x201], 0);
        assert_eq!(chip.memory[0x202], 0);
        assert_eq!(chip.memory[0x203], 0);
        assert_eq!(chip.memory[0x204], 0);
        assert_eq!(chip.memory[0x205], 0);
        assert_eq!(chip.memory[0x206], 64);
        assert_eq!(chip.memory[0x207], 0);
        assert_eq!(chip.memory[0x208], 0);
        assert_eq!(chip.memory[0x209], 0);
        assert_eq!(chip.memory[0x20A], 0);
        assert_eq!(chip.memory[0x20B], 0);
        assert_eq!(chip.memory[0x20C], 0);
        assert_eq!(chip.memory[0x20D], 0);
        assert_eq!(chip.memory[0x20E], 0);
        assert_eq!(chip.memory[0x20F], 128);
        assert_eq!(chip.i, 0x210);
    }

    #[test]
    fn opcode_ld_vx_vi() {
        let mut chip = Chip::default();
        chip.memory[0x200] = 32;
        chip.memory[0x206] = 64;
        chip.memory[0x20F] = 128;
        chip.i = 0x200;
        chip.ld_vx_vi(FLAG as u8);

        assert_eq!(chip.registers[0x0], 32);
        assert_eq!(chip.registers[0x1], 0);
        assert_eq!(chip.registers[0x2], 0);
        assert_eq!(chip.registers[0x2], 0);
        assert_eq!(chip.registers[0x3], 0);
        assert_eq!(chip.registers[0x4], 0);
        assert_eq!(chip.registers[0x5], 0);
        assert_eq!(chip.registers[0x6], 64);
        assert_eq!(chip.registers[0x7], 0);
        assert_eq!(chip.registers[0x8], 0);
        assert_eq!(chip.registers[0x9], 0);
        assert_eq!(chip.registers[0xA], 0);
        assert_eq!(chip.registers[0xB], 0);
        assert_eq!(chip.registers[0xC], 0);
        assert_eq!(chip.registers[0xD], 0);
        assert_eq!(chip.registers[0xE], 0);
        assert_eq!(chip.registers[0xF], 128);
        assert_eq!(chip.i, 0x210);
    }
}
