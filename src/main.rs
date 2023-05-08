#![allow(non_snake_case)]

mod cartridge;
mod cli;
mod console;

use std::env;
use crate::cli::cli::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}
