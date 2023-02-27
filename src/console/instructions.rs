#![allow(unused_variables)]

use crate::console::alu;
use crate::console::cpu::{Cpu};
use crate::console::mmu::{Endianness, Mmu};
use crate::console::registers::{Flags, RegIndex};

pub(crate) struct Instruction {
    opcode: u16,
    mnemonic: &'static str,
    size: u16,
    pub(crate) cycles: u16,
    _fn: fn(&mut Instruction, cpu: &mut Cpu, mmu: &mut Mmu),
}

impl Instruction {
    pub(crate) fn get_instruction(opcode: u16) -> Instruction {
        match opcode {
            0x0000 => Instruction{ opcode, mnemonic: "NOP", size: 1, cycles: 4, _fn: Instruction::op_0000 },
            0x0002 => Instruction{ opcode, mnemonic: "LD (BC),A", size: 1, cycles: 8, _fn: Instruction::op_0002 },
            0x0005 => Instruction{opcode, mnemonic: "DEC B", size: 1, cycles: 4, _fn: Instruction::op_0005 },
            0x0006 => Instruction{ opcode, mnemonic: "LD B,d8", size: 2, cycles: 8, _fn: Instruction::op_0006 },
            0x000C => Instruction{ opcode, mnemonic: "INC C", size: 1, cycles: 4, _fn: Instruction::op_000c },
            0x000E => Instruction{ opcode, mnemonic: "LD C,d8", size: 2, cycles: 8, _fn: Instruction::op_000e },
            0x0011 => Instruction{ opcode, mnemonic: "LD DE,d16", size: 3, cycles: 12, _fn: Instruction::op_0011 },
            0x0013 => Instruction{ opcode, mnemonic: "INC DE", size: 1, cycles: 8, _fn: Instruction::op_0013 },
            0x0017 => Instruction{ opcode, mnemonic: "RLA", size: 1, cycles: 4, _fn: Instruction::op_0017 },
            0x001A => Instruction{ opcode, mnemonic: "LD A,(DE)", size: 1, cycles: 8, _fn: Instruction::op_001a },
            0x0020 => Instruction{ opcode, mnemonic: "JR NZ,r8", size: 2, cycles: 8, _fn: Instruction::op_0020 },
            0x0021 => Instruction{ opcode, mnemonic: "LD HL,d16", size: 3, cycles: 12, _fn: Instruction::op_0021 },
            0x0022 => Instruction{ opcode, mnemonic: "LD (HL+),A", size: 1, cycles: 8, _fn: Instruction::op_0022 },
            0x0023 => Instruction{ opcode, mnemonic: "INC HL", size: 1, cycles: 8, _fn: Instruction::op_0023 },
            0x0031 => Instruction{ opcode, mnemonic: "LD SP,d16", size: 3, cycles: 12, _fn: Instruction::op_0031 },
            0x0032 => Instruction{ opcode, mnemonic: "LD (HL-),A", size: 1, cycles: 8, _fn: Instruction::op_0032 },
            // TODO 0x0034 => Instruction{ opcode, mnemonic: "INC (HL)", size: 1, cycles: 12, _fn: Instruction::op_0034 },
            0x003E => Instruction{ opcode, mnemonic: "LD A,d8", size: 2, cycles: 8, _fn: Instruction::op_003e },
            0x004F => Instruction{ opcode, mnemonic: "LD C,A", size: 1, cycles: 4, _fn: Instruction::op_004f },
            0x0077 => Instruction{ opcode, mnemonic: "LD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0077 },
            0x007B => Instruction{ opcode, mnemonic: "LD A,E", size: 1, cycles: 4, _fn: Instruction::op_007b },
            0x00AF => Instruction{ opcode, mnemonic: "XOR A", size: 1, cycles: 4, _fn: Instruction::op_00af },
            0x00C1 => Instruction{ opcode, mnemonic: "POP BC", size: 1, cycles: 12, _fn: Instruction::op_00c1 },
            0x00C5 => Instruction{ opcode, mnemonic: "PUSH BC", size: 1, cycles: 16, _fn: Instruction::op_00c5 },
            0x00C9 => Instruction{ opcode, mnemonic: "RET", size: 1, cycles: 16, _fn: Instruction::op_00c9 },
            0x00CD => Instruction{ opcode, mnemonic: "CALL a16", size: 3, cycles: 24, _fn: Instruction::op_00cd },
            0x00E0 => Instruction{ opcode, mnemonic: "LDH (a8), A", size: 2, cycles: 12, _fn: Instruction::op_00e0 },
            0x00E2 => Instruction{ opcode, mnemonic: "LD ($FF00+C),A", size: 1, cycles: 8, _fn: Instruction::op_00e2 },
            0x00FE => Instruction{ opcode, mnemonic: "CP d8", size: 2, cycles: 8, _fn: Instruction::op_00fe },
            0xCB11 => Instruction{ opcode, mnemonic: "RL C", size: 2, cycles: 8, _fn: Instruction::op_cb11 },
            0xCB7C => Instruction{ opcode, mnemonic: "BIT 7,H", size: 2, cycles: 8, _fn: Instruction::op_cb7c },
            _ => Instruction{ opcode, mnemonic: "Unimplemented", size: 1, cycles: 4, _fn: Instruction::unimplemented },
        }
    }

    pub(crate) fn execute(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> u16 {
        // println!("${:#06X}\t{:#06X}:\t{}",
        //     cpu.registers.get_word(RegIndex::PC), self.opcode, self.mnemonic);
        (self._fn)(self, cpu, mmu);
        self.cycles
    }

    fn unimplemented(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        panic!("Opcode {:#06X} (\"{}\") is unimplemented.", self.opcode, self.mnemonic);
    }

    /// UN-PREFIXED

    /// NOP
    /// 1  4
    /// - - - -
    fn op_0000(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
    }

    /// LD (BC),A
    /// 1  8
    /// - - - -
    fn op_0002(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let target_address = cpu.registers.get_word(RegIndex::BC);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
    }

    /// DEC B
    /// 1  4
    /// Z 1 H -
    fn op_0005(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let a = cpu.registers.get_byte(RegIndex::B);
        cpu.registers.decrement(RegIndex::B, 1);
        let b = cpu.registers.get_byte(RegIndex::B);
        let mut flags = cpu.registers.get_flags();
        flags.zero = b == 0;
        flags.subtract = true;
        flags.half_carry =  (b & 0xF0) < (a & 0xF0);
        cpu.registers.set_f(flags);
    }

    /// LD B,d8
    /// 2  8
    /// - - - -
    fn op_0006(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let source_address = cpu.registers.get_word(RegIndex::PC);
        let d8 = mmu.read_byte(source_address);
        cpu.registers.increment(RegIndex::PC, 1);
        cpu.registers.set_byte(RegIndex::B, d8);
    }

    /// INC C
    /// 1  4
    /// Z 0 H -
    fn op_000c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::C).wrapping_add(1);
        cpu.registers.set_byte(RegIndex::C, value);
        let mut flags = cpu.registers.get_flags();
        flags.zero = true;
        flags.subtract = false;
        flags.half_carry = true;
        cpu.registers.set_f(flags);
    }

    /// LD C,d8
    /// 2  8
    /// - - - -
    fn op_000e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let source_address = cpu.registers.get_word(RegIndex::PC);
        let d8 = mmu.read_byte(source_address);
        cpu.registers.increment(RegIndex::PC, 1);
        cpu.registers.set_byte(RegIndex::C, d8);
    }

    /// LD DE,d16
    /// 3  12
    /// - - - -
    fn op_0011(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let source_address = cpu.registers.get_word(RegIndex::PC);
        let d16 = mmu.read_word(source_address, Endianness::BIG);
        cpu.registers.increment(RegIndex::PC, 2);
        cpu.registers.set_word(RegIndex::DE, d16);
    }

    /// INC DE
    /// 1  8
    /// - - - -
    fn op_0013(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        cpu.registers.increment(RegIndex::DE, 1);
    }

    /// RLA
    /// 1  4
    /// Z 0 0 C
    fn op_0017(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::A);
        let mut flags = cpu.registers.get_flags();
        flags = alu::rl(value, flags.carry);
        cpu.registers.set_f(flags);
    }

    /// LD A,(DE)
    /// 1  8
    /// - - - -
    fn op_001a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let source_address = cpu.registers.get_word(RegIndex::DE);
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(RegIndex::A, value);
    }

    /// JR NZ,r8
    /// 2  12/8
    /// - - - -
    fn op_0020(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let source_address = cpu.registers.get_word(RegIndex::PC);
        cpu.registers.increment(RegIndex::PC, 1);
        let value = mmu.read_byte(source_address);
        let signed_value = alu::signed(value);

        let should_jump = !(cpu.registers.get_flags().zero);
        if should_jump {
            if signed_value < 0 {
                cpu.registers.decrement(RegIndex::PC, -signed_value as u16);
            }
            else {
                cpu.registers.increment(RegIndex::PC, signed_value as u16);
            }
            self.cycles += 4;
        }
    }

    /// LD HL,d16
    /// 3  12
    /// - - - -
    fn op_0021(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let source_address = cpu.registers.get_word(RegIndex::PC);
        let d16 = mmu.read_word(source_address, Endianness::BIG);
        cpu.registers.increment(RegIndex::PC, 2);
        cpu.registers.set_word(RegIndex::HL, d16);
    }

    /// LD (HL+),A
    /// 1  8
    /// - - - -
    fn op_0022(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let target_address = cpu.registers.get_word(RegIndex::HL);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
        cpu.registers.increment(RegIndex::HL, 1);
    }

    /// INC HL
    /// 1  8
    /// - - - -
    fn op_0023(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        cpu.registers.increment(RegIndex::HL, 1);
    }

    /// LD SP,d16
    /// 3  12
    /// - - - -
    fn op_0031(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let source_address = cpu.registers.get_word(RegIndex::PC);
        let d16 = mmu.read_word(source_address, Endianness::BIG);
        cpu.registers.increment(RegIndex::PC, 2);
        cpu.registers.set_word(RegIndex::SP, d16);
    }

    /// LD (HL-),A
    /// 1  8
    /// - - - -
    fn op_0032(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let target_address = cpu.registers.get_word(RegIndex::HL);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
        cpu.registers.set_word(RegIndex::HL, target_address.wrapping_sub(1));
    }

    /// INC (HL)
    /// 1 12
    /// Z 0 H -
    fn op_0034(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {

    }

    /// LD A,d8
    /// 2  8
    /// - - - -
    fn op_003e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let source_address = cpu.registers.get_word(RegIndex::PC);
        let d8 = mmu.read_byte(source_address);
        cpu.registers.increment(RegIndex::PC, 1);
        cpu.registers.set_byte(RegIndex::A, d8);
    }

    /// LD C,A
    /// 1  4
    /// - - - -
    fn op_004f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::A);
        cpu.registers.set_byte(RegIndex::C, value);
    }

    /// LD (HL),A
    /// 1  8
    /// - - - -
    fn op_0077(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let target_address = cpu.registers.get_word(RegIndex::HL);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
    }

    /// LD A,E
    /// 1  4
    /// - - - -
    fn op_007b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::E);
        cpu.registers.set_byte(RegIndex::A, value);
    }

    /// XOR A
    /// 1  4
    /// Z 0 0 0
    fn op_00af(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let mut value = cpu.registers.get_byte(RegIndex::A);
        value ^= value;
        cpu.registers.set_byte(RegIndex::A, value);
        let z = cpu.registers.get_byte(RegIndex::A) == 0;
        let flags = Flags {
            zero: z,
            subtract: false,
            half_carry: false,
            carry: false,
        };
        cpu.registers.set_f(flags);
    }

    /// POP BC
    /// 1  12
    /// - - - -
    fn op_00c1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        // rr=(SP) SP=SP+2 ; rr may be BC,DE,HL,AF
        let source_address = cpu.registers.get_word(RegIndex::SP);
        let value = mmu.read_word(source_address, Endianness::BIG);
        cpu.registers.set_word(RegIndex::BC, value);
        cpu.registers.increment(RegIndex::SP, 2);
    }

    /// PUSH BC
    /// 1  16
    /// - - - -
    fn op_00c5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        // SP=SP-2
        cpu.registers.decrement(RegIndex::SP, 2);
        // (SP)=BC
        let target_address = cpu.registers.get_word(RegIndex::SP);
        let value = cpu.registers.get_word(RegIndex::BC);
        mmu.load_word(target_address, value, Endianness::BIG);
    }

    /// RET
    /// 1 16
    /// - - - -
    fn op_00c9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        // PC=(SP)
        let sp = cpu.registers.get_word(RegIndex::SP);
        let jump_to_address = mmu.read_word(sp, Endianness::BIG);
        cpu.registers.set_word(RegIndex::PC, jump_to_address);
        // SP=SP+2
        cpu.registers.increment(RegIndex::SP, 2);
    }

    /// CALL a16
    /// 3  24
    /// - - - -
    fn op_00cd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        // read a16 aka (PC)
        let mut pc = cpu.registers.get_word(RegIndex::PC);
        cpu.registers.increment(RegIndex::PC, 2);
        let a16 = mmu.read_word(pc, Endianness::BIG);
        // SP=SP-2
        cpu.registers.decrement(RegIndex::SP, 2);
        let target_address = cpu.registers.get_word(RegIndex::SP);
        // (SP)=PC
        pc = cpu.registers.get_word(RegIndex::PC);
        mmu.load_word(target_address, pc, Endianness::BIG);
        // PC=a16
        cpu.registers.set_word(RegIndex::PC, a16);
    }

    /// LDH (a8),A
    /// 2  12
    /// - - - -
    fn op_00e0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let source_address = cpu.registers.get_word(RegIndex::PC);
        let a8 = mmu.read_byte(source_address);
        cpu.registers.increment(RegIndex::PC, 1);
        let target_address = 0xFF00 + (a8 as u16);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
    }

    /// LD ($FF00+C),A
    /// 1  8
    /// - - - -
    fn op_00e2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let target_address = 0xFF00 + (cpu.registers.get_byte(RegIndex::C) as u16);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
    }

    /// CP d8
    /// 2 8
    /// Z 1 H C
    fn op_00fe(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let a = cpu.registers.get_byte(RegIndex::A);
        let pc = cpu.registers.get_word(RegIndex::PC);
        let b = mmu.read_byte(pc);
        cpu.registers.increment(RegIndex::PC, 1);
        let (result, flags) = alu::subtract_byte(a, b);
        cpu.registers.set_f(flags);
    }

    /// ============ CB PREFIXED ============

    /// RL C
    /// 2  8
    /// Z 0 0 C
    fn op_cb11(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::C);
        let mut flags = cpu.registers.get_flags();
        flags = alu::rl(value, flags.carry);
        cpu.registers.set_f(flags);
    }

    /// BIT 7,H
    /// 2  8
    /// Z 0 1 -
    fn op_cb7c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::H);
        let mut flags = cpu.registers.get_flags();
        flags.zero = value & (1 << 7) == 0;
        flags.subtract = false;
        flags.half_carry = true;
        cpu.registers.set_f(flags);
    }
}
