use argh::FromArgs;

#[derive(FromArgs)]
/// chip8run is a chip8 emulator.
pub struct Cli {
    /// path to the program file
    #[argh(positional)]
    pub program_path: String,

    /// screen resolution scale
    #[argh(option, default = "10")]
    pub scale: u16,
}

pub fn get_options() -> Cli {
    argh::from_env()
}
