#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

use crate::cpu::{CPU};

mod cpu;
mod registers;
mod instructions;

fn main() {
    let mut cpu = CPU::default();

    cpu.run();
}
