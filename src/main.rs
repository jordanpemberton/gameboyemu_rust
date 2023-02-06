#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

mod alu;
mod cpu;
mod gpu;
mod instructions;
mod memory_bus;
mod registers;

use crate::cpu::{Cpu};

fn main() {
    let mut cpu = Cpu::new();
    cpu.run();
}
