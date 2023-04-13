#![allow(unused_variables)]

use sdl2::log::set_output_function;
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
            0x000A => Instruction { opcode, mnemonic: "LD A,(BC)", size: 1, cycles: 8, _fn: Instruction::op_000a },
            0x000B => Instruction { opcode, mnemonic: "DEC BC", size: 1, cycles: 8, _fn: Instruction::op_000b },
            0x000C => Instruction { opcode, mnemonic: "INC C", size: 1, cycles: 4, _fn: Instruction::op_000c },
            0x000D => Instruction { opcode, mnemonic: "DEC C", size: 1, cycles: 4, _fn: Instruction::op_000d },
            0x000E => Instruction { opcode, mnemonic: "LD C,d8", size: 2, cycles: 8, _fn: Instruction::op_000e },
            0x000F => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },

            0x0010 => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },
            0x0011 => Instruction { opcode, mnemonic: "LD DE,d16", size: 3, cycles: 12, _fn: Instruction::op_0011 },
            0x0012 => Instruction { opcode, mnemonic: "LD (DE),A", size: 1, cycles: 8, _fn: Instruction::op_0012 },
            0x0013 => Instruction { opcode, mnemonic: "INC DE", size: 1, cycles: 8, _fn: Instruction::op_0013 },
            0x0014 => Instruction { opcode, mnemonic: "INC D", size: 1, cycles: 4, _fn: Instruction::op_0014 },
            0x0015 => Instruction { opcode, mnemonic: "DEC D", size: 1, cycles: 4, _fn: Instruction::op_0015 },
            0x0016 => Instruction { opcode, mnemonic: "LD D,d8", size: 2, cycles: 8, _fn: Instruction::op_0016 },
            0x0017 => Instruction { opcode, mnemonic: "RLA", size: 1, cycles: 4, _fn: Instruction::op_0017 },
            0x0018 => Instruction { opcode, mnemonic: "JR r8", size: 2, cycles: 12, _fn: Instruction::op_0018 },
            0x0019 => Instruction { opcode, mnemonic: "ADD HL,DE", size: 1, cycles: 8, _fn: Instruction::op_0019 },
            0x001A => Instruction { opcode, mnemonic: "LD A,(DE)", size: 1, cycles: 8, _fn: Instruction::op_001a },
            0x001B => Instruction { opcode, mnemonic: "DEC DE", size: 1, cycles: 8, _fn: Instruction::op_001b },
            0x001C => Instruction { opcode, mnemonic: "INC E", size: 1, cycles: 4, _fn: Instruction::op_001c },
            0x001D => Instruction { opcode, mnemonic: "DEC E", size: 1, cycles: 4, _fn: Instruction::op_001d },
            0x001E => Instruction { opcode, mnemonic: "LD E,d8", size: 2, cycles: 8, _fn: Instruction::op_001e },
            0x001F => Instruction { opcode, mnemonic: "Unimplemented", size: 1, cycles: -1, _fn: Instruction::unimplemented },

            0x0020 => Instruction { opcode, mnemonic: "JR NZ,r8", size: 2, cycles: 8, _fn: Instruction::op_0020 },
            0x0021 => Instruction { opcode, mnemonic: "LD HL,d16", size: 3, cycles: 12, _fn: Instruction::op_0021 },
            0x0022 => Instruction { opcode, mnemonic: "LDI (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0022 },
            0x0023 => Instruction { opcode, mnemonic: "INC HL", size: 1, cycles: 8, _fn: Instruction::op_0023 },
            0x0024 => Instruction { opcode, mnemonic: "INC H", size: 1, cycles: 4, _fn: Instruction::op_0024 },
            0x0025 => Instruction { opcode, mnemonic: "DEC H", size: 1, cycles: 4, _fn: Instruction::op_0025 },
            0x0026 => Instruction { opcode, mnemonic: "LD H,d8", size: 2, cycles: 8, _fn: Instruction::op_0026 },
            0x0028 => Instruction { opcode, mnemonic: "JR Z,r8", size: 2, cycles: 8, _fn: Instruction::op_0028 },
            0x002A => Instruction { opcode, mnemonic: "LDI A,[HL]", size: 1, cycles: 8, _fn: Instruction::op_002a },
            0x002E => Instruction { opcode, mnemonic: "LD L,d8", size: 2, cycles: 8, _fn: Instruction::op_002e },
            0x002F => Instruction { opcode, mnemonic: "CPL", size: 1, cycles: 4, _fn: Instruction::op_002f },

            0x0030 => Instruction { opcode, mnemonic: "JR NC,r8", size: 2, cycles: 8, _fn: Instruction::op_0030 },
            0x0031 => Instruction { opcode, mnemonic: "LD SP,d16", size: 3, cycles: 12, _fn: Instruction::op_0031 },
            0x0032 => Instruction { opcode, mnemonic: "LDD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0032 },
            // 0x0034 => Instruction{ opcode, mnemonic: "INC (HL)", size: 1, cycles: 12, _fn: Instruction::op_0034 },
            0x0036 => Instruction { opcode, mnemonic: "LD (HL),d8", size: 2, cycles: 12, _fn: Instruction::op_0036 },
            // 0039 -- ADD HL, SP 1  8 - 0 H C
            0x003C => Instruction { opcode, mnemonic: "INC A", size: 1, cycles: 4, _fn: Instruction::op_003c },
            0x003D => Instruction { opcode, mnemonic: "DEC A", size: 1, cycles: 4, _fn: Instruction::op_003d },
            0x003E => Instruction { opcode, mnemonic: "LD A,d8", size: 2, cycles: 8, _fn: Instruction::op_003e },

            0x0047 => Instruction { opcode, mnemonic: "LD B,A", size: 1, cycles: 4, _fn: Instruction::op_0047 },
            0x004F => Instruction { opcode, mnemonic: "LD C,A", size: 1, cycles: 4, _fn: Instruction::op_004f },

            0x0057 => Instruction { opcode, mnemonic: "LD D,A", size: 1, cycles: 4, _fn: Instruction::op_0057 },
            0x005B => Instruction { opcode, mnemonic: "LD E,E", size: 1, cycles: 4, _fn: Instruction::op_005b },

            0x0067 => Instruction { opcode, mnemonic: "LD H,A", size: 1, cycles: 4, _fn: Instruction::op_0067 },

            0x0077 => Instruction { opcode, mnemonic: "LD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0077 },
            0x0078 => Instruction { opcode, mnemonic: "LD A,B", size: 1, cycles: 4, _fn: Instruction::op_0078 },
            0x0079 => Instruction { opcode, mnemonic: "LD A,C", size: 1, cycles: 4, _fn: Instruction::op_0079 },
            0x007B => Instruction { opcode, mnemonic: "LD A,E", size: 1, cycles: 4, _fn: Instruction::op_007b },
            0x007C => Instruction { opcode, mnemonic: "LD A,H", size: 1, cycles: 4, _fn: Instruction::op_007c },
            0x007D => Instruction { opcode, mnemonic: "LD A,L", size: 1, cycles: 4, _fn: Instruction::op_007d },

            0x0086 => Instruction { opcode, mnemonic: "ADD (HL)", size: 1, cycles: 8, _fn: Instruction::op_0086 },

            0x0090 => Instruction { opcode, mnemonic: "SUB B", size: 1, cycles: 4, _fn: Instruction::op_0090 },

            0x00A3 => Instruction { opcode, mnemonic: "AND E", size: 1, cycles: 4, _fn: Instruction::op_00a3 },
            0x00AF => Instruction { opcode, mnemonic: "XOR A", size: 1, cycles: 4, _fn: Instruction::op_00af },

            0x00B1 => Instruction { opcode, mnemonic: "OR C", size: 1, cycles: 4, _fn: Instruction::op_00b1 },
            0x00BE => Instruction { opcode, mnemonic: "CP (HL)", size: 1, cycles: 8, _fn: Instruction::op_00be },

            0x00C1 => Instruction { opcode, mnemonic: "POP BC", size: 1, cycles: 12, _fn: Instruction::op_00c1 },
            0x00C3 => Instruction { opcode, mnemonic: "JP a16", size: 3, cycles: 16, _fn: Instruction::op_00c3 },
            0x00C5 => Instruction { opcode, mnemonic: "PUSH BC", size: 1, cycles: 16, _fn: Instruction::op_00c5 },
            0x00C9 => Instruction { opcode, mnemonic: "RET", size: 1, cycles: 16, _fn: Instruction::op_00c9 },
            0x00CD => Instruction { opcode, mnemonic: "CALL a16", size: 3, cycles: 24, _fn: Instruction::op_00cd },

            0x00D5 => Instruction { opcode, mnemonic: "PUSH DE", size: 1, cycles: 16, _fn: Instruction::op_00d5 },

            0x00E0 => Instruction { opcode, mnemonic: "LDH (a8),A", size: 2, cycles: 12, _fn: Instruction::op_00e0 },
            0x00E1 => Instruction { opcode, mnemonic: "POP HL", size: 1, cycles: 12, _fn: Instruction::op_00e1 },
            0x00E2 => Instruction { opcode, mnemonic: "LDH (C),A", size: 1, cycles: 8, _fn: Instruction::op_00e2 },
            0x00E5 => Instruction { opcode, mnemonic: "PUSH HL", size: 1, cycles: 16, _fn: Instruction::op_00e5 },
            0x00EA => Instruction { opcode, mnemonic: "LD (a16),A", size: 3, cycles: 16, _fn: Instruction::op_00ea },

            0x00F0 => Instruction { opcode, mnemonic: "LDH A,(a8)", size: 2, cycles: 12, _fn: Instruction::op_00f0 },
            0x00F1 => Instruction { opcode, mnemonic: "POP AF", size: 1, cycles: 12, _fn: Instruction::op_00f1 },
            0x00F2 => Instruction { opcode, mnemonic: "LD A,($FF00+C)", size: 1, cycles: 8, _fn: Instruction::op_00f2 },
            0x00F3 => Instruction { opcode, mnemonic: "DI", size: 1, cycles: 4, _fn: Instruction::op_00f3 },
            0x00F5 => Instruction { opcode, mnemonic: "PUSH AF", size: 1, cycles: 16, _fn: Instruction::op_00f5 },
            0x00FA => Instruction { opcode, mnemonic: "LD A,(a16)", size: 3, cycles: 16, _fn: Instruction::op_00fa },
            0x00FE => Instruction { opcode, mnemonic: "CP d8", size: 2, cycles: 8, _fn: Instruction::op_00fe },
            0x00FF => Instruction { opcode, mnemonic: "RST 38H", size: 1, cycles: 16, _fn: Instruction::op_00ff },

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

    /// PUSH
    fn push(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, source_register: CpuRegIndex) {
        // SP=SP-2
        cpu.registers.decrement(CpuRegIndex::SP, 2);
        // (SP)=r16
        let value = cpu.registers.get_word(source_register);
        let target_address = cpu.registers.get_word(CpuRegIndex::SP);
        mmu.load_word(target_address, value, Endianness::BIG);
    }

    /// POP
    fn pop(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, target_register: CpuRegIndex) {
        // r16=(SP)
        let source_address = cpu.registers.get_word(CpuRegIndex::SP);
        let value = mmu.read_word(source_address, Endianness::BIG);
        cpu.registers.set_word(target_register, value);
        // SP=SP+2
        cpu.registers.increment(CpuRegIndex::SP, 2);
    }

    /// JUMP
    fn jump(&mut self, cpu: &mut Cpu, address: u16) {
        cpu.registers.set_word(CpuRegIndex::PC, address);
    }

    fn relative_jump(&mut self, cpu: &mut Cpu, d8: u8) {
        let jump_by = alu::signed_byte(d8);

        if jump_by < 0 {
            cpu.registers.decrement(CpuRegIndex::PC, (-1 * jump_by) as u16);
        } else {
            cpu.registers.increment(CpuRegIndex::PC, jump_by as u16);
        }

        self.cycles += 4;
    }

    /// CALL
    fn call(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, address: u16) {
        self.push(cpu, mmu, CpuRegIndex::PC);
        self.jump(cpu, address);
    }

    /// LD
    fn ld_r8_d8(&mut self, cpu: &mut Cpu, target: CpuRegIndex, value: u8) -> i16 {
        cpu.registers.set_byte(target, value);
        self.cycles
    }

    fn ld_r8_r8(&mut self, cpu: &mut Cpu, target: CpuRegIndex, source: CpuRegIndex) -> i16 {
        let value = cpu.registers.get_byte(source);
        cpu.registers.set_byte(target, value);
        self.cycles
    }

    fn ld_r8_a16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, target: CpuRegIndex, source_address: u16) -> i16 {
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(target, value);
        self.cycles
    }

    fn ld_r8_Pr16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, target: CpuRegIndex, source: CpuRegIndex) -> i16 {
        let source_address = cpu.registers.get_word(source);
        let value = mmu.read_byte(source_address);
        cpu.registers.set_byte(target, value);
        self.cycles
    }

    fn ld_r16_d16(&mut self, cpu: &mut Cpu, target: CpuRegIndex, args: &[u8]) -> i16 {
        let value = Instruction::read_args_as_word(args[0], args[1]);
        cpu.registers.set_word(target, value);
        self.cycles
    }

    fn ld_r16_r16(&mut self, cpu: &mut Cpu, target: CpuRegIndex, source: CpuRegIndex) -> i16 {
        let value = cpu.registers.get_word(source);
        cpu.registers.set_word(target, value);
        self.cycles
    }

    fn ld_r16_Pr16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, target: CpuRegIndex, source: CpuRegIndex) -> i16 {
        let source_address = cpu.registers.get_word(source);
        let value = mmu.read_word(source_address, Endianness::BIG);
        cpu.registers.set_word(target, value);
        self.cycles
    }

    fn ld_Pr16_d8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, target: CpuRegIndex, value: u8) -> i16 {
        let target_address = cpu.registers.get_word(target);
        mmu.load_byte(target_address, value);
        self.cycles
    }

    fn ld_Pr16_d16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, target: CpuRegIndex, args: &[u8]) -> i16 {
        let target_address = cpu.registers.get_word(target);
        let value = Instruction::read_args_as_word(args[0], args[1]);
        mmu.load_word(target_address, value, Endianness::BIG);
        self.cycles
    }

    fn ld_Pr16_r8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, target: CpuRegIndex, source: CpuRegIndex) -> i16 {
        let target_address = cpu.registers.get_word(target);
        let value = cpu.registers.get_byte(source);
        mmu.load_byte(target_address, value);
        self.cycles
    }

    fn ld_Pr16_r16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, target: CpuRegIndex, source: CpuRegIndex) -> i16 {
        let target_address = cpu.registers.get_word(target);
        let value = cpu.registers.get_word(source);
        mmu.load_word(target_address, value, Endianness::BIG);
        self.cycles
    }

    fn ld_a8_r8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, a8: u8, source: CpuRegIndex) -> i16 {
        let target_address = 0xFF00 + a8 as u16;
        let value = cpu.registers.get_byte(source);
        mmu.load_byte(target_address, value);
        self.cycles
    }

    fn ld_a16_r8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: CpuRegIndex) -> i16 {
        let target_address = Instruction::read_args_as_word(args[0], args[1]);
        let value = cpu.registers.get_byte(source);
        mmu.load_byte(target_address, value);
        self.cycles
    }

    fn ld_a16_r16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: CpuRegIndex) -> i16 {
        let target_address = Instruction::read_args_as_word(args[0], args[1]);
        let value = cpu.registers.get_word(source);
        mmu.load_word(target_address, value, Endianness::BIG);
        self.cycles
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
        let value = cpu.registers.get_byte(register);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::increment_byte(value, original_carry);
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
        let value = cpu.registers.get_byte(register);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::decrement_byte(value, original_carry);
        cpu.registers.set_byte(register, result);
        cpu.registers.set_flags(flags);
    }

    fn decrement_a16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, address: u16) {
        let value = mmu.read_byte(address);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::decrement_byte(value, original_carry);
        mmu.load_byte(address, result);
        cpu.registers.set_flags(flags);
    }

    /// INC/DEC 16 bit -- Does not affect flags
    fn increment_r16(&mut self, cpu: &mut Cpu, register: CpuRegIndex) {
        let r16 = cpu.registers.get_word(register);
        let result = alu::increment_word(r16);
        cpu.registers.set_word(register, result);
    }

    fn decrement_r16(&mut self, cpu: &mut Cpu, register: CpuRegIndex) {
        let r = cpu.registers.get_word(register);
        let result = alu::decrement_word(r);
        cpu.registers.set_word(register, r);
    }

    /// NOP
    /// 1 4
    /// - - - -
    fn op_0000(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cycles
    }

    /// LD BC,d16
    /// 3 12
    /// - - - -
    fn op_0001(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r16_d16(cpu, CpuRegIndex::BC, args)
    }
    
    /// LD (BC),A
    /// 1 8
    /// - - - -
    fn op_0002(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::BC, CpuRegIndex::A)
    }

    /// INC BC
    /// 1 8
    /// - - - -
    fn op_0003(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r16(cpu, CpuRegIndex::BC);
        self.cycles
    }

    /// INC B
    /// 1 4
    /// Z 0 H -
    fn op_0004(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::B);
        self.cycles
    }

    /// DEC B
    /// 1 4
    /// Z 1 H -
    fn op_0005(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::B);
        self.cycles
    }

    /// LD B,d8
    /// 2 8
    /// - - - -
    fn op_0006(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_d8(cpu, CpuRegIndex::B, args[0])
    }

    /// RLCA
    /// 1 4
    /// 0 0 0 C
    fn op_0007(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let value = cpu.registers.get_byte(CpuRegIndex::A);
        let (result, flags) = alu::rotate_left_circular(value);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// LD (a16),SP
    /// 3 20
    /// - - - -
    fn op_0008(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_a16_r16(cpu, mmu, args, CpuRegIndex::SP)
    }

    /// ADD HL,BC
    /// 1 8
    /// - 0 H C
    fn op_0009(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_r16_r16(cpu, CpuRegIndex::HL, CpuRegIndex::BC);
        self.cycles
    }

    /// LD A,(BC)
    /// 1  8
    /// - - - -
    fn op_000a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::A, CpuRegIndex::BC)
    }

    /// DEC BC 
    /// 1 8 
    /// - - - -
    fn op_000b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r16(cpu, CpuRegIndex::BC);
        self.cycles
    }
    
    /// INC C
    /// 1 4
    /// Z 0 H -
    fn op_000c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::C);
        self.cycles
    }

    /// DEC C
    /// 1 4
    /// Z 1 H -
    fn op_000d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::C);
        self.cycles
    }

    /// LD C,d8
    /// 2 8
    /// - - - -
    fn op_000e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_d8(cpu, CpuRegIndex::C, args[0])
    }

    /// LD DE,d16
    /// 3 12
    /// - - - -
    fn op_0011(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r16_d16(cpu, CpuRegIndex::DE, args)
    }

    /// LD (DE),A
    /// 1 8
    /// - - - -
    fn op_0012(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::DE, CpuRegIndex::A)
    }

    /// INC DE
    /// 1 8
    /// - - - -
    fn op_0013(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r16(cpu, CpuRegIndex::DE);
        self.cycles
    }

    /// INC D
    /// 1 4
    /// Z 0 H -
    fn op_0014(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::D);
        self.cycles
    }

    /// DEC D
    /// 1 4
    /// Z 1 H -
    fn op_0015(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::D);
        self.cycles
    }

    /// LD D,d8
    /// 2 8
    /// - - - -
    fn op_0016(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_d8(cpu,CpuRegIndex::D, args[0])
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
        self.cycles
    }

    /// JR r8
    /// 2 12
    /// - - - -
    fn op_0018(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.relative_jump(cpu, args[0]);
        self.cycles
    }

    /// ADD HL,DE
    /// 1 8
    /// - 0 H C
    fn op_0019(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_r16_r16(cpu, CpuRegIndex::HL, CpuRegIndex::DE);
        self.cycles
    }

    /// LD A,(DE)
    /// 1 8
    /// - - - -
    fn op_001a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::A, CpuRegIndex::DE)
    }

    /// DEC DE
    /// 1 8
    /// - - - -
    fn op_001b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r16(cpu, CpuRegIndex::DE);
        self.cycles
    }

    /// INC E
    /// 1 4
    /// Z 0 H -
    fn op_001c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::E);
        self.cycles
    }

    /// DEC E
    /// 1 4
    /// Z 1 H -
    fn op_001d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::E);
        self.cycles
    }

    /// LD E,d8
    /// 2 8
    /// - - - -
    fn op_001e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_d8(cpu,CpuRegIndex::E, args[0])
    }

    /// JR NZ,r8
    /// 2 12/8
    /// - - - -
    fn op_0020(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        // Weird behavior when running tetris at pc=0x27A1. arg1 should be 0xF8 but changes 0x2F,
        // which jumps ahead to pc=0x27D0 instead of back to pc=0x279B:
        // 0x27A1	0x0020	JR NZ,r8	0x2F
        if !(cpu.registers.get_flags().zero) {
            self.relative_jump(cpu, args[0]);
        }
        self.cycles
    }

    /// LD HL,d16
    /// 3 12
    /// - - - -
    fn op_0021(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r16_d16(cpu, CpuRegIndex::HL, args)
    }

    /// LDI (HL),A
    /// aka LD (HL+),A
    /// 1 8
    /// - - - -
    fn op_0022(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::HL, CpuRegIndex::A);
        cpu.registers.increment(CpuRegIndex::HL, 1);
        self.cycles
    }

    /// INC HL
    /// 1 8
    /// - - - -
    fn op_0023(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r16(cpu, CpuRegIndex::HL);
        self.cycles
    }

    /// INC H
    /// 1 4
    /// Z 0 H -
    fn op_0024(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::H);
        self.cycles
    }

    /// DEC H
    /// 1 4
    /// Z 1 H -
    fn op_0025(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::H);
        self.cycles
    }

    /// LD H,d8
    /// 2 8
    /// - - - -
    fn op_0026(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_d8(cpu,CpuRegIndex::H, args[0])
    }

    /// JR Z,r8
    /// 2 12/8
    /// - - - -
    fn op_0028(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if cpu.registers.get_flags().zero {
            self.relative_jump(cpu, args[0]);
        }
        self.cycles
    }

    /// ADD HL,HL
    /// 1 8
    /// - 0 H C
    fn op_0029(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_r16_r16(cpu, CpuRegIndex::HL, CpuRegIndex::HL);
        self.cycles
    }

    /// LDI A,(HL)
    /// aka LD A, (HL+)
    /// 1 8
    /// - - - -
    fn op_002a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::A, CpuRegIndex::HL);
        cpu.registers.increment(CpuRegIndex::HL, 1);
        self.cycles
    }

    /// DEC HL
    /// 1 8
    /// - - - -
    fn op_002b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r16(cpu, CpuRegIndex::HL);
        self.cycles
    }

    /// INC L
    /// 1 4
    /// Z 0 H -
    fn op_002c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::L);
        self.cycles
    }

    /// DEC L
    /// 1 4
    /// Z 1 H -
    fn op_002d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::L);
        self.cycles
    }

    /// LD L,d8
    /// 2 8
    /// - - - -
    fn op_002e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_d8(cpu,CpuRegIndex::L, args[0])
    }

    /// CPL
    /// 1 4
    /// - 1 1 -
    /// Gives the one's complement of A, i. e. all the bits of A are reversed individually.
    fn op_002f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        cpu.registers.set_byte(CpuRegIndex::A, !a);
        let flags = cpu.registers.get_flags();
        cpu.registers.set_flags(Flags {
            zero: flags.zero,
            subtract: true,
            half_carry: true,
            carry: flags.carry,
        });
        self.cycles
    }

    /// JR NC,r8
    /// 2 8/12
    /// - - - -
    fn op_0030(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !(cpu.registers.get_flags().carry) {
            self.relative_jump(cpu, args[0]);
        }
        self.cycles
    }

    /// LD SP,d16
    /// 3 12
    /// - - - -
    fn op_0031(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r16_d16(cpu, CpuRegIndex::SP, args)
    }

    /// LDD (HL),A
    /// aka LD (HL-),A
    /// 1 8
    /// - - - -
    fn op_0032(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::HL, CpuRegIndex::A);
        cpu.registers.decrement(CpuRegIndex::HL, 1);
        self.cycles
    }

    /// INC SP
    /// 1 8
    /// - - - -
    fn op_0033(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r16(cpu, CpuRegIndex::SP);
        self.cycles
    }

    /// INC (HL)
    /// 1 12
    /// Z 0 H -
    fn op_0034(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = cpu.registers.get_word(CpuRegIndex::HL);
        self.increment_a16(cpu, mmu, address);
        self.cycles
    }

    /// DEC (HL)
    /// 1 12
    /// Z 1 H -
    fn op_0035(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = cpu.registers.get_word(CpuRegIndex::HL);
        self.decrement_a16(cpu, mmu, address);
        self.cycles
    }

    /// LD (HL),d8
    /// 2 12
    /// - - - -
    fn op_0036(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_d8(cpu, mmu, CpuRegIndex::HL, args[0])
    }

    /// SCF
    /// 1 4
    /// - 0 0 1
    // TODO

    /// JR C,r8
    /// 2 8/12
    /// - - - -
    fn op_0038(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if cpu.registers.get_flags().carry {
            self.relative_jump(cpu, args[0]);
        }
        self.cycles
    }

    /// ADD HL,SP
    /// 1 8
    /// - 0 H C
    fn op_0039(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_r16_r16(cpu, CpuRegIndex::HL, CpuRegIndex::SP);
        self.cycles
    }

    /// LDD A,(HL)
    /// aka LD A,(HL-)
    /// 1 8
    /// - - - -
     fn op_003a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::A, CpuRegIndex::HL);
        cpu.registers.decrement(CpuRegIndex::HL, 1);
        self.cycles
    }

    /// DEC SP
    /// 1 8
    /// - - - -
    fn op_003b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r16(cpu, CpuRegIndex::SP);
        self.cycles
    }

    /// INC A
    /// 1 4
    /// Z 0 H -
    fn op_003c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_r8(cpu, CpuRegIndex::A);
        self.cycles
    }

    /// DEC A
    /// 1 4
    /// Z 1 H -
    fn op_003d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_r8(cpu, CpuRegIndex::A);
        self.cycles
    }

    /// LD A,d8
    /// 2 8
    /// - - - -
    fn op_003e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_d8(cpu,CpuRegIndex::A, args[0])
    }

    /// LD B,B
    /// 1 4
    /// - - - -
    fn op_0040(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::B, CpuRegIndex::B)
    }

    /// LD B,C
    /// 1 4
    /// - - - -
    fn op_0041(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::B, CpuRegIndex::C)
    }

    /// LD B,D
    /// 1 4
    /// - - - -
    fn op_0042(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::B, CpuRegIndex::D)
    }

    /// LD B,E
    /// 1 4
    /// - - - -
    fn op_0043(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::B, CpuRegIndex::E)
    }

    /// LD B,H
    /// 1 4
    /// - - - -
    fn op_0044(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::B, CpuRegIndex::H)
    }

    /// LD B,L
    /// 1 4
    /// - - - -
    fn op_0045(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::B, CpuRegIndex::L)
    }

    /// LD B,(HL)
    /// 1 8
    /// - - - -
    fn op_0046(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::B, CpuRegIndex::HL)
    }

    /// LD B,A
    /// 1 4
    /// - - - -
    fn op_0047(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::B, CpuRegIndex::A)
    }

    /// LD C,B
    /// 1 4
    /// - - - -
    fn op_0048(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::C, CpuRegIndex::B)
    }

    /// LD C,C
    /// 1 4
    /// - - - -
    fn op_0049(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::C, CpuRegIndex::C)
    }

    /// LD C,D
    /// 1 4
    /// - - - -
    fn op_004a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::C, CpuRegIndex::D)
    }

    /// LD C,E
    /// 1 4
    /// - - - -
    fn op_004b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::C, CpuRegIndex::E)
    }

    /// LD C,H
    /// 1 4
    /// - - - -
    fn op_004c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::C, CpuRegIndex::H)
    }

    /// LD C,L
    /// 1 4
    /// - - - -
    fn op_004d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::C, CpuRegIndex::L)
    }

    /// LD C,(HL)
    /// 1 8
    /// - - - -
    fn op_004e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::C, CpuRegIndex::HL)
    }

    /// LD C,A
    /// 1 4
    /// - - - -
    fn op_004f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::C, CpuRegIndex::A)
    }

    /// LD D,B
    /// 1 4
    /// - - - -
    fn op_0050(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::D, CpuRegIndex::B)
    }

    /// LD D,C
    /// 1 4
    /// - - - -
    fn op_0051(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::D, CpuRegIndex::C)
    }

    /// LD D,D
    /// 1 4
    /// - - - -
    fn op_0052(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::D, CpuRegIndex::D)
    }

    /// LD D,E
    /// 1 4
    /// - - - -
    fn op_0053(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::D, CpuRegIndex::E)
    }

    /// LD D,H
    /// 1 4
    /// - - - -
    fn op_0054(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::D, CpuRegIndex::H)
    }

    /// LD D,L
    /// 1 4
    /// - - - -
    fn op_0055(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::D, CpuRegIndex::L)
    }

    /// LD D,(HL)
    /// 1 8
    /// - - - -
    fn op_0056(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::D, CpuRegIndex::HL)
    }

    /// LD D,A
    /// 1 4
    /// - - - -
    fn op_0057(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::D, CpuRegIndex::A)
    }

    /// LD E,B
    /// 1 4
    /// - - - -
    fn op_0058(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::E, CpuRegIndex::B)
    }

    /// LD E,C
    /// 1 4
    /// - - - -
    fn op_0059(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::E, CpuRegIndex::C)
    }

    /// LD E,D
    /// 1 4
    /// - - - -
    fn op_005a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::E, CpuRegIndex::D)
    }

    /// LD E,E
    /// 1 4
    /// - - - -
    fn op_005b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::E, CpuRegIndex::E)
    }

    /// LD E,H
    /// 1 4
    /// - - - -
    fn op_005c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::E, CpuRegIndex::H)
    }

    /// LD E,L
    /// 1 4
    /// - - - -
    fn op_005d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::E, CpuRegIndex::L)
    }

    /// LD E,(HL)
    /// 1 8
    /// - - - -
    fn op_005e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::E, CpuRegIndex::HL)
    }

    /// LD E,A
    /// 1 4
    /// - - - -
    fn op_005f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::E, CpuRegIndex::A)
    }

    /// LD H,B
    /// 1 4
    /// - - - -
    fn op_0060(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::H, CpuRegIndex::B)
    }

    /// LD H,C
    /// 1 4
    /// - - - -
    fn op_0061(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::H, CpuRegIndex::C)
    }

    /// LD H,D
    /// 1 4
    /// - - - -
    fn op_0062(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::H, CpuRegIndex::D)
    }

    /// LD H,E
    /// 1 4
    /// - - - -
    fn op_0063(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::H, CpuRegIndex::E)
    }

    /// LD H,H
    /// 1 4
    /// - - - -
    fn op_0064(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::H, CpuRegIndex::H)
    }

    /// LD H,L
    /// 1 4
    /// - - - -
    fn op_0065(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::H, CpuRegIndex::L)
    }

    /// LD H,(HL)
    /// 1 8
    /// - - - -
    fn op_0066(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::H, CpuRegIndex::HL)
    }

    /// LD H,A
    /// 1 4
    /// - - - -
    fn op_0067(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::H, CpuRegIndex::A)
    }

    /// LD L,B
    /// 1 4
    /// - - - -
    fn op_0068(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::L, CpuRegIndex::B)
    }

    /// LD L,C
    /// 1 4
    /// - - - -
    fn op_0069(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::L, CpuRegIndex::C)
    }

    /// LD L,D
    /// 1 4
    /// - - - -
    fn op_006a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::L, CpuRegIndex::D)
    }

    /// LD L,E
    /// 1 4
    /// - - - -
    fn op_006b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::L, CpuRegIndex::E)
    }

    /// LD L,H
    /// 1 4
    /// - - - -
    fn op_006c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::L, CpuRegIndex::H)
    }

    /// LD L,L
    /// 1 4
    /// - - - -
    fn op_006d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::L, CpuRegIndex::L)
    }

    /// LD L,(HL)
    /// 1 8
    /// - - - -
    fn op_006e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu, CpuRegIndex::L, CpuRegIndex::HL)
    }

    /// LD L,A
    /// 1 4
    /// - - - -
    fn op_006f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu, CpuRegIndex::L, CpuRegIndex::A)
    }

    /// LD (HL),B
    /// 1 8
    /// - - - -
    fn op_0070(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::HL, CpuRegIndex::B)
    }

    /// LD (HL),C
    /// 1 8
    /// - - - -
    fn op_0071(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::HL, CpuRegIndex::C)
    }

    /// LD (HL),D
    /// 1 8
    /// - - - -
    fn op_0072(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::HL, CpuRegIndex::D)
    }

    /// LD (HL),E
    /// 1 8
    /// - - - -
    fn op_0073(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::HL, CpuRegIndex::E)
    }

    /// LD (HL),H
    /// 1 8
    /// - - - -
    fn op_0074(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::HL, CpuRegIndex::H)
    }

    /// LD (HL),L
    /// 1 8
    /// - - - -
    fn op_0075(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::HL, CpuRegIndex::L)
    }

    /// HALT

    /// LD (HL),A
    /// 1 8
    /// - - - -
    fn op_0077(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_Pr16_r8(cpu, mmu, CpuRegIndex::HL, CpuRegIndex::A)
    }

    /// LD A,B
    /// 1 4
    /// - - - -
    fn op_0078(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu,  CpuRegIndex::A, CpuRegIndex::B)
    }

    /// LD A,C
    /// 1 4
    /// - - - -
    fn op_0079(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu,  CpuRegIndex::A, CpuRegIndex::C)
    }

    /// LD A,D
    /// 1 4
    /// - - - -
    fn op_007a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu,  CpuRegIndex::A, CpuRegIndex::D)
    }

    /// LD A,E
    /// 1 4
    /// - - - -
    fn op_007b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu,  CpuRegIndex::A, CpuRegIndex::E)
    }

    /// LD A,H
    /// 1 4
    /// - - - -
    fn op_007c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu,  CpuRegIndex::A, CpuRegIndex::H)
    }

    /// LD A,L
    /// 1 4
    /// - - - -
    fn op_007d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu,  CpuRegIndex::A, CpuRegIndex::L)
    }

    /// LD A,(HL)
    /// 1 8
    /// - - - -
    fn op_007e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_Pr16(cpu, mmu,  CpuRegIndex::A, CpuRegIndex::HL)
    }

    /// LD A,A
    /// 1 4
    /// - - - -
    fn op_007f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_r8_r8(cpu,  CpuRegIndex::A, CpuRegIndex::A)
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
        self.cycles
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
        self.cycles
    }

    /// AND E
    /// 1 4
    /// Z 0 1 0
    fn op_00a3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let e = cpu.registers.get_byte(CpuRegIndex::E);
        let (result, flags) = alu::and_byte(a, e);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// XOR A
    /// 1 4
    /// Z 0 0 0
    fn op_00af(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let (result, flags) = alu::xor_byte(a, a);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// OR C
    /// 1 4
    /// Z 0 0 0
    fn op_00b1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let c = cpu.registers.get_byte(CpuRegIndex::C);
        let (result, flags) = alu::or_byte(a, c);
        cpu.registers.set_byte(CpuRegIndex::A, a);
        cpu.registers.set_flags(flags);
        self.cycles
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
        self.cycles
    }

    /// POP BC
    /// 1 12
    /// - - - -
    fn op_00c1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.pop(cpu, mmu, CpuRegIndex::BC);
        self.cycles
    }

    /// JP a16
    /// 3 16
    /// - - - -
    fn op_00c3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = Instruction::read_args_as_word(args[0], args[1]);
        self.jump(cpu, address);
        self.cycles
    }

    /// PUSH BC
    /// 1 16
    /// - - - -
    fn op_00c5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.push(cpu, mmu, CpuRegIndex::BC);
        self.cycles
    }

    /// RET
    /// 1 16
    /// - - - -
    fn op_00c9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.pop(cpu, mmu, CpuRegIndex::PC);
        self.cycles
    }

    /// CALL a16
    /// 3 24
    /// - - - -
    fn op_00cd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = Instruction::read_args_as_word(args[0], args[1]);
        self.call(cpu, mmu, address);
        self.cycles
    }

    /// PUSH DE
    /// 1 16
    /// - - - -
    fn op_00d5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.push(cpu, mmu, CpuRegIndex::DE);
        self.cycles
    }

    /// LDH (a8),A
    /// aka LD ($FF00+a8),A
    /// 2 12
    /// - - - -
    fn op_00e0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_a8_r8(cpu, mmu, args[0], CpuRegIndex::A)
    }

    /// POP HL
    /// 1 12
    /// - - - -
    fn op_00e1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.pop(cpu, mmu, CpuRegIndex::HL);
        self.cycles
    }

    /// LDH (C),A
    /// aka LD ($FF00+C),A
    /// 1 8
    /// - - - -
    fn op_00e2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a8 = cpu.registers.get_byte(CpuRegIndex::C);
        self.ld_a8_r8(cpu, mmu, a8, CpuRegIndex::A)
    }

    /// PUSH HL
    /// 1 16
    /// - - - -
    fn op_00e5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.push(cpu, mmu, CpuRegIndex::HL);
        self.cycles
    }

    /// LD (a16),A 
    /// 3 16 
    /// - - - -
    fn op_00ea(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_a16_r8(cpu, mmu, args, CpuRegIndex::A)
    }

    /// LDH A,(a8)
    /// aka LD A,($FF00+a8)
    /// 2 12
    /// - - - -
    fn op_00f0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a16 = 0xFF00 + args[0] as u16;
        self.ld_r8_a16(cpu, mmu, CpuRegIndex::A, a16)
    }

    /// POP AF
    /// 1 12
    /// Z N H C
    fn op_00f1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.pop(cpu, mmu, CpuRegIndex::AF);
        let flags = Flags {
            zero: true,
            subtract: true,
            half_carry: true,
            carry: true,
        };
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// LD A,($FF00+C)
    /// 1 8
    /// - - - -
    fn op_00f2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a16 = 0xFF00 + (cpu.registers.get_byte(CpuRegIndex::C) as u16);
        self.ld_r8_a16(cpu, mmu, CpuRegIndex::A, a16)
    }

    /// DI
    /// 1 4
    /// - - - -
    fn op_00f3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.interrupts.set_ime(false, mmu);
        self.cycles
    }

    /// PUSH AF
    /// 1 16
    /// - - - -
    fn op_00f5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.push(cpu, mmu, CpuRegIndex::AF);
        self.cycles
    }

    /// LD A,(a16)
    /// 3 16
    /// - - - -
    fn op_00fa(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a16 = Instruction::read_args_as_word(args[0], args[1]);
        self.ld_r8_a16(cpu, mmu, CpuRegIndex::A, a16)
    }

    /// CP d8
    /// 2 8
    /// Z 1 H C
    fn op_00fe(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let (result, flags) = alu::subtract_byte(a, args[0]);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// RST 38H
    /// 1 16
    /// - - - -
    fn op_00ff(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = 0x38;
        self.call(cpu, mmu, address);
        self.cycles
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
        self.cycles
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
        self.cycles
    }
}
