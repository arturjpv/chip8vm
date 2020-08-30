# About chip8vm
chip8vm is a [rust](https://www.rust-lang.org/) learning project. This library provides a full implementation of the CHIP-8 instruction set for the [chip8rtic](https://github.com/arturjpv/chip8rtic) project.


# Running the example
The library includes a quick test application using the [ggez](https://ggez.rs/) library to test the emulator in the computer before integration with [chip8rtic](https://github.com/arturjpv/chip8rtic).

To run the emulator application execute it as a regular rust example:

```
cargo run --release --example chip8run -- ./games/BRIX
```

# Emulator keys
The emulator example uses the following keymappings for the input, simulating the CHIP-8 Keypad layout:

| Keyboard | CHIP-8 |
|:-------: |:------:|
|        1 | 1 KEY  |
|        2 | 2 KEY  |
|        3 | 3 KEY  |
|        4 | C KEY  |
|        Q | 4 KEY  |
|        W | 5 KEY  |
|        E | 6 KEY  |
|        R | D KEY  |
|        A | 7 KEY  |
|        S | 8 KEY  |
|        D | 9 KEY  |
|        F | E KEY  |
|        Z | A KEY  |
|        X | 0 KEY  |
|        C | B KEY  |
|        V | F KEY  |

# Known issues
The example application was written to test the emulator but it has some timing issues, usually being to fast for CHIP-8, especially for the input handling. So not all ROMs behave as expected. Some improvements are needed in the example application to better integrate with Chip-8 timings.

# ROM Credits
ROMs are provided in the `games` folder for testing purposes. Credits of ROMs go to individual creators:

* 15PUZZLE - Roger Ivie
* BLINKY - Hans Christian Egeberg
* BLITZ - David Winter
* BRIX - Andreas Gustafsson
* CONNECT4 - David Winter
* GUESS - David Winter
* HIDDEN - David Winter
* INVADERS - David Winter
* KALEID - Joseph Weisbecker
* MAZE - ?
* MERLIN - David Winter
* MISSILE - David Winter
* PONG - Paul Vervalin
* PONG2 - David Winter
* PUZZLE - ?
* SYZYGY - Roy Trevino
* TANK - ?
* TETRIS - Fran Dachille
* TICTAC - David Winter
* UFO - Lutz V
* VBRIX - Paul Robson
* VERS - JMN
* WIPEOFF - Joseph Weisbecker