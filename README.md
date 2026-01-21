# Game Boy Emulator Project

## Dependencies
- Rust, Cargo
-- To install (Ubuntu), see: https://rust-lang.org/tools/install/
- SDL2
-- To install (Ubuntu): `sudo apt-get install libsdl2-dev`

## To Build
- `cargo build [--release]`

## To Cross Compile
- TODO

## To Package 
- TODO

## To Run
`cargo run [--release] [<path/to/rom>] [skipboot] [debug] [printcpu]`

Args:
- `skipboot`: Skip the boot sequence
- `debug`: Run with debugger enabled.
- `printcpu`: Print CPU commands to output.

## To Run Tests
`cargo run [--release] [<path/to/testrom>] [debug] [printcpu]`

## Key Inputs
Program Inputs:
- `Escape`: Quit GamBoy

JoyPad Inputs:
- `Up Arrow`: Up
- `Down Arrow`: Down
- `Left Arrow`: Left
- `Right Arrow`: Right
- `A`: Select
- `S`: Start
- `X`: A Button
- `Z`: B Button

Debugger Inputs:
When running with the `debug` flag, you can use:
- `B`: Break /Resume -- Break and dump info to the output.
- `O`: Output -- Output the display as plain text.
- `P`: Peek -- Dump info to the output (without breaking).
- (FUTURE TODO) `S`: Step
