# Game Boy Emulator Project

Forver WIP (very buggy, very slow)

![GamBoy screenshot](/docs/screenshots/gamboy-trans-kirby.png)

## Dependencies
- Rust, Cargo
    - To install (Ubuntu), see: https://rust-lang.org/tools/install/
- SDL2
    - To install (Ubuntu): `sudo apt-get install libsdl2-dev`

## To Build
`cargo build [--release]`

## To Cross Compile
- TODO

## To Package 
- TODO

## To Run
`cargo run [--release] [<path/to/rom>] [skipboot] [debug] [printcpu]`

### Args:
- The first arg is a filepath to a rom file.
    - (WIP) if 'norom', run without a cartridge.
- `skipboot`: Skip the boot sequence
- `debug`: Run with debugger enabled.
- `printcpu`: Print CPU commands to output.

### Game Roms
If a game rom filepath is not provided via the first command line argument, GamBoy will allow you to select a rom from the `/roms/` directory.

## To Run Tests
`cargo run [--release] [<path/to/testrom>] [debug] [printcpu]`

## Key Inputs
### Program Inputs:
- `Escape`: Quit GamBoy
- (FUTURE TODO): Swap color palette

### JoyPad Inputs:
- `Up Arrow`: Up
- `Down Arrow`: Down
- `Left Arrow`: Left
- `Right Arrow`: Right
- `A`: Select
- `S`: Start
- `X`: A Button
- `Z`: B Button

### Debugger Inputs:
When running with the `debug` flag, you can use:
- `B`: Break /Resume -- Break and dump info to the output.
- `O`: Output -- Output the display as plain text.
- `P`: Peek -- Dump info to the output (without breaking).
- (FUTURE TODO) `S`: Step

## More Resources
See [/resources/](./resources)
