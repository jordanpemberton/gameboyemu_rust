#![allow(dead_code)]
#![allow(unused_variables)]

use crate::console::cpu::Cpu;
use crate::console::cpu_registers::CpuRegIndex;
use crate::console::mmu::Mmu;
use crate::console::ppu::Ppu;
use crate::console::timer::Timer;

pub(crate) enum DebugAction {
    BREAK,
    PEEK,
    STEP,
}

pub(crate) struct Debugger {
    pub(crate) enabled: bool,
    pub(crate) active: bool,
}

impl Debugger {
    pub(crate) fn new() -> Debugger {
        Debugger {
            enabled: true,
            active: false,
        }
    }

    pub(crate) fn is_active(&self) -> bool {
        self.enabled && self.active
    }

    pub(crate) fn break_or_cont(&mut self, cpu: Option<&Cpu>, mmu: Option<&Mmu>, timer: Option<&Timer>, locals: Option<Vec<(&str, &str)>>) {
        if self.enabled {
            self.active = !self.active;
            if self.active {
                self.dump(cpu, mmu, timer, locals);
            }
        }
    }

    pub(crate) fn peek(&mut self, cpu: Option<&Cpu>, mmu: Option<&Mmu>, timer: Option<&Timer>, locals: Option<Vec<(&str, &str)>>) {
        if self.enabled {
            self.dump(cpu, mmu, timer, locals);
        }
    }

    pub(crate) fn print_screen_to_stdout(&mut self, ppu: &Ppu) {
        for x in 0..ppu.lcd.width + 2 {
            print!("_");
        }
        println!();
        for y in 0..ppu.lcd.height {
            print!("|");
            for x in 0..ppu.lcd.width {
                let color = ppu.lcd.data[y as usize][x as usize];
                match color % 4 {
                    3 => print!("@"),
                    2 => print!("+"),
                    1 => print!("."),
                    _ => print!(" "),
                }
            }
            print!("|");
            println!();
        }
        for x in 0..ppu.lcd.width + 2 {
            print!("_");
        }
        println!();
    }

    pub(crate) fn print_cpu_exec(cpu: &mut Cpu, mmu: &Mmu, start_pc: u16, opcode: u16, mnemonic: &str, args: &[u8]) {
        Debugger::print_cpu_exec_op1(cpu, mmu, start_pc, opcode, mnemonic, args);
        // Debugger::print_cpu_exec_op2(cpu, mmu, start_pc, opcode, mnemonic, args);
    }

    // Option 1, Example: "0xC375	0x00CA	JP Z,a16      	0xB9	0xC1"
    pub(crate) fn print_cpu_exec_op1(cpu: &mut Cpu, mmu: &Mmu, start_pc: u16, opcode: u16, mnemonic: &str, args: &[u8]) {
        if !cpu.visited.contains(&start_pc) { // 0x234E == DrMario PollJoypad
            print!("{:#06X}\t{:#06X}\t{:<14}", start_pc, opcode, mnemonic);
            if args.len() > 0 {
                print!("\t{:#04X}", args[0]);
            };
            if args.len() > 1 {
                print!("\t{:#04X}", args[1]);
            };
            println!();
            cpu.visited.insert(start_pc);
        }
    }

    // Option 2, Example: "A: 0C F: 40 B: 06 C: FF D: C8 E: 46 H: 8A L: 74 SP: DFF7 PC: 00:C762 (13 A9 22 22)"
    // (to check against blargg test logging)
    pub(crate) fn print_cpu_exec_op2(cpu: &mut Cpu, mmu: &Mmu, start_pc: u16, opcode: u16, mnemonic: &str, args: &[u8]) {
        println!(
            "A: {:02X} F: {:02X} B: {:02X} C: {:02X} \
                    D: {:02X} E: {:02X} H: {:02X} L: {:02X} \
                    SP: {:04X} PC: {:02X}:{:04X} ({:02X} {:02X} {:02X} {:02X})",
            cpu.registers.get_byte(CpuRegIndex::A),
            cpu.registers.get_byte(CpuRegIndex::F),
            cpu.registers.get_byte(CpuRegIndex::B),
            cpu.registers.get_byte(CpuRegIndex::C),
            cpu.registers.get_byte(CpuRegIndex::D),
            cpu.registers.get_byte(CpuRegIndex::E),
            cpu.registers.get_byte(CpuRegIndex::H),
            cpu.registers.get_byte(CpuRegIndex::L),
            cpu.registers.get_word(CpuRegIndex::SP),
            0, // ?
            start_pc,
            mmu.read_8(start_pc),
            mmu.read_8(start_pc + 1),
            mmu.read_8(start_pc + 2),
            mmu.read_8(start_pc + 3),
        );
    }

    fn dump(&mut self, cpu: Option<&Cpu>, mmu: Option<&Mmu>, timer: Option<&Timer>, locals: Option<Vec<(&str, &str)>>) {
        if let Some(_cpu) = cpu {
            self.dump_cpu_state(_cpu);
        }
        if let Some(_mmu) = mmu {
            self.dump_mmu_state(_mmu);
        }
        if let Some(_timer) = timer {
            self.dump_timer_state(_timer);
        }
        if let Some(_locals) = locals {
            self.dump_key_value_pairs(_locals);
        }
    }

    fn dump_key_value_pairs(&self, data: Vec<(&str, &str)>) {
        for (key, value) in data {
            println!("{}:\t{}", key, value);
        }
    }

    fn dump_cpu_state(&self, cpu: &Cpu) {
        self.dump_key_value_pairs(vec![("Cpu.is_halted", format!("\t{}", cpu.is_halted).as_str())]);
        self.dump_key_value_pairs(vec![("Cpu.registers", format!("\n{}", cpu.registers).as_str())]);
        self.dump_key_value_pairs(vec![("Cpu.interrupts", format!("\t{}", cpu.interrupts).as_str())]);
    }

    fn dump_timer_state(&self, timer: &Timer) {
        self.dump_key_value_pairs(vec![("Timer", format!("\n{}", timer).as_str())]);
    }

    fn dump_mmu_state(&self, mmu: &Mmu) {
        let cols: usize = 16;

        let mut vram_values_str = String::new();
        let vram: &[u8] = &mmu.read_buffer(0x8000, 0xA000);
        let rows: usize = 2; // for all: (vram.len() as u16 / cols as u16) as usize;

        let mut i = 0;
        for row in 0..rows {
            vram_values_str.push_str(format!("\n{:#06X} :", i + 0x8000).as_str());

            for col in 0..cols {
                if vram[i] > 0 {
                    vram_values_str.push_str(format!("\t{:#04X}", vram[i]).as_str());
                } else {
                    vram_values_str.push_str("\t____");
                }
                i += 1;
            }
            vram_values_str.push_str("\n");
        }

        self.dump_key_value_pairs(vec![
            (
                format!("VRAM[0x0000..{:#06X}]", (rows * cols)).as_str(),
                format!("\n{}\n", vram_values_str).as_str()
            )
        ]);
    }
}
