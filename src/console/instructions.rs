#![allow(unused_variables)]

use crate::console::alu;
use crate::console::cpu::{Cpu};
use crate::console::input::CallbackAction::DEBUG;
use crate::console::mmu::{Endianness, Mmu};
use crate::console::registers::{Flags, RegIndex};

pub(crate) struct Instruction {
    pub(crate) opcode: u16,
    pub(crate) mnemonic: &'static str,
    pub(crate) size: u16,
    pub(crate) cycles: u16,
    _fn: fn(&mut Instruction, cpu: &mut Cpu, mmu: &mut Mmu) -> i16,
}

impl Instruction {
    pub(crate) fn get_instruction(opcode: u16) -> Instruction {
        match opcode {
            0x0000 => Instruction { opcode, mnemonic: "NOP", size: 1, cycles: 4, _fn: Instruction::op_0000 },
            0x0002 => Instruction { opcode, mnemonic: "LD (BC),A", size: 1, cycles: 8, _fn: Instruction::op_0002 },
            0x0004 => Instruction { opcode, mnemonic: "INC B", size: 1, cycles: 4, _fn: Instruction::op_0004 },
            0x0005 => Instruction { opcode, mnemonic: "DEC B", size: 1, cycles: 4, _fn: Instruction::op_0005 },
            0x0006 => Instruction { opcode, mnemonic: "LD B,d8", size: 2, cycles: 8, _fn: Instruction::op_0006 },
            0x000C => Instruction { opcode, mnemonic: "INC C", size: 1, cycles: 4, _fn: Instruction::op_000c },
            0x000D => Instruction { opcode, mnemonic: "DEC C", size: 1, cycles: 4, _fn: Instruction::op_000d },
            0x000E => Instruction { opcode, mnemonic: "LD C,d8", size: 2, cycles: 8, _fn: Instruction::op_000e },
            0x0011 => Instruction { opcode, mnemonic: "LD DE,d16", size: 3, cycles: 12, _fn: Instruction::op_0011 },
            0x0013 => Instruction { opcode, mnemonic: "INC DE", size: 1, cycles: 8, _fn: Instruction::op_0013 },
            0x0015 => Instruction { opcode, mnemonic: "DEC D", size: 1, cycles: 4, _fn: Instruction::op_0015 },
            0x0016 => Instruction { opcode, mnemonic: "LD D,d8", size: 2, cycles: 8, _fn: Instruction::op_0016 },
            0x0017 => Instruction { opcode, mnemonic: "RLA", size: 1, cycles: 4, _fn: Instruction::op_0017 },
            0x0018 => Instruction { opcode, mnemonic: "JR r8", size: 2, cycles: 12, _fn: Instruction::op_0018 },
            0x001A => Instruction { opcode, mnemonic: "LD A,(DE)", size: 1, cycles: 8, _fn: Instruction::op_001a },
            0x001D => Instruction { opcode, mnemonic: "DEC E", size: 1, cycles: 4, _fn: Instruction::op_001d },
            0x001E => Instruction { opcode, mnemonic: "LD E,d8", size: 2, cycles: 8, _fn: Instruction::op_001e },
            0x0020 => Instruction { opcode, mnemonic: "JR NZ,r8", size: 2, cycles: 8, _fn: Instruction::op_0020 },
            0x0021 => Instruction { opcode, mnemonic: "LD HL,d16", size: 3, cycles: 12, _fn: Instruction::op_0021 },
            0x0022 => Instruction { opcode, mnemonic: "LD (HL+),A", size: 1, cycles: 8, _fn: Instruction::op_0022 },
            0x0023 => Instruction { opcode, mnemonic: "INC HL", size: 1, cycles: 8, _fn: Instruction::op_0023 },
            0x0024 => Instruction { opcode, mnemonic: "INC H", size: 1, cycles: 4, _fn: Instruction::op_0024 },
            0x0028 => Instruction { opcode, mnemonic: "JR Z,r8", size: 2, cycles: 8, _fn: Instruction::op_0028 },
            0x002E => Instruction { opcode, mnemonic: "LD L,d8", size: 2, cycles: 8, _fn: Instruction::op_002e },
            0x0031 => Instruction { opcode, mnemonic: "LD SP,d16", size: 3, cycles: 12, _fn: Instruction::op_0031 },
            0x0032 => Instruction { opcode, mnemonic: "LD (HL-),A", size: 1, cycles: 8, _fn: Instruction::op_0032 },
            // TODO 0x0034 => Instruction{ opcode, mnemonic: "INC (HL)", size: 1, cycles: 12, _fn: Instruction::op_0034 },
            0x003D => Instruction { opcode, mnemonic: "DEC A", size: 1, cycles: 4, _fn: Instruction::op_003d },
            0x003E => Instruction { opcode, mnemonic: "LD A,d8", size: 2, cycles: 8, _fn: Instruction::op_003e },
            // TODO 0x0042 => Instruction { opcode, mnemonic: "", size: 1, cycles: 4, _fn: Instruction::op_0042 },
            0x004F => Instruction { opcode, mnemonic: "LD C,A", size: 1, cycles: 4, _fn: Instruction::op_004f },
            0x0057 => Instruction { opcode, mnemonic: "LD D,A", size: 1, cycles: 4, _fn: Instruction::op_0057 },
            0x0067 => Instruction { opcode, mnemonic: "LD H,A", size: 1, cycles: 4, _fn: Instruction::op_0067 },
            0x0077 => Instruction { opcode, mnemonic: "LD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0077 },
            0x0078 => Instruction { opcode, mnemonic: "LD A,B", size: 1, cycles: 4, _fn: Instruction::op_0078 },
            0x007B => Instruction { opcode, mnemonic: "LD A,E", size: 1, cycles: 4, _fn: Instruction::op_007b },
            0x007C => Instruction { opcode, mnemonic: "LD A,H", size: 1, cycles: 4, _fn: Instruction::op_007c },
            0x007D => Instruction { opcode, mnemonic: "LD A,L", size: 1, cycles: 4, _fn: Instruction::op_007d },
            0x0086 => Instruction { opcode, mnemonic: "ADD (HL)", size: 1, cycles: 8, _fn: Instruction::op_0086 },
            0x0090 => Instruction { opcode, mnemonic: "SUB B", size: 1, cycles: 4, _fn: Instruction::op_0090 },
            0x00AF => Instruction { opcode, mnemonic: "XOR A", size: 1, cycles: 4, _fn: Instruction::op_00af },
            0x00BE => Instruction { opcode, mnemonic: "CP (HL)", size: 1, cycles: 8, _fn: Instruction::op_00be },
            0x00C1 => Instruction { opcode, mnemonic: "POP BC", size: 1, cycles: 12, _fn: Instruction::op_00c1 },
            0x00C3 => Instruction { opcode, mnemonic: "JP a16", size: 3, cycles: 16, _fn: Instruction::op_00c3 },
            0x00C5 => Instruction { opcode, mnemonic: "PUSH BC", size: 1, cycles: 16, _fn: Instruction::op_00c5 },
            0x00C9 => Instruction { opcode, mnemonic: "RET", size: 1, cycles: 16, _fn: Instruction::op_00c9 },
            0x00CD => Instruction { opcode, mnemonic: "CALL a16", size: 3, cycles: 24, _fn: Instruction::op_00cd },
            0x00E0 => Instruction { opcode, mnemonic: "LD ($FF00+a8),A", size: 2, cycles: 12, _fn: Instruction::op_00e0 },
            0x00E2 => Instruction { opcode, mnemonic: "LD ($FF00+C),A", size: 1, cycles: 8, _fn: Instruction::op_00e2 },
            0x00EA => Instruction { opcode, mnemonic: "LD (a16),A", size: 3, cycles: 16, _fn: Instruction::op_00ea },
            0x00F0 => Instruction { opcode, mnemonic: "LDH A,($FF00+a8)", size: 2, cycles: 12, _fn: Instruction::op_00f0 },
            0x00F2 => Instruction { opcode, mnemonic: "LD A,($FF00+C)", size: 1, cycles: 8, _fn: Instruction::op_00f2 },
            0x00FE => Instruction { opcode, mnemonic: "CP d8", size: 2, cycles: 8, _fn: Instruction::op_00fe },
            0xCB11 => Instruction { opcode, mnemonic: "RL C", size: 2, cycles: 8, _fn: Instruction::op_cb11 },
            0xCB7C => Instruction { opcode, mnemonic: "BIT 7,H", size: 2, cycles: 8, _fn: Instruction::op_cb7c },
            _ => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: 4, _fn: Instruction::unimplemented },
        }
    }

    pub(crate) fn execute(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        (self._fn)(self, cpu, mmu)
    }

    /// JR PC+dd
    fn relative_jump(&mut self, cpu: &mut Cpu, d8: u8) {
        let jump_by = alu::signed_byte(d8);

        if jump_by < 0 {
            cpu.registers.decrement(RegIndex::PC, -jump_by as u16);
        } else {
            cpu.registers.increment(RegIndex::PC, jump_by as u16);
        }

        self.cycles += 4;
    }

    /// INC r8
    fn increment_r8(&mut self, cpu: &mut Cpu, register: RegIndex) {
        let r = cpu.registers.get_byte(register);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::increment_byte(r, original_carry);
        cpu.registers.set_byte(register, result);
        cpu.registers.set_f(flags);
    }

    /// DEC r8
    fn decrement_r8(&mut self, cpu: &mut Cpu, register: RegIndex) {
        let r = cpu.registers.get_byte(register);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::decrement_byte(r, original_carry);
        cpu.registers.set_byte(register, result);
        cpu.registers.set_f(flags);
    }

    fn unimplemented(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        println!("Opcode {:#06X} (\"{}\") is unimplemented.", self.opcode, self.mnemonic);
        -1
    }

    /// UN-PREFIXED

    /// NOP
    /// 1 4
    /// - - - -
    fn op_0000(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        self.cycles as i16
    }

    /// LD (BC),A
    /// 1 8
    /// - - - -
    fn op_0002(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let target_address = cpu.registers.get_word(RegIndex::BC);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
        self.cycles as i16
    }

    /// INC B
    /// 1 4
    /// Z 0 H -
    fn op_0004(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        self.increment_r8(cpu, RegIndex::B);
        self.cycles as i16
    }

    /// DEC B
    /// 1 4
    /// Z 1 H -
    fn op_0005(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        self.decrement_r8(cpu, RegIndex::B);
        self.cycles as i16
    }

    /// LD B,d8
    /// 2 8
    /// - - - -
    fn op_0006(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d8 = cpu.read_byte_at_pc(mmu);
        cpu.registers.set_byte(RegIndex::B, d8);
        self.cycles as i16
    }

    /// INC C
    /// 1 4
    /// Z 0 H -
    fn op_000c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        self.increment_r8(cpu, RegIndex::C);
        self.cycles as i16
    }

    /// DEC C
    /// 1 4
    /// Z 1 H -
    fn op_000d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        self.decrement_r8(cpu, RegIndex::C);
        self.cycles as i16
    }

    /// LD C,d8
    /// 2 8
    /// - - - -
    fn op_000e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d8 = cpu.read_byte_at_pc(mmu);
        cpu.registers.set_byte(RegIndex::C, d8);
        self.cycles as i16
    }

    /// LD DE,d16
    /// 3 12
    /// - - - -
    fn op_0011(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d16 = cpu.read_word_at_pc(mmu);
        cpu.registers.set_word(RegIndex::DE, d16);
        self.cycles as i16
    }

    /// INC DE
    /// 1 8
    /// - - - -
    fn op_0013(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        cpu.registers.increment(RegIndex::DE, 1);
        self.cycles as i16
    }

    /// DEC D
    /// 1 4
    /// Z 1 H -
    fn op_0015(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        self.decrement_r8(cpu, RegIndex::D);
        self.cycles as i16
    }

    /// LD D,d8
    /// 2 8
    /// - - - -
    fn op_0016(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d8 = cpu.read_byte_at_pc(mmu);
        cpu.registers.set_byte(RegIndex::D, d8);
        self.cycles as i16
    }

    /// RLA
    /// 1 4
    /// Z 0 0 C
    fn op_0017(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::A);
        let mut flags = cpu.registers.get_flags();
        flags = alu::rl(value, flags.carry);
        cpu.registers.set_f(flags);
        self.cycles as i16
    }

    /// JR r8
    /// 2 12
    /// - - - -
    fn op_0018(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d8 = cpu.read_byte_at_pc(mmu);
        self.relative_jump(cpu, d8);
        self.cycles as i16
    }

    /// LD A,(DE)
    /// 1 8
    /// - - - -
    fn op_001a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let source_address = cpu.registers.get_word(RegIndex::DE);
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(RegIndex::A, value);
        self.cycles as i16
    }

    /// DEC E
    /// 1 4
    /// Z 1 H -
    fn op_001d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        self.decrement_r8(cpu, RegIndex::E);
        self.cycles as i16
    }

    /// LD E,d8
    /// 2 8
    /// - - - -
    fn op_001e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d8 = cpu.read_byte_at_pc(mmu);
        cpu.registers.set_byte(RegIndex::E, d8);
        self.cycles as i16
    }

    /// JR NZ,r8
    /// 2 12/8
    /// - - - -
    fn op_0020(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d8 = cpu.read_byte_at_pc(mmu);
        if !(cpu.registers.get_flags().zero) {
            self.relative_jump(cpu, d8);
        }
        self.cycles as i16
    }

    /// LD HL,d16
    /// 3 12
    /// - - - -
    fn op_0021(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d16 = cpu.read_word_at_pc(mmu);
        cpu.registers.set_word(RegIndex::HL, d16);
        self.cycles as i16
    }

    /// LD (HL+),A
    /// 1 8
    /// - - - -
    fn op_0022(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let target_address = cpu.registers.get_word(RegIndex::HL);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
        cpu.registers.increment(RegIndex::HL, 1);
        self.cycles as i16
    }

    /// INC HL
    /// 1 8
    /// - - - -
    fn op_0023(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        cpu.registers.increment(RegIndex::HL, 1);
        self.cycles as i16
    }

    /// INC H
    /// 1 4
    /// Z 0 H -
    fn op_0024(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        cpu.registers.increment(RegIndex::H, 1);
        self.cycles as i16
    }

    /// JR Z,r8
    /// 2 12/8
    /// - - - -
    fn op_0028(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d8 = cpu.read_byte_at_pc(mmu);
        if cpu.registers.get_flags().zero {
            self.relative_jump(cpu, d8);
        }
        self.cycles as i16
    }

    /// LD L,d8
    /// 2 8
    /// - - - -
    fn op_002e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d8 = cpu.read_byte_at_pc(mmu);
        cpu.registers.set_byte(RegIndex::L, d8);
        self.cycles as i16
    }

    /// LD SP,d16
    /// 3 12
    /// - - - -
    fn op_0031(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d16 = cpu.read_word_at_pc(mmu);
        cpu.registers.set_word(RegIndex::SP, d16);
        self.cycles as i16
    }

    /// LD (HL-),A
    /// 1 8
    /// - - - -
    fn op_0032(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let target_address = cpu.registers.get_word(RegIndex::HL);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
        cpu.registers.decrement(RegIndex::HL, 1);
        self.cycles as i16
    }

    /// DEC A
    /// 1 4
    /// Z 1 H -
    fn op_003d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        self.decrement_r8(cpu, RegIndex::A);
        self.cycles as i16
    }

    /// INC (HL)
    /// 1 12
    /// Z 0 H -
    fn op_0034(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        cpu.registers.increment(RegIndex::HL, 1);
        self.cycles as i16
    }

    /// LD A,d8
    /// 2 8
    /// - - - -
    fn op_003e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let d8 = cpu.read_byte_at_pc(mmu);
        cpu.registers.set_byte(RegIndex::A, d8);
        self.cycles as i16
    }

    /// LD C,A
    /// 1 4
    /// - - - -
    fn op_004f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::A);
        cpu.registers.set_byte(RegIndex::C, value);
        self.cycles as i16
    }

    /// LD D,A
    /// 1 4
    /// - - - -
    fn op_0057(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::A);
        cpu.registers.set_byte(RegIndex::D, value);
        self.cycles as i16
    }

    /// LD H,A
    /// 1 4
    /// - - - -
    fn op_0067(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::A);
        cpu.registers.set_byte(RegIndex::H, value);
        self.cycles as i16
    }

    /// LD (HL),A
    /// 1 8
    /// - - - -
    fn op_0077(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let target_address = cpu.registers.get_word(RegIndex::HL);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
        self.cycles as i16
    }

    /// LD A,B
    /// 1 4
    /// - - - -
    fn op_0078(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::B);
        cpu.registers.set_byte(RegIndex::A, value);
        self.cycles as i16
    }

    /// LD A,E
    /// 1 4
    /// - - - -
    fn op_007b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::E);
        cpu.registers.set_byte(RegIndex::A, value);
        self.cycles as i16
    }

    /// LD A,H
    /// 1 4
    /// - - - -
    fn op_007c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::H);
        cpu.registers.set_byte(RegIndex::A, value);
        self.cycles as i16
    }

    /// LD A,L
    /// 1 4
    /// - - - -
    fn op_007d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::L);
        cpu.registers.set_byte(RegIndex::A, value);
        self.cycles as i16
    }

    /// ADD (HL)
    /// 1 8
    /// Z 0 H C
    fn op_0086(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let a = cpu.registers.get_byte(RegIndex::A);
        let hl = cpu.registers.get_word(RegIndex::HL);
        let b = mmu.read_byte(hl);
        let (result, flags) = alu::add_byte(a, b);
        cpu.registers.set_f(flags);
        cpu.registers.set_byte(RegIndex::A, result);
        self.cycles as i16
    }

    /// SUB B
    /// 1 4
    /// Z 1 H C
    fn op_0090(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let a = cpu.registers.get_byte(RegIndex::A);
        let b = cpu.registers.get_byte(RegIndex::B);
        let (result, flags) = alu::subtract_byte(a, b);
        cpu.registers.set_f(flags);
        cpu.registers.set_byte(RegIndex::A, result);
        self.cycles as i16
    }

    /// XOR A
    /// 1 4
    /// Z 0 0 0
    fn op_00af(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
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
        self.cycles as i16
    }

    /// CP (HL)
    /// 1 8
    /// Z 1 H C
    fn op_00be(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let a = cpu.registers.get_byte(RegIndex::A);
        let source_address = cpu.registers.get_word(RegIndex::HL);
        let d8 = mmu.read_byte(source_address);
        let (result, flags) = alu::subtract_byte(a, d8);
        cpu.registers.set_f(flags);
        self.cycles as i16
    }

    /// POP BC
    /// 1 12
    /// - - - -
    fn op_00c1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        // rr=(SP)
        let sp = cpu.registers.get_word(RegIndex::SP);
        let value = mmu.read_word(sp, Endianness::BIG);
        cpu.registers.set_word(RegIndex::BC, value);
        // SP=SP+2
        cpu.registers.increment(RegIndex::SP, 2);
        self.cycles as i16
    }

    /// JP a16
    /// 3 16
    /// - - - -
    fn op_00c3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let a16 = cpu.read_word_at_pc(mmu);
        cpu.registers.set_word(RegIndex::PC, a16);
        self.cycles as i16
    }

    /// PUSH BC
    /// 1 16
    /// - - - -
    fn op_00c5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        // SP=SP-2
        cpu.registers.decrement(RegIndex::SP, 2);
        // (SP)=BC
        let value = cpu.registers.get_word(RegIndex::BC);
        let sp = cpu.registers.get_word(RegIndex::SP);
        mmu.load_word(sp, value, Endianness::BIG);
        self.cycles as i16
    }

    /// RET
    /// 1 16
    /// - - - -
    fn op_00c9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        // PC=(SP)
        let sp = cpu.registers.get_word(RegIndex::SP);
        let jump_to_address = mmu.read_word(sp, Endianness::BIG);
        cpu.registers.set_word(RegIndex::PC, jump_to_address);
        // SP=SP+2
        cpu.registers.increment(RegIndex::SP, 2);
        self.cycles as i16
    }

    /// CALL a16
    /// 3 24
    /// - - - -
    fn op_00cd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let a16 = cpu.read_word_at_pc(mmu);

        // SP=SP-2
        cpu.registers.decrement(RegIndex::SP, 2);
        // (SP)=PC
        let sp = cpu.registers.get_word(RegIndex::SP);
        let pc = cpu.registers.get_word(RegIndex::PC);
        mmu.load_word(sp, pc, Endianness::BIG);
        // PC=nn
        cpu.registers.set_word(RegIndex::PC, a16);

        self.cycles as i16
    }

    /// LD ($FF00+a8),A
    /// 2 12
    /// - - - -
    fn op_00e0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let a8 = cpu.read_byte_at_pc(mmu);
        let target_address = 0xFF00 + (a8 as u16);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
        self.cycles as i16
    }

    /// LD ($FF00+C),A
    /// 1 8
    /// - - - -
    fn op_00e2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let target_address = 0xFF00 + (cpu.registers.get_byte(RegIndex::C) as u16);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
        self.cycles as i16
    }
    
    /// LD (a16),A 
    /// 3 16 
    /// - - - -
    fn op_00ea(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let a16 = cpu.read_word_at_pc(mmu);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(a16, value);
        self.cycles as i16
    }

    /// LD A,($FF00+a8)
    /// 2 12
    /// - - - -
    fn op_00f0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let a8 = cpu.read_byte_at_pc(mmu);
        let source_address = 0xFF00 + a8 as u16;
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(RegIndex::A, value);
        self.cycles as i16
    }

    /// LD A,($FF00+C)
    /// 1 8
    /// - - - -
    fn op_00f2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let source_address = 0xFF00 + (cpu.registers.get_byte(RegIndex::C) as u16);
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(RegIndex::A, value);
        self.cycles as i16
    }

    /// CP d8
    /// 2 8
    /// Z 1 H C
    fn op_00fe(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let a = cpu.registers.get_byte(RegIndex::A);
        let d8 = cpu.read_byte_at_pc(mmu);
        let (result, flags) = alu::subtract_byte(a, d8);
        cpu.registers.set_f(flags);
        self.cycles as i16
    }

    /// ============ CB PREFIXED ============

    /// RL C
    /// 2 8
    /// Z 0 0 C
    fn op_cb11(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::C);
        let mut flags = cpu.registers.get_flags();
        flags = alu::rl(value, flags.carry);
        cpu.registers.set_f(flags);
        self.cycles as i16
    }

    /// BIT 7,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb7c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> i16 {
        let value = cpu.registers.get_byte(RegIndex::H);
        let mut flags = cpu.registers.get_flags();
        // The opposite of the nth bit of the second operand is written into the Z flag.
        flags.zero = value & 0b1000_0000 != 0;
        flags.subtract = false;
        flags.half_carry = true;
        cpu.registers.set_f(flags);
        self.cycles as i16
    }
}
