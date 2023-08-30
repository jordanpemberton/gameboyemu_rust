#![allow(non_snake_case)]

mod cartridge;
mod cli;
mod console;

use crate::cli::cli::run;

fn main() {
    run();
}
