#![allow(unused_variables)]

use crate::console::alu;
use crate::console::cpu::{Cpu, PREFIX_BYTE};
use crate::console::input::CallbackAction::DEBUG;
use crate::console::mmu::{Endianness, Mmu};
use crate::console::cpu_registers::{Flags, CpuRegIndex};

pub(crate) struct Instruction {
    pub(crate) opcode: u16,
    pub(crate) mnemonic: &'static str,
    pub(crate) size: u16,
    pub(crate) cycles: i16,
    _fn: fn(&mut Instruction, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16,
}

impl Instruction {
    pub(crate) fn get_instruction(opcode: u16) -> Instruction {
        match opcode {
            0x0000 => Instruction { opcode, mnemonic: "NOP", size: 1, cycles: 4, _fn: Instruction::op_0000 },
            0x0001 => Instruction { opcode, mnemonic: "LD BC,d16", size: 3, cycles: 12, _fn: Instruction::op_0001 },
            0x0002 => Instruction { opcode, mnemonic: "LD (BC),A", size: 1, cycles: 8, _fn: Instruction::op_0002 },
            0x0003 => Instruction { opcode, mnemonic: "INC BC", size: 1, cycles: 8, _fn: Instruction::op_0003 },
            0x0004 => Instruction { opcode, mnemonic: "INC B", size: 1, cycles: 4, _fn: Instruction::op_0004 },
            0x0005 => Instruction { opcode, mnemonic: "DEC B", size: 1, cycles: 4, _fn: Instruction::op_0005 },
            0x0006 => Instruction { opcode, mnemonic: "LD B,d8", size: 2, cycles: 8, _fn: Instruction::op_0006 },
            0x0007 => Instruction { opcode, mnemonic: "RLCA", size: 1, cycles: 4, _fn: Instruction::op_0007 },
            0x0008 => Instruction { opcode, mnemonic: "LD (a16),SP", size: 3, cycles: 20, _fn: Instruction::op_0008 },
            0x0009 => Instruction { opcode, mnemonic: "ADD HL,BC", size: 1, cycles: 8, _fn: Instruction::op_0009 },
            0x000A => Instruction { opcode, mnemonic: "LD A,(BC)", size: 1, cycles: 8, _fn: Instruction::op_000A },
            0x000B => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x000C => Instruction { opcode, mnemonic: "INC C", size: 1, cycles: 4, _fn: Instruction::op_000c },
            0x000D => Instruction { opcode, mnemonic: "DEC C", size: 1, cycles: 4, _fn: Instruction::op_000d },
            0x000E => Instruction { opcode, mnemonic: "LD C,d8", size: 2, cycles: 8, _fn: Instruction::op_000e },
            0x000F => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x0010 => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x0011 => Instruction { opcode, mnemonic: "LD DE,d16", size: 3, cycles: 12, _fn: Instruction::op_0011 },
            0x0012 => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x0013 => Instruction { opcode, mnemonic: "INC DE", size: 1, cycles: 8, _fn: Instruction::op_0013 },
            0x0014 => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x0015 => Instruction { opcode, mnemonic: "DEC D", size: 1, cycles: 4, _fn: Instruction::op_0015 },
            0x0016 => Instruction { opcode, mnemonic: "LD D,d8", size: 2, cycles: 8, _fn: Instruction::op_0016 },
            0x0017 => Instruction { opcode, mnemonic: "RLA", size: 1, cycles: 4, _fn: Instruction::op_0017 },
            0x0018 => Instruction { opcode, mnemonic: "JR r8", size: 2, cycles: 12, _fn: Instruction::op_0018 },
            0x0019 => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x001A => Instruction { opcode, mnemonic: "LD A,(DE)", size: 1, cycles: 8, _fn: Instruction::op_001a },
            0x001B => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x001C => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x001D => Instruction { opcode, mnemonic: "DEC E", size: 1, cycles: 4, _fn: Instruction::op_001d },
            0x001E => Instruction { opcode, mnemonic: "LD E,d8", size: 2, cycles: 8, _fn: Instruction::op_001e },
            0x001F => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x0020 => Instruction { opcode, mnemonic: "JR NZ,r8", size: 2, cycles: 8, _fn: Instruction::op_0020 },
            0x0021 => Instruction { opcode, mnemonic: "LD HL,d16", size: 3, cycles: 12, _fn: Instruction::op_0021 },
            0x0022 => Instruction { opcode, mnemonic: "LDI (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0022 },
            0x0023 => Instruction { opcode, mnemonic: "INC HL", size: 1, cycles: 8, _fn: Instruction::op_0023 },
            0x0024 => Instruction { opcode, mnemonic: "INC H", size: 1, cycles: 4, _fn: Instruction::op_0024 },
            0x0025 => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x0026 => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x0028 => Instruction { opcode, mnemonic: "JR Z,r8", size: 2, cycles: 8, _fn: Instruction::op_0028 },
            0x002A => Instruction { opcode, mnemonic: "LDI A,[HL]", size: 1, cycles: 8, _fn: Instruction::op_002a },
            0x002E => Instruction { opcode, mnemonic: "LD L,d8", size: 2, cycles: 8, _fn: Instruction::op_002e },
            0x0031 => Instruction { opcode, mnemonic: "LD SP,d16", size: 3, cycles: 12, _fn: Instruction::op_0031 },
            0x0032 => Instruction { opcode, mnemonic: "LDD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0032 },
            // TODO 0x0034 => Instruction{ opcode, mnemonic: "INC (HL)", size: 1, cycles: 12, _fn: Instruction::op_0034 },
            0x0036 => Instruction { opcode, mnemonic: "LD (HL),d8", size: 2, cycles: 12, _fn: Instruction::op_0036 },
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
            0x00E0 => Instruction { opcode, mnemonic: "LDH (a8),A", size: 2, cycles: 12, _fn: Instruction::op_00e0 },
            0x00E2 => Instruction { opcode, mnemonic: "LD ($FF00+C),A", size: 1, cycles: 8, _fn: Instruction::op_00e2 },
            0x00EA => Instruction { opcode, mnemonic: "LD (a16),A", size: 3, cycles: 16, _fn: Instruction::op_00ea },
            0x00F0 => Instruction { opcode, mnemonic: "LDH A,($FF00+a8)", size: 2, cycles: 12, _fn: Instruction::op_00f0 },
            0x00F2 => Instruction { opcode, mnemonic: "LD A,($FF00+C)", size: 1, cycles: 8, _fn: Instruction::op_00f2 },
            0x00F3 => Instruction { opcode, mnemonic: "DI", size: 1, cycles: 4, _fn: Instruction::op_00f3 },
            0x00FE => Instruction { opcode, mnemonic: "CP d8", size: 2, cycles: 8, _fn: Instruction::op_00fe },
            0xCB11 => Instruction { opcode, mnemonic: "RL C", size: 2, cycles: 8, _fn: Instruction::op_cb11 },
            0xCB7C => Instruction { opcode, mnemonic: "BIT 7,H", size: 2, cycles: 8, _fn: Instruction::op_cb7c },
            _ => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
        }
    }

    pub(crate) fn is_cbprefixed(&self) -> bool {
        ((self.opcode & 0xFF00) >> 8) as u8 == PREFIX_BYTE
    }

    pub(crate) fn execute(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        (self._fn)(self, cpu, mmu, args)
    }

    fn unimplemented(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        println!("Opcode {:#06X} (\"{}\") is unimplemented.", self.opcode, self.mnemonic);
        self.cycles
    }

    fn read_args_as_word(arg1: u8, arg2: u8) -> u16 {
        ((arg2 as u16) << 8) | (arg1 as u16)
    }
    
    fn relative_jump(&mut self, cpu: &mut Cpu, d8: u8) {
        let jump_by = alu::signed_byte(d8);

        if jump_by < 0 {
            cpu.registers.decrement(CpuRegIndex::PC, -jump_by as u16);
        } else {
            cpu.registers.increment(CpuRegIndex::PC, jump_by as u16);
        }

        self.cycles += 4;
    }

    /// ADD/SUB 8 bit -- Affects all flags
    fn add_r8_r8(&mut self, cpu: &mut Cpu, target: CpuRegIndex, source: CpuRegIndex) {
        let a = cpu.registers.get_byte(target);
        let b = cpu.registers.get_byte(source);
        let (result, flags) = alu::add_byte(a, b);
        cpu.registers.set_byte(target, result);
        cpu.registers.set_flags(flags);
    }

    fn add_a16_r8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, address: u16, r8: CpuRegIndex) {
        let a = mmu.read_byte(address);
        let b = cpu.registers.get_byte(r8);
        let (result, flags) = alu::add_byte(a, b);
        mmu.load_byte(address, result);
        cpu.registers.set_flags(flags);
    }

    /// ADD/SUB 16 bit -- Affects all flags
    fn add_r16_r16(&mut self, cpu: &mut Cpu, target: CpuRegIndex, source: CpuRegIndex) {
        let a = cpu.registers.get_word(target);
        let b = cpu.registers.get_word(source);
        let (result, flags) = alu::add_word(a, b);
        cpu.registers.set_word(target, result);
        cpu.registers.set_flags(flags);
    }

    /// INC/DEC 8 bit -- Affects all flags except carry flag
    fn increment_r8(&mut self, cpu: &mut Cpu, register: CpuRegIndex) {
        let r8 = cpu.registers.get_byte(register);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::increment_byte(r8, original_carry);
        cpu.registers.set_byte(register, result);
        cpu.registers.set_flags(flags);
    }

    fn increment_a16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, address: u16) {
        let value = mmu.read_byte(address);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::increment_byte(value, original_carry);
        mmu.load_byte(address, result);
        cpu.registers.set_flags(flags);
    }

    fn decrement_r8(&mut self, cpu: &mut Cpu, register: CpuRegIndex) {
        let r = cpu.registers.get_byte(register);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::decrement_byte(r, original_carry);
        cpu.registers.set_byte(register, result);
        cpu.registers.set_flags(flags);
    }

    /// INC/DEC 16 bit -- Does not affect flags
    fn increment_r16(&mut self, cpu: &mut Cpu, register: CpuRegIndex) {
        let r16 = cpu.registers.get_word(register);
        let result = alu::increment_word(r16);
        cpu.registers.set_word(register, result);
    }

    /// NOP
    /// 1 4
    /// - - - -
    fn op_0000(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cycles as i16
    }

    /// LD BC,d16
    /// 3 12
    /// - - - -
    fn op_0001(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = Instruction::read_args_as_word(args[0], args[1]);
        cpu.registers.set_word(CpuRegIndex::BC, value);
        self.cycles as i16
    }
    
    /// LD (BC),A
    /// 1 8
    /// - - - -
    fn op_0002(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let target_address = cpu.registers.get_word(CpuRegIndex::BC);
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        mmu.load_byte(target_address, value);
        self.cycles as i16
    }

    /// INC BC
    /// 1 8
    /// - - - -
    fn op_0003(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        // TODO
        self.cycles as i16
    }

    /// INC B
    /// 1 4
    /// Z 0 H -
    fn op_0004(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::B);
        self.cycles as i16
    }

    /// DEC B
    /// 1 4
    /// Z 1 H -
    fn op_0005(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::B);
        self.cycles as i16
    }

    /// LD B,d8
    /// 2 8
    /// - - - -
    fn op_0006(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.registers.set_byte(CpuRegIndex::B, args[0]);
        self.cycles as i16
    }

    /// RLCA
    /// 1 4
    /// 0 0 0 C
    fn op_0007(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        let (result, flags) = alu::rotate_left_circular(value);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self.cycles as i16
    }

    /// LD (a16),SP
    /// 3 20
    /// - - - -
    fn op_0008(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let target_address = Instruction::read_args_as_word(args[0], args[1]);
        let value = cpu.registers.get_word(CpuRegIndex::SP);
        mmu.load_word(target_address, value, Endianness::BIG);
        self.cycles as i16
    }

    /// ADD HL,BC
    /// 1 8
    /// - 0 H C
    fn op_0009(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_r16_r16(cpu, CpuRegIndex::HL, CpuRegIndex::BC);
        self.cycles as i16
    }

    /// LD A,(BC)
    /// 1  8
    /// - - - -
    fn op_000A(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let source_address = cpu.registers.get_word(CpuRegIndex::BC);
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(CpuRegIndex::A, value);
        self.cycles as i16
    }


    /// INC C
    /// 1 4
    /// Z 0 H -
    fn op_000c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::C);
        self.cycles as i16
    }

    /// DEC C
    /// 1 4
    /// Z 1 H -
    fn op_000d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::C);
        self.cycles as i16
    }

    /// LD C,d8
    /// 2 8
    /// - - - -
    fn op_000e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.registers.set_byte(CpuRegIndex::C, args[0]);
        self.cycles as i16
    }

    /// LD DE,d16
    /// 3 12
    /// - - - -
    fn op_0011(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = Instruction::read_args_as_word(args[0], args[1]);
        cpu.registers.set_word(CpuRegIndex::DE, value);
        self.cycles as i16
    }

    /// INC DE
    /// 1 8
    /// - - - -
    fn op_0013(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r16(cpu, CpuRegIndex::DE);
        self.cycles as i16
    }

    /// DEC D
    /// 1 4
    /// Z 1 H -
    fn op_0015(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::D);
        self.cycles as i16
    }

    /// LD D,d8
    /// 2 8
    /// - - - -
    fn op_0016(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.registers.set_byte(CpuRegIndex::D, args[0]);
        self.cycles as i16
    }

    /// RLA
    /// 1 4
    /// Z 0 0 C
    fn op_0017(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        let old_flags = cpu.registers.get_flags();
        let (result, flags) = alu::rotate_left(value, old_flags.carry);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self.cycles as i16
    }

    /// JR r8
    /// 2 12
    /// - - - -
    fn op_0018(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.relative_jump(cpu, args[0]);
        self.cycles as i16
    }

    /// LD A,(DE)
    /// 1 8
    /// - - - -
    fn op_001a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let source_address = cpu.registers.get_word(CpuRegIndex::DE);
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(CpuRegIndex::A, value);
        self.cycles as i16
    }

    /// DEC E
    /// 1 4
    /// Z 1 H -
    fn op_001d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::E);
        self.cycles as i16
    }

    /// LD E,d8
    /// 2 8
    /// - - - -
    fn op_001e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.registers.set_byte(CpuRegIndex::E, args[0]);
        self.cycles as i16
    }

    /// JR NZ,r8
    /// 2 12/8
    /// - - - -
    fn op_0020(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !(cpu.registers.get_flags().zero) {
            self.relative_jump(cpu, args[0]);
        }
        self.cycles as i16
    }

    /// LD HL,d16
    /// 3 12
    /// - - - -
    fn op_0021(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = Instruction::read_args_as_word(args[0], args[1]);
        cpu.registers.set_word(CpuRegIndex::HL, value);
        self.cycles as i16
    }

    /// LDI (HL),A
    /// aka LD (HL+),A
    /// 1 8
    /// - - - -
    fn op_0022(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let target_address = cpu.registers.get_word(CpuRegIndex::HL);
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        mmu.load_byte(target_address, value);
        cpu.registers.increment(CpuRegIndex::HL, 1);
        self.cycles as i16
    }

    /// INC HL
    /// 1 8
    /// - - - -
    fn op_0023(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r16(cpu, CpuRegIndex::HL);
        self.cycles as i16
    }

    /// INC H
    /// 1 4
    /// Z 0 H -
    fn op_0024(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::H);
        self.cycles as i16
    }

    /// JR Z,r8
    /// 2 12/8
    /// - - - -
    fn op_0028(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if cpu.registers.get_flags().zero {
            self.relative_jump(cpu, args[0]);
        }
        self.cycles as i16
    }

    /// LDI A,(HL)
    /// aka LD A, (HL+)
    /// 1 8
    /// - - - -
    fn op_002a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let source_address = cpu.registers.get_word(CpuRegIndex::HL);
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(CpuRegIndex::A, value);
        cpu.registers.increment(CpuRegIndex::HL, 1);
        self.cycles as i16
    }

    /// LD L,d8
    /// 2 8
    /// - - - -
    fn op_002e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.registers.set_byte(CpuRegIndex::L, args[0]);
        self.cycles as i16
    }

    /// LD SP,d16
    /// 3 12
    /// - - - -
    fn op_0031(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = Instruction::read_args_as_word(args[0], args[1]);
        cpu.registers.set_word(CpuRegIndex::SP, value);
        self.cycles as i16
    }

    /// LDD (HL),A
    /// aka LD (HL-),A
    /// 1 8
    /// - - - -
    fn op_0032(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let target_address = cpu.registers.get_word(CpuRegIndex::HL);
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        mmu.load_byte(target_address, value);
        cpu.registers.decrement(CpuRegIndex::HL, 1);
        self.cycles as i16
    }

    /// LD (HL),d8
    /// 2 12
    /// - - - -
    fn op_0036(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let target_address = cpu.registers.get_word(CpuRegIndex::HL);
        mmu.load_byte(target_address, args[0]);
        self.cycles as i16
    }

    /// DEC A
    /// 1 4
    /// Z 1 H -
    fn op_003d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::A);
        self.cycles as i16
    }

    /// INC (HL)
    /// 1 12
    /// Z 0 H -
    fn op_0034(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = cpu.registers.get_word(CpuRegIndex::HL);
        self.increment_a16(cpu, mmu, address);
        self.cycles as i16
    }

    /// LD A,d8
    /// 2 8
    /// - - - -
    fn op_003e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.registers.set_byte(CpuRegIndex::A, args[0]);
        self.cycles as i16
    }

    /// LD C,A
    /// 1 4
    /// - - - -
    fn op_004f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        cpu.registers.set_byte(CpuRegIndex::C, value);
        self.cycles as i16
    }

    /// LD D,A
    /// 1 4
    /// - - - -
    fn op_0057(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        cpu.registers.set_byte(CpuRegIndex::D, value);
        self.cycles as i16
    }

    /// LD H,A
    /// 1 4
    /// - - - -
    fn op_0067(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        cpu.registers.set_byte(CpuRegIndex::H, value);
        self.cycles as i16
    }

    /// LD (HL),A
    /// 1 8
    /// - - - -
    fn op_0077(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let target_address = cpu.registers.get_word(CpuRegIndex::HL);
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        mmu.load_byte(target_address, value);
        self.cycles as i16
    }

    /// LD A,B
    /// 1 4
    /// - - - -
    fn op_0078(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::B);
        cpu.registers.set_byte(CpuRegIndex::A, value);
        self.cycles as i16
    }

    /// LD A,E
    /// 1 4
    /// - - - -
    fn op_007b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::E);
        cpu.registers.set_byte(CpuRegIndex::A, value);
        self.cycles as i16
    }

    /// LD A,H
    /// 1 4
    /// - - - -
    fn op_007c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::H);
        cpu.registers.set_byte(CpuRegIndex::A, value);
        self.cycles as i16
    }

    /// LD A,L
    /// 1 4
    /// - - - -
    fn op_007d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::L);
        cpu.registers.set_byte(CpuRegIndex::A, value);
        self.cycles as i16
    }

    /// ADD (HL)
    /// 1 8
    /// Z 0 H C
    fn op_0086(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let hl = cpu.registers.get_word(CpuRegIndex::HL);
        let b = mmu.read_byte(hl);
        let (result, flags) = alu::add_byte(a, b);
        cpu.registers.set_flags(flags);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        self.cycles as i16
    }

    /// SUB B
    /// 1 4
    /// Z 1 H C
    fn op_0090(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let b = cpu.registers.get_byte(CpuRegIndex::B);
        let (result, flags) = alu::subtract_byte(a, b);
        cpu.registers.set_flags(flags);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        self.cycles as i16
    }

    /// XOR A
    /// 1 4
    /// Z 0 0 0
    fn op_00af(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let mut value = cpu.registers.get_byte(CpuRegIndex::A);
        value ^= value;
        cpu.registers.set_byte(CpuRegIndex::A, value);
        let z = cpu.registers.get_byte(CpuRegIndex::A) == 0;
        let flags = Flags {
            zero: z,
            subtract: false,
            half_carry: false,
            carry: false,
        };
        cpu.registers.set_flags(flags);
        self.cycles as i16
    }

    /// CP (HL)
    /// 1 8
    /// Z 1 H C
    fn op_00be(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let source_address = cpu.registers.get_word(CpuRegIndex::HL);
        let b = mmu.read_byte(source_address);
        let (result, flags) = alu::subtract_byte(a, b);
        cpu.registers.set_flags(flags);
        self.cycles as i16
    }

    /// POP BC
    /// 1 12
    /// - - - -
    fn op_00c1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        // rr=(SP)
        let sp = cpu.registers.get_word(CpuRegIndex::SP);
        let value = mmu.read_word(sp, Endianness::BIG);
        cpu.registers.set_word(CpuRegIndex::BC, value);
        // SP=SP+2
        cpu.registers.increment(CpuRegIndex::SP, 2);
        self.cycles as i16
    }

    /// JP a16
    /// 3 16
    /// - - - -
    fn op_00c3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = Instruction::read_args_as_word(args[0], args[1]);
        cpu.registers.set_word(CpuRegIndex::PC, address);
        self.cycles as i16
    }

    /// PUSH BC
    /// 1 16
    /// - - - -
    fn op_00c5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        // SP=SP-2
        cpu.registers.decrement(CpuRegIndex::SP, 2);
        // (SP)=BC
        let value = cpu.registers.get_word(CpuRegIndex::BC);
        let sp = cpu.registers.get_word(CpuRegIndex::SP);
        mmu.load_word(sp, value, Endianness::BIG);
        self.cycles as i16
    }

    /// RET
    /// 1 16
    /// - - - -
    fn op_00c9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        // PC=(SP)
        let sp = cpu.registers.get_word(CpuRegIndex::SP);
        let jump_to_address = mmu.read_word(sp, Endianness::BIG);
        cpu.registers.set_word(CpuRegIndex::PC, jump_to_address);
        // SP=SP+2
        cpu.registers.increment(CpuRegIndex::SP, 2);
        self.cycles as i16
    }

    /// CALL a16
    /// 3 24
    /// - - - -
    fn op_00cd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = Instruction::read_args_as_word(args[0], args[1]);

        // SP=SP-2
        cpu.registers.decrement(CpuRegIndex::SP, 2);

        // (SP)=PC
        let sp = cpu.registers.get_word(CpuRegIndex::SP);
        let pc = cpu.registers.get_word(CpuRegIndex::PC);
        mmu.load_word(sp, pc, Endianness::BIG);

        // PC=nn
        cpu.registers.set_word(CpuRegIndex::PC, address);

        self.cycles as i16
    }

    /// LDH (a8),A
    /// aka LD ($FF00+a8),A
    /// 2 12
    /// - - - -
    fn op_00e0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let target_address = 0xFF00 + (args[0] as u16);
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        mmu.load_byte(target_address, value);
        self.cycles as i16
    }

    /// LD ($FF00+C),A
    /// 1 8
    /// - - - -
    fn op_00e2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let target_address = 0xFF00 + (cpu.registers.get_byte(CpuRegIndex::C) as u16);
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        mmu.load_byte(target_address, value);
        self.cycles as i16
    }
    
    /// LD (a16),A 
    /// 3 16 
    /// - - - -
    fn op_00ea(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let target_address = Instruction::read_args_as_word(args[0], args[1]);
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        mmu.load_byte(target_address, value);
        self.cycles as i16
    }

    /// LD A,($FF00+a8)
    /// 2 12
    /// - - - -
    fn op_00f0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let source_address = 0xFF00 + args[0] as u16;
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(CpuRegIndex::A, value);
        self.cycles as i16
    }

    /// LD A,($FF00+C)
    /// 1 8
    /// - - - -
    fn op_00f2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let source_address = 0xFF00 + (cpu.registers.get_byte(CpuRegIndex::C) as u16);
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(CpuRegIndex::A, value);
        self.cycles as i16
    }

    /// DI
    /// 1 4
    /// - - - -
    fn op_00f3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.interrupts.set_ime(false, mmu);
        self.cycles as i16
    }

    /// CP d8
    /// 2 8
    /// Z 1 H C
    fn op_00fe(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let (result, flags) = alu::subtract_byte(a, args[0]);
        cpu.registers.set_flags(flags);
        self.cycles as i16
    }

    /// ============ CB PREFIXED ============

    /// RL C
    /// 2 8
    /// Z 0 0 C
    fn op_cb11(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::C);
        let old_flags = cpu.registers.get_flags();
        let (result, flags) = alu::rotate_left(value, old_flags.carry);
        cpu.registers.set_byte(CpuRegIndex::C, result);
        cpu.registers.set_flags(flags);
        self.cycles as i16
    }

    /// BIT 7,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb7c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::H);
        let mut flags = cpu.registers.get_flags();
        // The opposite of the nth bit of the second operand is written into the Z flag.
        flags.zero = value & 0b1000_0000 != 0;
        flags.subtract = false;
        flags.half_carry = true;
        cpu.registers.set_flags(flags);
        self.cycles as i16
    }
}
