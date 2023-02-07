#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

mod cpu;
mod gpu;
mod instructions;
mod mmu;
mod registers;

use crate::cpu::{Cpu};
use crate::mmu::Mmu;

pub(crate) fn run(cpu: &mut Cpu, mmu: &mut Mmu) {
    loop {
        cpu.step(mmu);
    }
}

fn main() {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();

    run(&mut cpu, &mut mmu);
}
