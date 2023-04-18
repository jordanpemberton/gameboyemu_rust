#![allow(unused_variables)]

use sdl2::log::set_output_function;
use crate::console::alu;
use crate::console::cpu::{Cpu, PREFIX_BYTE};
use crate::console::input::CallbackAction::DEBUG;
use crate::console::mmu::{Endianness, Mmu};
use crate::console::cpu_registers::{Flags, CpuRegIndex};

#[derive(Clone, Copy, Debug)]
enum Src {
    D8, D16,
    A8, A16,
    A, B, C, D, E, F, H, L,
    AF, BC, DE, HL, PC, SP,
    Ca,
    AFa, BCa, DEa, HLa,
}

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
            // 0x000F

            // 0x0010
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
            // 0x001F

            0x0020 => Instruction { opcode, mnemonic: "JR NZ,r8", size: 2, cycles: 8, _fn: Instruction::op_0020 },
            0x0021 => Instruction { opcode, mnemonic: "LD HL,d16", size: 3, cycles: 12, _fn: Instruction::op_0021 },
            0x0022 => Instruction { opcode, mnemonic: "LDI (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0022 },
            0x0023 => Instruction { opcode, mnemonic: "INC HL", size: 1, cycles: 8, _fn: Instruction::op_0023 },
            0x0024 => Instruction { opcode, mnemonic: "INC H", size: 1, cycles: 4, _fn: Instruction::op_0024 },
            0x0025 => Instruction { opcode, mnemonic: "DEC H", size: 1, cycles: 4, _fn: Instruction::op_0025 },
            0x0026 => Instruction { opcode, mnemonic: "LD H,d8", size: 2, cycles: 8, _fn: Instruction::op_0026 },
            // 0x0027
            0x0028 => Instruction { opcode, mnemonic: "JR Z,r8", size: 2, cycles: 8, _fn: Instruction::op_0028 },
            0x002A => Instruction { opcode, mnemonic: "LDI A,(HL)", size: 1, cycles: 8, _fn: Instruction::op_002a },
            0x002B => Instruction { opcode, mnemonic: "DEC HL", size: 1, cycles: 8, _fn: Instruction::op_002b },
            0x002C => Instruction { opcode, mnemonic: "INC L", size: 1, cycles: 4, _fn: Instruction::op_002c },
            0x002D => Instruction { opcode, mnemonic: "DEC L", size: 1, cycles: 4, _fn: Instruction::op_002d },
            0x002E => Instruction { opcode, mnemonic: "LD L,d8", size: 2, cycles: 8, _fn: Instruction::op_002e },
            0x002F => Instruction { opcode, mnemonic: "CPL", size: 1, cycles: 4, _fn: Instruction::op_002f },

            0x0030 => Instruction { opcode, mnemonic: "JR NC,r8", size: 2, cycles: 8, _fn: Instruction::op_0030 },
            0x0031 => Instruction { opcode, mnemonic: "LD SP,d16", size: 3, cycles: 12, _fn: Instruction::op_0031 },
            0x0032 => Instruction { opcode, mnemonic: "LDD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0032 },
            0x0033 => Instruction{ opcode, mnemonic: "INC SP", size: 1, cycles: 8, _fn: Instruction::op_0033 },
            0x0034 => Instruction{ opcode, mnemonic: "INC (HL)", size: 1, cycles: 12, _fn: Instruction::op_0034 },
            0x0035 => Instruction { opcode, mnemonic: "DEC (HL)", size: 1, cycles: 12, _fn: Instruction::op_0035 },
            0x0036 => Instruction { opcode, mnemonic: "LD (HL),d8", size: 2, cycles: 12, _fn: Instruction::op_0036 },
            // 0x0037
            // 0x0038
            // 0x0039
            0x003A => Instruction { opcode, mnemonic: "LDD A,(HL)", size: 1, cycles: 8, _fn: Instruction::op_003a },
            0x003B => Instruction { opcode, mnemonic: "DEC SP", size: 1, cycles: 8, _fn: Instruction::op_003b },
            0x003C => Instruction { opcode, mnemonic: "INC A", size: 1, cycles: 4, _fn: Instruction::op_003c },
            0x003D => Instruction { opcode, mnemonic: "DEC A", size: 1, cycles: 4, _fn: Instruction::op_003d },
            0x003E => Instruction { opcode, mnemonic: "LD A,d8", size: 2, cycles: 8, _fn: Instruction::op_003e },
            // 0x003F

            // 0x0040
            // 0x0041
            // 0x0042
            // 0x0043
            // 0x0044
            // 0x0045
            // 0x0046
            0x0047 => Instruction { opcode, mnemonic: "LD B,A", size: 1, cycles: 4, _fn: Instruction::op_0047 },
            // 0x0048
            // 0x0049
            // 0x004A
            // 0x004B
            // 0x004C
            // 0x004D
            // 0x004E
            0x004F => Instruction { opcode, mnemonic: "LD C,A", size: 1, cycles: 4, _fn: Instruction::op_004f },

            // 0x0050
            // 0x0051
            // 0x0052
            // 0x0053
            // 0x0054
            // 0x0055
            // 0x0056
            0x0057 => Instruction { opcode, mnemonic: "LD D,A", size: 1, cycles: 4, _fn: Instruction::op_0057 },
            // 0x0058
            // 0x0059
            // 0x005A
            0x005B => Instruction { opcode, mnemonic: "LD E,E", size: 1, cycles: 4, _fn: Instruction::op_005b },
            // 0x005C
            // 0x005D
            // 0x005E
            // 0x005F

            // 0x0060
            // 0x0061
            // 0x0062
            // 0x0063
            // 0x0064
            // 0x0065
            // 0x0066
            0x0067 => Instruction { opcode, mnemonic: "LD H,A", size: 1, cycles: 4, _fn: Instruction::op_0067 },
            // 0x0068
            // 0x0069
            // 0x006A
            // 0x006B
            // 0x006C
            // 0x006D
            // 0x006E
            // 0x006F

            // 0x0070
            // 0x0071
            // 0x0072
            // 0x0073
            // 0x0074
            // 0x0075
            // 0x0076
            0x0077 => Instruction { opcode, mnemonic: "LD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0077 },
            0x0078 => Instruction { opcode, mnemonic: "LD A,B", size: 1, cycles: 4, _fn: Instruction::op_0078 },
            0x0079 => Instruction { opcode, mnemonic: "LD A,C", size: 1, cycles: 4, _fn: Instruction::op_0079 },
            // 0x007A
            0x007B => Instruction { opcode, mnemonic: "LD A,E", size: 1, cycles: 4, _fn: Instruction::op_007b },
            0x007C => Instruction { opcode, mnemonic: "LD A,H", size: 1, cycles: 4, _fn: Instruction::op_007c },
            0x007D => Instruction { opcode, mnemonic: "LD A,L", size: 1, cycles: 4, _fn: Instruction::op_007d },
            // 0x007E
            // 0x007F

            // 0x0080
            // 0x0081
            // 0x0082
            // 0x0083
            // 0x0084
            // 0x0085
            0x0086 => Instruction { opcode, mnemonic: "ADD (HL)", size: 1, cycles: 8, _fn: Instruction::op_0086 },
            // 0x0087
            // 0x0088
            // 0x0089
            // 0x008A
            // 0x008B
            // 0x008C
            // 0x008D
            // 0x008E
            // 0x008F

            0x0090 => Instruction { opcode, mnemonic: "SUB B", size: 1, cycles: 4, _fn: Instruction::op_0090 },
            0x0091 => Instruction { opcode, mnemonic: "SUB C", size: 1, cycles: 4, _fn: Instruction::op_0091 },
            0x0092 => Instruction { opcode, mnemonic: "SUB D", size: 1, cycles: 4, _fn: Instruction::op_0092 },
            0x0093 => Instruction { opcode, mnemonic: "SUB E", size: 1, cycles: 4, _fn: Instruction::op_0093 },
            0x0094 => Instruction { opcode, mnemonic: "SUB H", size: 1, cycles: 4, _fn: Instruction::op_0094 },
            0x0095 => Instruction { opcode, mnemonic: "SUB L", size: 1, cycles: 4, _fn: Instruction::op_0095 },
            0x0096 => Instruction { opcode, mnemonic: "SUB (HL)", size: 1, cycles: 8, _fn: Instruction::op_0096 },
            0x0097 => Instruction { opcode, mnemonic: "SUB A", size: 1, cycles: 4, _fn: Instruction::op_0097 },
            // 0x0091
            // 0x0092
            // 0x0093
            // 0x0094
            // 0x0095
            // 0x0096
            // 0x0097
            // 0x0098
            // 0x0099
            // 0x009A
            // 0x009B
            // 0x009C
            // 0x009D
            // 0x009E
            // 0x009F

            0x00A0 => Instruction { opcode, mnemonic: "AND B", size: 1, cycles: 4, _fn: Instruction::op_00a0 },
            0x00A1 => Instruction { opcode, mnemonic: "AND C", size: 1, cycles: 4, _fn: Instruction::op_00a1 },
            0x00A2 => Instruction { opcode, mnemonic: "AND D", size: 1, cycles: 4, _fn: Instruction::op_00a2 },
            0x00A3 => Instruction { opcode, mnemonic: "AND E", size: 1, cycles: 4, _fn: Instruction::op_00a3 },
            0x00A4 => Instruction { opcode, mnemonic: "AND H", size: 1, cycles: 4, _fn: Instruction::op_00a4 },
            0x00A5 => Instruction { opcode, mnemonic: "AND L", size: 1, cycles: 4, _fn: Instruction::op_00a5 },
            0x00A6 => Instruction { opcode, mnemonic: "AND (HL)", size: 1, cycles: 4, _fn: Instruction::op_00a6 },
            0x00A7 => Instruction { opcode, mnemonic: "AND A", size: 1, cycles: 8, _fn: Instruction::op_00a7 },
            // 0x00A8
            // 0x00A9
            // 0x00AA
            // 0x00AB
            // 0x00AC
            // 0x00AD
            // 0x00AE
            0x00AF => Instruction { opcode, mnemonic: "XOR A", size: 1, cycles: 4, _fn: Instruction::op_00af },

            // 0x00B0
            0x00B1 => Instruction { opcode, mnemonic: "OR C", size: 1, cycles: 4, _fn: Instruction::op_00b1 },
            // 0x00B2
            // 0x00B3
            // 0x00B4
            // 0x00B5
            // 0x00B6
            // 0x00B7
            // 0x00B8
            // 0x00B9
            // 0x00BA
            // 0x00BB
            // 0x00BC
            // 0x00BD
            0x00BE => Instruction { opcode, mnemonic: "CP (HL)", size: 1, cycles: 8, _fn: Instruction::op_00be },
            // 0x00BF

            // 0x00C0 => RET NZ
            0x00C1 => Instruction { opcode, mnemonic: "POP BC", size: 1, cycles: 12, _fn: Instruction::op_00c1 },
            // 0x00C2
            0x00C3 => Instruction { opcode, mnemonic: "JP a16", size: 3, cycles: 16, _fn: Instruction::op_00c3 },
            0x00C4 => Instruction { opcode, mnemonic: "CALL NZ,a16", size: 3, cycles: 12, _fn: Instruction::op_00c4 },
            0x00C5 => Instruction { opcode, mnemonic: "PUSH BC", size: 1, cycles: 16, _fn: Instruction::op_00c5 },
            // 0x00C6
            // 0x00C7
            // 0x00C8
            0x00C9 => Instruction { opcode, mnemonic: "RET", size: 1, cycles: 16, _fn: Instruction::op_00c9 },
            // 0x00CA
            // 0x00CB
            // 0x00CC
            0x00CD => Instruction { opcode, mnemonic: "CALL a16", size: 3, cycles: 24, _fn: Instruction::op_00cd },
            0x00CE => Instruction { opcode, mnemonic: "ADC A,d8", size: 2, cycles: 8, _fn: Instruction::op_00ce },
            // 0x00CF

            // 0x00D0 => RET NC
            // 0x00D1 => Instruction { opcode, mnemonic: "POP DE", size: 1, cycles: 12, _fn: Instruction::op_00d1 },
            // 0x00D2 => JP NC,a16
            0x00D3 => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            // 0x00D4 => CALL NC,a16
            0x00D5 => Instruction { opcode, mnemonic: "PUSH DE", size: 1, cycles: 16, _fn: Instruction::op_00d5 },
            0x00D6 => Instruction { opcode, mnemonic: "SUB d8", size: 2, cycles: 8, _fn: Instruction::op_00d6 },
            // 0x00D7 => RST 10H
            // 0x00D8 => RET C
            // 0x00D9 => RETI
            // 0x00DA => JP C,a16
            0x00DB => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            // 0x00DC => CALL C,a16
            0x00DD => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            // 0x00DE => SBC A,d8
            // 0x00DF => RST 18H

            0x00E0 => Instruction { opcode, mnemonic: "LDH (a8),A", size: 2, cycles: 12, _fn: Instruction::op_00e0 },
            0x00E1 => Instruction { opcode, mnemonic: "POP HL", size: 1, cycles: 12, _fn: Instruction::op_00e1 },
            0x00E2 => Instruction { opcode, mnemonic: "LDH (C),A", size: 1, cycles: 8, _fn: Instruction::op_00e2 },
            0x00E3 => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00E4 => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00E5 => Instruction { opcode, mnemonic: "PUSH HL", size: 1, cycles: 16, _fn: Instruction::op_00e5 },
            0x00E6 => Instruction { opcode, mnemonic: "AND d8", size: 2, cycles: 8, _fn: Instruction::op_00e6 },
            // 0x00E7 => RST 20H
            // 0x00E8 => Instruction { opcode, mnemonic: "ADD SP,r8", size: 2, cycles: 16, _fn: Instruction::op_00e8 },
            // 0x00E9 => JP HL
            0x00EA => Instruction { opcode, mnemonic: "LD (a16),A", size: 3, cycles: 16, _fn: Instruction::op_00ea },
            0x00EB => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00EC => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00ED => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            // 0x00EE => XOR d8
            // 0x00EF => RST 28H

            0x00F0 => Instruction { opcode, mnemonic: "LDH A,(a8)", size: 2, cycles: 12, _fn: Instruction::op_00f0 },
            0x00F1 => Instruction { opcode, mnemonic: "POP AF", size: 1, cycles: 12, _fn: Instruction::op_00f1 },
            0x00F2 => Instruction { opcode, mnemonic: "LD A,($FF00+C)", size: 1, cycles: 8, _fn: Instruction::op_00f2 },
            0x00F3 => Instruction { opcode, mnemonic: "DI", size: 1, cycles: 4, _fn: Instruction::op_00f3 },
            // 0x00F4 => Invalid
            0x00F5 => Instruction { opcode, mnemonic: "PUSH AF", size: 1, cycles: 16, _fn: Instruction::op_00f5 },
            // 0x00F6 => OR d8
            // 0x00F7 => RST 30H
            // 0x00F8 => Instruction { opcode, mnemonic: "LD HL,SP+r8", size: 2, cycles: 12, _fn: Instruction::op_00f8 },
            // 0x00F9 => Instruction { opcode, mnemonic: "LD SP,HL", size: 1, cycles: 8, _fn: Instruction::op_00f9 },
            0x00FA => Instruction { opcode, mnemonic: "LD A,(a16)", size: 3, cycles: 16, _fn: Instruction::op_00fa },
            // 0x00FB => EI
            0x00FC => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00FD => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00FE => Instruction { opcode, mnemonic: "CP d8", size: 2, cycles: 8, _fn: Instruction::op_00fe },
            0x00FF => Instruction { opcode, mnemonic: "RST 38H", size: 1, cycles: 16, _fn: Instruction::op_00ff },

            // ================================= CB PREFIXED =======================================

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

    fn invalid(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        println!("Opcode {:#06X} (\"{}\") is invalid.", self.opcode, self.mnemonic);
        self.cycles
    }

    fn get_source_value(cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src) -> u16 {
        match source {
            Src::D8 => args[0] as u16,
            Src::D16 => ((args[1] as u16) << 8) | (args[0] as u16),

            Src::A8 => mmu.read_byte(0xFF00 | args[0] as u16) as u16,
            Src::A16 => mmu.read_byte(((args[1] as u16) << 8) | (args[0] as u16)) as u16,

            Src::A => cpu.registers.get_byte(CpuRegIndex::A) as u16,
            Src::B => cpu.registers.get_byte(CpuRegIndex::B) as u16,
            Src::C => cpu.registers.get_byte(CpuRegIndex::C) as u16,
            Src::D => cpu.registers.get_byte(CpuRegIndex::D) as u16,
            Src::E => cpu.registers.get_byte(CpuRegIndex::E) as u16,
            Src::F => cpu.registers.get_byte(CpuRegIndex::F) as u16,
            Src::H => cpu.registers.get_byte(CpuRegIndex::H) as u16,
            Src::L => cpu.registers.get_byte(CpuRegIndex::L) as u16,

            Src::AF => cpu.registers.get_word(CpuRegIndex::AF) as u16,
            Src::BC => cpu.registers.get_word(CpuRegIndex::BC) as u16,
            Src::DE => cpu.registers.get_word(CpuRegIndex::DE) as u16,
            Src::HL => cpu.registers.get_word(CpuRegIndex::HL) as u16,
            Src::PC => cpu.registers.get_word(CpuRegIndex::PC) as u16,
            Src::SP => cpu.registers.get_word(CpuRegIndex::SP) as u16,

            Src::AFa => mmu.read_byte(cpu.registers.get_word(CpuRegIndex::AF)) as u16,
            Src::BCa => mmu.read_byte(cpu.registers.get_word(CpuRegIndex::BC)) as u16,
            Src::DEa => mmu.read_byte(cpu.registers.get_word(CpuRegIndex::DE)) as u16,
            Src::HLa => mmu.read_byte(cpu.registers.get_word(CpuRegIndex::HL)) as u16,

            _ => panic!("Unsupported source {:?}", source),
        }
    }

    fn set_target_value(cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, value: u16) {
        match target {
            Src::A8 => mmu.load_byte(0xFF00 | args[0] as u16, value as u8),
            Src::A16 => mmu.load_word(((args[1] as u16) << 8) | (args[0] as u16), value, Endianness::BIG),

            Src::A => cpu.registers.set_byte(CpuRegIndex::A, value as u8),
            Src::B => cpu.registers.set_byte(CpuRegIndex::B, value as u8),
            Src::C => cpu.registers.set_byte(CpuRegIndex::C, value as u8),
            Src::D => cpu.registers.set_byte(CpuRegIndex::D, value as u8),
            Src::E => cpu.registers.set_byte(CpuRegIndex::E, value as u8),
            Src::F => cpu.registers.set_byte(CpuRegIndex::F, value as u8),
            Src::H => cpu.registers.set_byte(CpuRegIndex::H, value as u8),
            Src::L => cpu.registers.set_byte(CpuRegIndex::L, value as u8),

            Src::AF => cpu.registers.set_word(CpuRegIndex::AF, value),
            Src::BC => cpu.registers.set_word(CpuRegIndex::BC, value),
            Src::DE => cpu.registers.set_word(CpuRegIndex::DE, value),
            Src::HL => cpu.registers.set_word(CpuRegIndex::HL, value),
            Src::PC => cpu.registers.set_word(CpuRegIndex::PC, value),
            Src::SP => cpu.registers.set_word(CpuRegIndex::SP, value),

            Src::Ca => mmu.load_byte(0xFF | cpu.registers.get_byte(CpuRegIndex::C) as u16, value as u8),

            Src::AFa => mmu.load_byte(cpu.registers.get_word(CpuRegIndex::AF), value as u8),
            Src::BCa => mmu.load_byte(cpu.registers.get_word(CpuRegIndex::BC), value as u8),
            Src::DEa => mmu.load_byte(cpu.registers.get_word(CpuRegIndex::DE), value as u8),
            Src::HLa => mmu.load_byte(cpu.registers.get_word(CpuRegIndex::HL), value as u8),

            _ => panic!("Unsupported target {:?}", target),
        }
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
    fn ld(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let value = Instruction::get_source_value(cpu, mmu, args, source);
        Instruction::set_target_value(cpu, mmu, args, target, value);
        self.cycles
    }

    /// ADD
    fn add_8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let a = Instruction::get_source_value(cpu, mmu, args, target) as u8;
        let b = Instruction::get_source_value(cpu, mmu, args, source) as u8;
        let (result, flags) = alu::add_byte(a, b);
        Instruction::set_target_value(cpu, mmu, args, target, result as u16);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    fn add_16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let a = Instruction::get_source_value(cpu, mmu, args, target);
        let b = Instruction::get_source_value(cpu, mmu, args, source);
        let (result, flags) = alu::add_word(a, b);
        Instruction::set_target_value(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// SUB
    fn sub_8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let a = Instruction::get_source_value(cpu, mmu, args, target) as u8;
        let b = Instruction::get_source_value(cpu, mmu, args, source) as u8;
        let (result, flags) = alu::subtract_byte(a, b);
        Instruction::set_target_value(cpu, mmu, args, target, result as u16);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    fn sub_16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let a = Instruction::get_source_value(cpu, mmu, args, target);
        let b = Instruction::get_source_value(cpu, mmu, args, source);
        let (result, flags) = alu::subtract_word(a, b);
        Instruction::set_target_value(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
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

    /// AND
    fn and(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], src: Src) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let b = Instruction::get_source_value(cpu, mmu, args, src) as u8;
        let (result, flags) = alu::and_byte(a, b);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self .cycles
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
        self.ld(cpu, mmu, args, Src::BC, Src::D16)
    }
    
    /// LD (BC),A
    /// 1 8
    /// - - - -
    fn op_0002(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::BCa, Src::A)
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
        self.ld(cpu, mmu, args, Src::B, Src::D8)
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
        self.ld(cpu, mmu, args, Src::A16, Src::SP)
    }

    /// ADD HL,BC
    /// 1 8
    /// - 0 H C
    fn op_0009(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_16(cpu, mmu, args, Src::HL, Src::BC)
    }

    /// LD A,(BC)
    /// 1  8
    /// - - - -
    fn op_000a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::BCa)
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
        self.ld(cpu, mmu, args, Src::C, Src::D8)
    }

    /// LD DE,d16
    /// 3 12
    /// - - - -
    fn op_0011(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::DE, Src::D16)
    }

    /// LD (DE),A
    /// 1 8
    /// - - - -
    fn op_0012(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::DEa, Src::A)
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
        self.ld(cpu, mmu, args, Src::D, Src::D8)
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
        self.add_16(cpu, mmu, args, Src::HL, Src::DE)
    }

    /// LD A,(DE)
    /// 1 8
    /// - - - -
    fn op_001a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::DEa)
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
        self.ld(cpu, mmu, args, Src::E, Src::D8)
    }

    /// JR NZ,r8
    /// 2 12/8
    /// - - - -
    fn op_0020(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !(cpu.registers.get_flags().zero) {
            self.relative_jump(cpu, args[0]);
        }
        self.cycles
    }

    /// LD HL,d16
    /// 3 12
    /// - - - -
    fn op_0021(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HL, Src::D16)
    }

    /// LDI (HL),A
    /// aka LD (HL+),A
    /// 1 8
    /// - - - -
    fn op_0022(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HLa, Src::A);
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
        self.ld(cpu, mmu, args, Src::H, Src::D8)
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
        self.add_16(cpu, mmu, args, Src::HL, Src::HL)
    }

    /// LDI A,(HL)
    /// aka LD A, (HL+)
    /// 1 8
    /// - - - -
    fn op_002a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::HLa);
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
        self.ld(cpu, mmu, args, Src::L, Src::D8)
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
        self.ld(cpu, mmu, args, Src::SP, Src::D16)
    }

    /// LDD (HL),A
    /// aka LD (HL-),A
    /// 1 8
    /// - - - -
    fn op_0032(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HLa, Src::A);
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
        self.ld(cpu, mmu, args, Src::HLa, Src::D8)
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
        self.add_16(cpu, mmu, args, Src::HL, Src::SP)
    }

    /// LDD A,(HL)
    /// aka LD A,(HL-)
    /// 1 8
    /// - - - -
     fn op_003a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::HLa);
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
        self.ld(cpu, mmu, args, Src::A, Src::D8)
    }

    /// LD B,B
    /// 1 4
    /// - - - -
    fn op_0040(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::B, Src::B)
    }

    /// LD B,C
    /// 1 4
    /// - - - -
    fn op_0041(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::B, Src::C)
    }

    /// LD B,D
    /// 1 4
    /// - - - -
    fn op_0042(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::B, Src::D)
    }

    /// LD B,E
    /// 1 4
    /// - - - -
    fn op_0043(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::B, Src::E)
    }

    /// LD B,H
    /// 1 4
    /// - - - -
    fn op_0044(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::B, Src::H)
    }

    /// LD B,L
    /// 1 4
    /// - - - -
    fn op_0045(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::B, Src::L)
    }

    /// LD B,(HL)
    /// 1 8
    /// - - - -
    fn op_0046(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::B, Src::HLa)
    }

    /// LD B,A
    /// 1 4
    /// - - - -
    fn op_0047(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::B, Src::A)
    }

    /// LD C,B
    /// 1 4
    /// - - - -
    fn op_0048(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::C, Src::B)
    }

    /// LD C,C
    /// 1 4
    /// - - - -
    fn op_0049(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::C, Src::C)
    }

    /// LD C,D
    /// 1 4
    /// - - - -
    fn op_004a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::C, Src::D)
    }

    /// LD C,E
    /// 1 4
    /// - - - -
    fn op_004b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::C, Src::E)
    }

    /// LD C,H
    /// 1 4
    /// - - - -
    fn op_004c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::C, Src::H)
    }

    /// LD C,L
    /// 1 4
    /// - - - -
    fn op_004d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::C, Src::L)
    }

    /// LD C,(HL)
    /// 1 8
    /// - - - -
    fn op_004e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::C, Src::HLa)
    }

    /// LD C,A
    /// 1 4
    /// - - - -
    fn op_004f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::C, Src::A)
    }

    /// LD D,B
    /// 1 4
    /// - - - -
    fn op_0050(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::D, Src::B)
    }

    /// LD D,C
    /// 1 4
    /// - - - -
    fn op_0051(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::D, Src::C)
    }

    /// LD D,D
    /// 1 4
    /// - - - -
    fn op_0052(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::D, Src::D)
    }

    /// LD D,E
    /// 1 4
    /// - - - -
    fn op_0053(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::D, Src::E)
    }

    /// LD D,H
    /// 1 4
    /// - - - -
    fn op_0054(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::D, Src::H)
    }

    /// LD D,L
    /// 1 4
    /// - - - -
    fn op_0055(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::D, Src::L)
    }

    /// LD D,(HL)
    /// 1 8
    /// - - - -
    fn op_0056(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::D, Src::HLa)
    }

    /// LD D,A
    /// 1 4
    /// - - - -
    fn op_0057(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::D, Src::A)
    }

    /// LD E,B
    /// 1 4
    /// - - - -
    fn op_0058(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::E, Src::B)
    }

    /// LD E,C
    /// 1 4
    /// - - - -
    fn op_0059(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::E, Src::C)
    }

    /// LD E,D
    /// 1 4
    /// - - - -
    fn op_005a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::E, Src::D)
    }

    /// LD E,E
    /// 1 4
    /// - - - -
    fn op_005b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::E, Src::E)
    }

    /// LD E,H
    /// 1 4
    /// - - - -
    fn op_005c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::E, Src::H)
    }

    /// LD E,L
    /// 1 4
    /// - - - -
    fn op_005d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::E, Src::L)
    }

    /// LD E,(HL)
    /// 1 8
    /// - - - -
    fn op_005e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::E, Src::HLa)
    }

    /// LD E,A
    /// 1 4
    /// - - - -
    fn op_005f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::E, Src::A)
    }

    /// LD H,B
    /// 1 4
    /// - - - -
    fn op_0060(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::H, Src::B)
    }

    /// LD H,C
    /// 1 4
    /// - - - -
    fn op_0061(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::H, Src::C)
    }

    /// LD H,D
    /// 1 4
    /// - - - -
    fn op_0062(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::H, Src::D)
    }

    /// LD H,E
    /// 1 4
    /// - - - -
    fn op_0063(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::H, Src::E)
    }

    /// LD H,H
    /// 1 4
    /// - - - -
    fn op_0064(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::H, Src::H)
    }

    /// LD H,L
    /// 1 4
    /// - - - -
    fn op_0065(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::H, Src::L)
    }

    /// LD H,(HL)
    /// 1 8
    /// - - - -
    fn op_0066(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::H, Src::HLa)
    }

    /// LD H,A
    /// 1 4
    /// - - - -
    fn op_0067(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::H, Src::A)
    }

    /// LD L,B
    /// 1 4
    /// - - - -
    fn op_0068(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::L, Src::B)
    }

    /// LD L,C
    /// 1 4
    /// - - - -
    fn op_0069(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::L, Src::C)
    }

    /// LD L,D
    /// 1 4
    /// - - - -
    fn op_006a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::L, Src::D)
    }

    /// LD L,E
    /// 1 4
    /// - - - -
    fn op_006b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::L, Src::E)
    }

    /// LD L,H
    /// 1 4
    /// - - - -
    fn op_006c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::L, Src::H)
    }

    /// LD L,L
    /// 1 4
    /// - - - -
    fn op_006d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::L, Src::L)
    }

    /// LD L,(HL)
    /// 1 8
    /// - - - -
    fn op_006e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::L, Src::HLa)
    }

    /// LD L,A
    /// 1 4
    /// - - - -
    fn op_006f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::L, Src::A)
    }

    /// LD (HL),B
    /// 1 8
    /// - - - -
    fn op_0070(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HLa, Src::B)
    }

    /// LD (HL),C
    /// 1 8
    /// - - - -
    fn op_0071(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HLa, Src::C)
    }

    /// LD (HL),D
    /// 1 8
    /// - - - -
    fn op_0072(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HLa, Src::D)
    }

    /// LD (HL),E
    /// 1 8
    /// - - - -
    fn op_0073(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HLa, Src::E)
    }

    /// LD (HL),H
    /// 1 8
    /// - - - -
    fn op_0074(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HLa, Src::H)
    }

    /// LD (HL),L
    /// 1 8
    /// - - - -
    fn op_0075(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HLa, Src::L)
    }

    /// HALT

    /// LD (HL),A
    /// 1 8
    /// - - - -
    fn op_0077(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::HLa, Src::A)
    }

    /// LD A,B
    /// 1 4
    /// - - - -
    fn op_0078(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::B)
    }

    /// LD A,C
    /// 1 4
    /// - - - -
    fn op_0079(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::C)
    }

    /// LD A,D
    /// 1 4
    /// - - - -
    fn op_007a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::D)
    }

    /// LD A,E
    /// 1 4
    /// - - - -
    fn op_007b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::E)
    }

    /// LD A,H
    /// 1 4
    /// - - - -
    fn op_007c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::H)
    }

    /// LD A,L
    /// 1 4
    /// - - - -
    fn op_007d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::L)
    }

    /// LD A,(HL)
    /// 1 8
    /// - - - -
    fn op_007e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::HLa)
    }

    /// LD A,A
    /// 1 4
    /// - - - -
    fn op_007f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::A)
    }

    /// ADD (HL)
    /// 1 8
    /// Z 0 H C
    fn op_0086(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::HLa)
    }

    /// SUB B
    /// 1 4
    /// Z 1 H C
    fn op_0090(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::B)
    }

    /// SUB C
    /// 1 4
    /// Z 1 H C
    fn op_0091(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::C)
    }

    /// SUB D
    /// 1 4
    /// Z 1 H C
    fn op_0092(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::D)
    }

    /// SUB E
    /// 1 4
    /// Z 1 H C
    fn op_0093(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::E)
    }

    /// SUB H
    /// 1 4
    /// Z 1 H C
    fn op_0094(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::H)
    }

    /// SUB L
    /// 1 4
    /// Z 1 H C
    fn op_0095(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::L)
    }

    /// SUB (HL)
    /// 1 8
    /// Z 1 H C
    fn op_0096(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::HLa)
    }

    /// SUB A
    /// 1 4
    /// Z 1 H C
    fn op_0097(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::A)
    }

    /// AND B
    /// 1 4
    /// Z 0 1 0
    fn op_00a0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::B)
    }

    /// AND C
    /// 1 4
    /// Z 0 1 0
    fn op_00a1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::C)
    }

    /// AND D
    /// 1 4
    /// Z 0 1 0
    fn op_00a2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::D)
    }

    /// AND E
    /// 1 4
    /// Z 0 1 0
    fn op_00a3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::E)
    }

    /// AND H
    /// 1 4
    /// Z 0 1 0
    fn op_00a4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::H)
    }

    /// AND L
    /// 1 4
    /// Z 0 1 0
    fn op_00a5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::L)
    }

    /// AND (HL)
    /// 1 8
    /// Z 0 1 0
    fn op_00a6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::HLa)
    }

    /// AND A
    /// 1 4
    /// Z 0 1 0
    fn op_00a7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::A)
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
        let address = ((args[1] as u16) << 8) | (args[0] as u16);
        self.jump(cpu, address);
        self.cycles
    }

    /// CALL NZ,a16
    /// 3 12/24
    /// - - - -
    fn op_00c4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !(cpu.registers.get_flags().zero) {
            let address = ((args[1] as u16) << 8) | (args[0] as u16);
            self.call(cpu, mmu, address);
            self.cycles += 12;
        }
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
        let address = ((args[1] as u16) << 8) | (args[0] as u16);
        self.call(cpu, mmu, address);
        self.cycles
    }

    /// ADC A,d8
    /// 2 8
    /// Z 0 H C
    fn op_00ce(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::D8)
    }

    /// PUSH DE
    /// 1 16
    /// - - - -
    fn op_00d5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.push(cpu, mmu, CpuRegIndex::DE);
        self.cycles
    }

    /// SUB d8
    /// 2 8
    /// Z 1 H C
    fn op_00d6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::D8)
    }

    /// LDH (a8),A
    /// aka LD ($FF00+a8),A
    /// 2 12
    /// - - - -
    fn op_00e0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A8, Src::A)
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
        self.ld(cpu, mmu, args, Src::Ca, Src::A)
    }

    /// PUSH HL
    /// 1 16
    /// - - - -
    fn op_00e5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.push(cpu, mmu, CpuRegIndex::HL);
        self.cycles
    }

    /// AND d8
    /// 2 8
    /// Z 0 1 0
    fn op_00e6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::D8)
    }

    /// LD (a16),A 
    /// 3 16 
    /// - - - -
    fn op_00ea(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A16, Src::A)
    }

    /// LDH A,(a8)
    /// aka LD A,($FF00+a8)
    /// 2 12
    /// - - - -
    fn op_00f0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld(cpu, mmu, args, Src::A, Src::A8)
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
        self.ld(cpu, mmu, args, Src::A, Src::Ca)
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
        self.ld(cpu, mmu, args, Src::A, Src::A16)
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
