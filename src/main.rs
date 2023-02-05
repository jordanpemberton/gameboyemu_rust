#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

mod cpu;

fn main() {
    let mut cpu = cpu::CPU {
        registers: cpu::Registers {
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
        sc: 0,
        bus: cpu::MemoryBus {
            memory: [0; 0xFFFF]
        },
    };

    cpu.step();
}
