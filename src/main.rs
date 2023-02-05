#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

use crate::cpu::{CPU, MemoryBus};
use crate::registers::Registers;

mod cpu;
mod registers;
mod instructions;

fn main() {
    let mut cpu = CPU {
        registers: Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
        },
        pc: 0,
        sp: 0,
        bus: MemoryBus {
            memory: [0; 0xFFFF]
        },
        is_halted: false,
    };

    cpu.step();
}
