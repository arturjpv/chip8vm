// Registers
pub const NUM_RESGISTERS: usize = 16;
pub const FLAG: usize = 0xF;

// Memory mapping
// 0x000..0x1FF -> Reserved for interpreter (font)
// 0x200..0xE8F -> Program
// 0xE90..0xFFF -> Reserved for variables and display
pub const MEM_SIZE: usize = 4096;
pub const PROG_START: usize = 0x200;
pub const PROG_END: usize = 0xE8F;

// Stack
pub const STACK_SIZE: usize = 16;

// Timers
pub const NUM_TIMERS: usize = 2;
pub const DELAY_TIMER: usize = 0;
pub const SOUND_TIMER: usize = 1;
