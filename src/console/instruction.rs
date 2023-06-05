#![allow(dead_code)]
#![allow(unused_variables)]

use crate::console::alu;
use crate::console::cpu::{Cpu, PREFIX_BYTE};
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
            0x000F => Instruction { opcode, mnemonic: "RRCA", size: 1, cycles: 4, _fn: Instruction::op_000f },

            0x0010 => Instruction { opcode, mnemonic: "STOP", size: 2, cycles: 4, _fn: Instruction::op_0010 },
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
            0x001F => Instruction { opcode, mnemonic: "RRA", size: 1, cycles: 4, _fn: Instruction::op_001f },

            0x0020 => Instruction { opcode, mnemonic: "JR NZ,r8", size: 2, cycles: 8, _fn: Instruction::op_0020 },
            0x0021 => Instruction { opcode, mnemonic: "LD HL,d16", size: 3, cycles: 12, _fn: Instruction::op_0021 },
            0x0022 => Instruction { opcode, mnemonic: "LDI (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0022 },
            0x0023 => Instruction { opcode, mnemonic: "INC HL", size: 1, cycles: 8, _fn: Instruction::op_0023 },
            0x0024 => Instruction { opcode, mnemonic: "INC H", size: 1, cycles: 4, _fn: Instruction::op_0024 },
            0x0025 => Instruction { opcode, mnemonic: "DEC H", size: 1, cycles: 4, _fn: Instruction::op_0025 },
            0x0026 => Instruction { opcode, mnemonic: "LD H,d8", size: 2, cycles: 8, _fn: Instruction::op_0026 },
            0x0027 => Instruction { opcode, mnemonic: "DAA", size: 1, cycles: 4, _fn: Instruction::op_0027 },
            0x0028 => Instruction { opcode, mnemonic: "JR Z,r8", size: 2, cycles: 8, _fn: Instruction::op_0028 },
            0x0029 => Instruction { opcode, mnemonic: "ADD HL,HL", size: 1, cycles: 8, _fn: Instruction::op_0029 },
            0x002A => Instruction { opcode, mnemonic: "LDI A,(HL)", size: 1, cycles: 8, _fn: Instruction::op_002a },
            0x002B => Instruction { opcode, mnemonic: "DEC HL", size: 1, cycles: 8, _fn: Instruction::op_002b },
            0x002C => Instruction { opcode, mnemonic: "INC L", size: 1, cycles: 4, _fn: Instruction::op_002c },
            0x002D => Instruction { opcode, mnemonic: "DEC L", size: 1, cycles: 4, _fn: Instruction::op_002d },
            0x002E => Instruction { opcode, mnemonic: "LD L,d8", size: 2, cycles: 8, _fn: Instruction::op_002e },
            0x002F => Instruction { opcode, mnemonic: "CPL", size: 1, cycles: 4, _fn: Instruction::op_002f },

            0x0030 => Instruction { opcode, mnemonic: "JR NC,r8", size: 2, cycles: 8, _fn: Instruction::op_0030 },
            0x0031 => Instruction { opcode, mnemonic: "LD SP,d16", size: 3, cycles: 12, _fn: Instruction::op_0031 },
            0x0032 => Instruction { opcode, mnemonic: "LDD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0032 },
            0x0033 => Instruction { opcode, mnemonic: "INC SP", size: 1, cycles: 8, _fn: Instruction::op_0033 },
            0x0034 => Instruction { opcode, mnemonic: "INC (HL)", size: 1, cycles: 12, _fn: Instruction::op_0034 },
            0x0035 => Instruction { opcode, mnemonic: "DEC (HL)", size: 1, cycles: 12, _fn: Instruction::op_0035 },
            0x0036 => Instruction { opcode, mnemonic: "LD (HL),d8", size: 2, cycles: 12, _fn: Instruction::op_0036 },
            0x0037 => Instruction { opcode, mnemonic: "SCF", size: 1, cycles: 4, _fn: Instruction::op_0037 },
            0x0038 => Instruction { opcode, mnemonic: "JR C,r8", size: 2, cycles: 8, _fn: Instruction::op_0038 },
            0x0039 => Instruction { opcode, mnemonic: "ADD HL,SP", size: 1, cycles: 8, _fn: Instruction::op_0039 },
            0x003A => Instruction { opcode, mnemonic: "LDD A,(HL)", size: 1, cycles: 8, _fn: Instruction::op_003a },
            0x003B => Instruction { opcode, mnemonic: "DEC SP", size: 1, cycles: 8, _fn: Instruction::op_003b },
            0x003C => Instruction { opcode, mnemonic: "INC A", size: 1, cycles: 4, _fn: Instruction::op_003c },
            0x003D => Instruction { opcode, mnemonic: "DEC A", size: 1, cycles: 4, _fn: Instruction::op_003d },
            0x003E => Instruction { opcode, mnemonic: "LD A,d8", size: 2, cycles: 8, _fn: Instruction::op_003e },
            0x003F => Instruction { opcode, mnemonic: "CCF", size: 1, cycles: 4, _fn: Instruction::op_003f },

            0x0040 => Instruction { opcode, mnemonic: "LD B,B", size: 1, cycles: 4, _fn: Instruction::op_0040 },
            0x0041 => Instruction { opcode, mnemonic: "LD B,C", size: 1, cycles: 4, _fn: Instruction::op_0041 },
            0x0042 => Instruction { opcode, mnemonic: "LD B,D", size: 1, cycles: 4, _fn: Instruction::op_0042 },
            0x0043 => Instruction { opcode, mnemonic: "LD B,E", size: 1, cycles: 4, _fn: Instruction::op_0043 },
            0x0044 => Instruction { opcode, mnemonic: "LD B,H", size: 1, cycles: 4, _fn: Instruction::op_0044 },
            0x0045 => Instruction { opcode, mnemonic: "LD B,L", size: 1, cycles: 4, _fn: Instruction::op_0045 },
            0x0046 => Instruction { opcode, mnemonic: "LD B,(HL)", size: 1, cycles: 4, _fn: Instruction::op_0046 },
            0x0047 => Instruction { opcode, mnemonic: "LD B,A", size: 1, cycles: 4, _fn: Instruction::op_0047 },
            0x0048 => Instruction { opcode, mnemonic: "LD C,B", size: 1, cycles: 4, _fn: Instruction::op_0048 },
            0x0049 => Instruction { opcode, mnemonic: "LD C,C", size: 1, cycles: 4, _fn: Instruction::op_0049 },
            0x004A => Instruction { opcode, mnemonic: "LD C,D", size: 1, cycles: 4, _fn: Instruction::op_004a },
            0x004B => Instruction { opcode, mnemonic: "LD C,E", size: 1, cycles: 4, _fn: Instruction::op_004b },
            0x004C => Instruction { opcode, mnemonic: "LD C,H", size: 1, cycles: 4, _fn: Instruction::op_004c },
            0x004D => Instruction { opcode, mnemonic: "LD C,L", size: 1, cycles: 4, _fn: Instruction::op_004d },
            0x004E => Instruction { opcode, mnemonic: "LD C,(HL)", size: 1, cycles: 8, _fn: Instruction::op_004e },
            0x004F => Instruction { opcode, mnemonic: "LD C,A", size: 1, cycles: 4, _fn: Instruction::op_004f },

            0x0050 => Instruction { opcode, mnemonic: "LD D,B", size: 1, cycles: 4, _fn: Instruction::op_0050 },
            0x0051 => Instruction { opcode, mnemonic: "LD D,C", size: 1, cycles: 4, _fn: Instruction::op_0051 },
            0x0052 => Instruction { opcode, mnemonic: "LD D,D", size: 1, cycles: 4, _fn: Instruction::op_0052 },
            0x0053 => Instruction { opcode, mnemonic: "LD D,E", size: 1, cycles: 4, _fn: Instruction::op_0053 },
            0x0054 => Instruction { opcode, mnemonic: "LD D,H", size: 1, cycles: 4, _fn: Instruction::op_0054 },
            0x0055 => Instruction { opcode, mnemonic: "LD D,L", size: 1, cycles: 4, _fn: Instruction::op_0055 },
            0x0056 => Instruction { opcode, mnemonic: "LD D,(HL)", size: 1, cycles: 8, _fn: Instruction::op_0056 },
            0x0057 => Instruction { opcode, mnemonic: "LD D,A", size: 1, cycles: 4, _fn: Instruction::op_0057 },
            0x0058 => Instruction { opcode, mnemonic: "LD E,B", size: 1, cycles: 4, _fn: Instruction::op_0058 },
            0x0059 => Instruction { opcode, mnemonic: "LD E,C", size: 1, cycles: 4, _fn: Instruction::op_0059 },
            0x005A => Instruction { opcode, mnemonic: "LD E,D", size: 1, cycles: 4, _fn: Instruction::op_005a },
            0x005B => Instruction { opcode, mnemonic: "LD E,E", size: 1, cycles: 4, _fn: Instruction::op_005b },
            0x005C => Instruction { opcode, mnemonic: "LD E,H", size: 1, cycles: 4, _fn: Instruction::op_005c },
            0x005D => Instruction { opcode, mnemonic: "LD E,L", size: 1, cycles: 4, _fn: Instruction::op_005d },
            0x005E => Instruction { opcode, mnemonic: "LD E,(HL)", size: 1, cycles: 8, _fn: Instruction::op_005e },
            0x005F => Instruction { opcode, mnemonic: "LD E,A", size: 1, cycles: 4, _fn: Instruction::op_005f },

            0x0060 => Instruction { opcode, mnemonic: "LD H,B", size: 1, cycles: 4, _fn: Instruction::op_0060 },
            0x0061 => Instruction { opcode, mnemonic: "LD H,C", size: 1, cycles: 4, _fn: Instruction::op_0061 },
            0x0062 => Instruction { opcode, mnemonic: "LD H,D", size: 1, cycles: 4, _fn: Instruction::op_0062 },
            0x0063 => Instruction { opcode, mnemonic: "LD H,E", size: 1, cycles: 4, _fn: Instruction::op_0063 },
            0x0064 => Instruction { opcode, mnemonic: "LD H,H", size: 1, cycles: 4, _fn: Instruction::op_0064 },
            0x0065 => Instruction { opcode, mnemonic: "LD H,L", size: 1, cycles: 4, _fn: Instruction::op_0065 },
            0x0066 => Instruction { opcode, mnemonic: "LD H,(HL)", size: 1, cycles: 8, _fn: Instruction::op_0066 },
            0x0067 => Instruction { opcode, mnemonic: "LD H,A", size: 1, cycles: 4, _fn: Instruction::op_0067 },
            0x0068 => Instruction { opcode, mnemonic: "LD L,B", size: 1, cycles: 4, _fn: Instruction::op_0068 },
            0x0069 => Instruction { opcode, mnemonic: "LD L,C", size: 1, cycles: 4, _fn: Instruction::op_0069 },
            0x006A => Instruction { opcode, mnemonic: "LD L,D", size: 1, cycles: 4, _fn: Instruction::op_006a },
            0x006B => Instruction { opcode, mnemonic: "LD L,E", size: 1, cycles: 4, _fn: Instruction::op_006b },
            0x006C => Instruction { opcode, mnemonic: "LD L,H", size: 1, cycles: 4, _fn: Instruction::op_006c },
            0x006D => Instruction { opcode, mnemonic: "LD L,L", size: 1, cycles: 4, _fn: Instruction::op_006d },
            0x006E => Instruction { opcode, mnemonic: "LD L,(HL)", size: 1, cycles: 8, _fn: Instruction::op_006e },
            0x006F => Instruction { opcode, mnemonic: "LD L,A", size: 1, cycles: 4, _fn: Instruction::op_006f },

            0x0070 => Instruction { opcode, mnemonic: "LD (HL),B", size: 1, cycles: 8, _fn: Instruction::op_0070 },
            0x0071 => Instruction { opcode, mnemonic: "LD (HL),C", size: 1, cycles: 8, _fn: Instruction::op_0071 },
            0x0072 => Instruction { opcode, mnemonic: "LD (HL),D", size: 1, cycles: 8, _fn: Instruction::op_0072 },
            0x0073 => Instruction { opcode, mnemonic: "LD (HL),E", size: 1, cycles: 8, _fn: Instruction::op_0073 },
            0x0074 => Instruction { opcode, mnemonic: "LD (HL),H", size: 1, cycles: 8, _fn: Instruction::op_0074 },
            0x0075 => Instruction { opcode, mnemonic: "LD (HL),L", size: 1, cycles: 8, _fn: Instruction::op_0075 },
            0x0076 => Instruction { opcode, mnemonic: "HALT", size: 1, cycles: 4, _fn: Instruction::op_0076 },
            0x0077 => Instruction { opcode, mnemonic: "LD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0077 },
            0x0078 => Instruction { opcode, mnemonic: "LD A,B", size: 1, cycles: 4, _fn: Instruction::op_0078 },
            0x0079 => Instruction { opcode, mnemonic: "LD A,C", size: 1, cycles: 4, _fn: Instruction::op_0079 },
            0x007A => Instruction { opcode, mnemonic: "LD A,D", size: 1, cycles: 4, _fn: Instruction::op_007a },
            0x007B => Instruction { opcode, mnemonic: "LD A,E", size: 1, cycles: 4, _fn: Instruction::op_007b },
            0x007C => Instruction { opcode, mnemonic: "LD A,H", size: 1, cycles: 4, _fn: Instruction::op_007c },
            0x007D => Instruction { opcode, mnemonic: "LD A,L", size: 1, cycles: 4, _fn: Instruction::op_007d },
            0x007E => Instruction { opcode, mnemonic: "LD A,(HL)", size: 1, cycles: 8, _fn: Instruction::op_007e },
            0x007F => Instruction { opcode, mnemonic: "LD A,A", size: 1, cycles: 4, _fn: Instruction::op_007f },

            0x0080 => Instruction { opcode, mnemonic: "ADD A,B", size: 1, cycles: 4, _fn: Instruction::op_0080 },
            0x0081 => Instruction { opcode, mnemonic: "ADD A,C", size: 1, cycles: 4, _fn: Instruction::op_0081 },
            0x0082 => Instruction { opcode, mnemonic: "ADD A,D", size: 1, cycles: 4, _fn: Instruction::op_0082 },
            0x0083 => Instruction { opcode, mnemonic: "ADD A,E", size: 1, cycles: 4, _fn: Instruction::op_0083 },
            0x0084 => Instruction { opcode, mnemonic: "ADD A,H", size: 1, cycles: 4, _fn: Instruction::op_0084 },
            0x0085 => Instruction { opcode, mnemonic: "ADD A,L", size: 1, cycles: 4, _fn: Instruction::op_0085 },
            0x0086 => Instruction { opcode, mnemonic: "ADD A,(HL)", size: 1, cycles: 8, _fn: Instruction::op_0086 },
            0x0087 => Instruction { opcode, mnemonic: "ADD A,A", size: 1, cycles: 4, _fn: Instruction::op_0087 },
            0x0088 => Instruction { opcode, mnemonic: "ADC A,B", size: 1, cycles: 4, _fn: Instruction::op_0088 },
            0x0089 => Instruction { opcode, mnemonic: "ADC A,C", size: 1, cycles: 4, _fn: Instruction::op_0089 },
            0x008A => Instruction { opcode, mnemonic: "ADC A,D", size: 1, cycles: 4, _fn: Instruction::op_008a },
            0x008B => Instruction { opcode, mnemonic: "ADC A,E", size: 1, cycles: 4, _fn: Instruction::op_008b },
            0x008C => Instruction { opcode, mnemonic: "ADC A,H", size: 1, cycles: 4, _fn: Instruction::op_008c },
            0x008D => Instruction { opcode, mnemonic: "ADC A,L", size: 1, cycles: 4, _fn: Instruction::op_008d },
            0x008E => Instruction { opcode, mnemonic: "ADC A,(HL)", size: 1, cycles: 8, _fn: Instruction::op_008e },
            0x008F => Instruction { opcode, mnemonic: "ADC A,A", size: 1, cycles: 4, _fn: Instruction::op_008f },

            0x0090 => Instruction { opcode, mnemonic: "SUB A,B", size: 1, cycles: 4, _fn: Instruction::op_0090 },
            0x0091 => Instruction { opcode, mnemonic: "SUB A,C", size: 1, cycles: 4, _fn: Instruction::op_0091 },
            0x0092 => Instruction { opcode, mnemonic: "SUB A,D", size: 1, cycles: 4, _fn: Instruction::op_0092 },
            0x0093 => Instruction { opcode, mnemonic: "SUB A,E", size: 1, cycles: 4, _fn: Instruction::op_0093 },
            0x0094 => Instruction { opcode, mnemonic: "SUB A,H", size: 1, cycles: 4, _fn: Instruction::op_0094 },
            0x0095 => Instruction { opcode, mnemonic: "SUB A,L", size: 1, cycles: 4, _fn: Instruction::op_0095 },
            0x0096 => Instruction { opcode, mnemonic: "SUB A,(HL)", size: 1, cycles: 8, _fn: Instruction::op_0096 },
            0x0097 => Instruction { opcode, mnemonic: "SUB A,A", size: 1, cycles: 4, _fn: Instruction::op_0097 },
            0x0098 => Instruction { opcode, mnemonic: "SBC A,B", size: 1, cycles: 4, _fn: Instruction::op_0098 },
            0x0099 => Instruction { opcode, mnemonic: "SBC A,C", size: 1, cycles: 4, _fn: Instruction::op_0099 },
            0x009A => Instruction { opcode, mnemonic: "SBC A,D", size: 1, cycles: 4, _fn: Instruction::op_009a },
            0x009B => Instruction { opcode, mnemonic: "SBC A,E", size: 1, cycles: 4, _fn: Instruction::op_009b },
            0x009C => Instruction { opcode, mnemonic: "SBC A,H", size: 1, cycles: 4, _fn: Instruction::op_009c },
            0x009D => Instruction { opcode, mnemonic: "SBC A,L", size: 1, cycles: 4, _fn: Instruction::op_009d },
            0x009E => Instruction { opcode, mnemonic: "SBC A,(HL)", size: 1, cycles: 8, _fn: Instruction::op_009e },
            0x009F => Instruction { opcode, mnemonic: "SBC A,A", size: 1, cycles: 4, _fn: Instruction::op_009f },

            0x00A0 => Instruction { opcode, mnemonic: "AND B", size: 1, cycles: 4, _fn: Instruction::op_00a0 },
            0x00A1 => Instruction { opcode, mnemonic: "AND C", size: 1, cycles: 4, _fn: Instruction::op_00a1 },
            0x00A2 => Instruction { opcode, mnemonic: "AND D", size: 1, cycles: 4, _fn: Instruction::op_00a2 },
            0x00A3 => Instruction { opcode, mnemonic: "AND E", size: 1, cycles: 4, _fn: Instruction::op_00a3 },
            0x00A4 => Instruction { opcode, mnemonic: "AND H", size: 1, cycles: 4, _fn: Instruction::op_00a4 },
            0x00A5 => Instruction { opcode, mnemonic: "AND L", size: 1, cycles: 4, _fn: Instruction::op_00a5 },
            0x00A6 => Instruction { opcode, mnemonic: "AND (HL)", size: 1, cycles: 4, _fn: Instruction::op_00a6 },
            0x00A7 => Instruction { opcode, mnemonic: "AND A", size: 1, cycles: 8, _fn: Instruction::op_00a7 },
            0x00A8 => Instruction { opcode, mnemonic: "XOR B", size: 1, cycles: 4, _fn: Instruction::op_00a8 },
            0x00A9 => Instruction { opcode, mnemonic: "XOR C", size: 1, cycles: 4, _fn: Instruction::op_00a9 },
            0x00AA => Instruction { opcode, mnemonic: "XOR D", size: 1, cycles: 4, _fn: Instruction::op_00aa },
            0x00AB => Instruction { opcode, mnemonic: "XOR E", size: 1, cycles: 4, _fn: Instruction::op_00ab },
            0x00AC => Instruction { opcode, mnemonic: "XOR H", size: 1, cycles: 4, _fn: Instruction::op_00ac },
            0x00AD => Instruction { opcode, mnemonic: "XOR L", size: 1, cycles: 4, _fn: Instruction::op_00ad },
            0x00AE => Instruction { opcode, mnemonic: "XOR (HL)", size: 1, cycles: 8, _fn: Instruction::op_00ae },
            0x00AF => Instruction { opcode, mnemonic: "XOR A", size: 1, cycles: 4, _fn: Instruction::op_00af },

            0x00B0 => Instruction { opcode, mnemonic: "OR B", size: 1, cycles: 4, _fn: Instruction::op_00b0 },
            0x00B1 => Instruction { opcode, mnemonic: "OR C", size: 1, cycles: 4, _fn: Instruction::op_00b1 },
            0x00B2 => Instruction { opcode, mnemonic: "OR D", size: 1, cycles: 4, _fn: Instruction::op_00b2 },
            0x00B3 => Instruction { opcode, mnemonic: "OR E", size: 1, cycles: 4, _fn: Instruction::op_00b3 },
            0x00B4 => Instruction { opcode, mnemonic: "OR H", size: 1, cycles: 4, _fn: Instruction::op_00b4 },
            0x00B5 => Instruction { opcode, mnemonic: "OR L", size: 1, cycles: 4, _fn: Instruction::op_00b5 },
            0x00B6 => Instruction { opcode, mnemonic: "OR (HL)", size: 1, cycles: 8, _fn: Instruction::op_00b6 },
            0x00B7 => Instruction { opcode, mnemonic: "OR A", size: 1, cycles: 4, _fn: Instruction::op_00b7 },
            0x00B8 => Instruction { opcode, mnemonic: "CP B", size: 1, cycles: 4, _fn: Instruction::op_00b8 },
            0x00B9 => Instruction { opcode, mnemonic: "CP C", size: 1, cycles: 4, _fn: Instruction::op_00b9 },
            0x00BA => Instruction { opcode, mnemonic: "CP D", size: 1, cycles: 4, _fn: Instruction::op_00ba },
            0x00BB => Instruction { opcode, mnemonic: "CP E", size: 1, cycles: 4, _fn: Instruction::op_00bb },
            0x00BC => Instruction { opcode, mnemonic: "CP H", size: 1, cycles: 4, _fn: Instruction::op_00bc },
            0x00BD => Instruction { opcode, mnemonic: "CP L", size: 1, cycles: 4, _fn: Instruction::op_00bd },
            0x00BE => Instruction { opcode, mnemonic: "CP (HL)", size: 1, cycles: 8, _fn: Instruction::op_00be },
            0x00BF => Instruction { opcode, mnemonic: "CP A", size: 1, cycles: 4, _fn: Instruction::op_00bf },

            0x00C0 => Instruction { opcode, mnemonic: "RET NZ", size: 1, cycles: 8, _fn: Instruction::op_00c0 },
            0x00C1 => Instruction { opcode, mnemonic: "POP BC", size: 1, cycles: 12, _fn: Instruction::op_00c1 },
            0x00C2 => Instruction { opcode, mnemonic: "JP NZ,a16", size: 3, cycles: 12, _fn: Instruction::op_00c2 },
            0x00C3 => Instruction { opcode, mnemonic: "JP a16", size: 3, cycles: 16, _fn: Instruction::op_00c3 },
            0x00C4 => Instruction { opcode, mnemonic: "CALL NZ,a16", size: 3, cycles: 12, _fn: Instruction::op_00c4 },
            0x00C5 => Instruction { opcode, mnemonic: "PUSH BC", size: 1, cycles: 16, _fn: Instruction::op_00c5 },
            0x00C6 => Instruction { opcode, mnemonic: "ADD A,d8", size: 2, cycles: 8, _fn: Instruction::op_00c6 },
            0x00C7 => Instruction { opcode, mnemonic: "RST 00H", size: 1, cycles: 16, _fn: Instruction::op_00c7 },
            0x00C8 => Instruction { opcode, mnemonic: "RET Z", size: 1, cycles: 8, _fn: Instruction::op_00c8 },
            0x00C9 => Instruction { opcode, mnemonic: "RET", size: 1, cycles: 16, _fn: Instruction::op_00c9 },
            0x00CA => Instruction { opcode, mnemonic: "JP Z,a16", size: 3, cycles: 12, _fn: Instruction::op_00ca },
            // 0x00CB = PREFIX
            0x00CC => Instruction { opcode, mnemonic: "CALL Z,a16", size: 3, cycles: 12, _fn: Instruction::op_00cc },
            0x00CD => Instruction { opcode, mnemonic: "CALL a16", size: 3, cycles: 24, _fn: Instruction::op_00cd },
            0x00CE => Instruction { opcode, mnemonic: "ADC A,d8", size: 2, cycles: 8, _fn: Instruction::op_00ce },
            0x00CF => Instruction { opcode, mnemonic: "RST 08H", size: 1, cycles: 16, _fn: Instruction::op_00cf },

            0x00D0 => Instruction { opcode, mnemonic: "RET NC", size: 1, cycles: 8, _fn: Instruction::op_00d0 },
            0x00D1 => Instruction { opcode, mnemonic: "POP DE", size: 1, cycles: 12, _fn: Instruction::op_00d1 },
            0x00D2 => Instruction { opcode, mnemonic: "JP NC,a16", size: 3, cycles: 12, _fn: Instruction::op_00d2 },
            0x00D3 => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00D4 => Instruction { opcode, mnemonic: "CALL NC,a16", size: 3, cycles: 12, _fn: Instruction::op_00d4 },
            0x00D5 => Instruction { opcode, mnemonic: "PUSH DE", size: 1, cycles: 16, _fn: Instruction::op_00d5 },
            0x00D6 => Instruction { opcode, mnemonic: "SUB d8", size: 2, cycles: 8, _fn: Instruction::op_00d6 },
            0x00d7 => Instruction { opcode, mnemonic: "RST 10H", size: 1, cycles: 16, _fn: Instruction::op_00d7 },
            0x00D8 => Instruction { opcode, mnemonic: "RET C", size: 1, cycles: 8, _fn: Instruction::op_00d8 },
            0x00D9 => Instruction { opcode, mnemonic: "RET I", size: 1, cycles: 16, _fn: Instruction::op_00d9 },
            0x00DA => Instruction { opcode, mnemonic: "JP C,a16", size: 3, cycles: 12, _fn: Instruction::op_00da },
            0x00DB => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00DC => Instruction { opcode, mnemonic: "CALL C,a16", size: 3, cycles: 12, _fn: Instruction::op_00dc },
            0x00DD => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00DE => Instruction { opcode, mnemonic: "SBC A,d8", size: 2, cycles: 8, _fn: Instruction::op_00de },
            0x00DF => Instruction { opcode, mnemonic: "RST 18H", size: 1, cycles: 16, _fn: Instruction::op_00df },

            0x00E0 => Instruction { opcode, mnemonic: "LDH (a8),A", size: 2, cycles: 12, _fn: Instruction::op_00e0 },
            0x00E1 => Instruction { opcode, mnemonic: "POP HL", size: 1, cycles: 12, _fn: Instruction::op_00e1 },
            0x00E2 => Instruction { opcode, mnemonic: "LDH (C),A", size: 1, cycles: 8, _fn: Instruction::op_00e2 },
            0x00E3 => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00E4 => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00E5 => Instruction { opcode, mnemonic: "PUSH HL", size: 1, cycles: 16, _fn: Instruction::op_00e5 },
            0x00E6 => Instruction { opcode, mnemonic: "AND d8", size: 2, cycles: 8, _fn: Instruction::op_00e6 },
            0x00E7 => Instruction { opcode, mnemonic: "RST 20H", size: 1, cycles: 16, _fn: Instruction::op_00e7 },
            0x00E8 => Instruction { opcode, mnemonic: "ADD SP,r8", size: 2, cycles: 16, _fn: Instruction::op_00e8 },
            0x00E9 => Instruction { opcode, mnemonic: "JP HL", size: 1, cycles: 4, _fn: Instruction::op_00e9 },
            0x00EA => Instruction { opcode, mnemonic: "LD (a16),A", size: 3, cycles: 16, _fn: Instruction::op_00ea },
            0x00EB => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00EC => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00ED => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00EE => Instruction { opcode, mnemonic: "XOR d8", size: 2, cycles: 8, _fn: Instruction::op_00ee },
            0x00EF => Instruction { opcode, mnemonic: "RST 28H", size: 1, cycles: 16, _fn: Instruction::op_00ef },

            0x00F0 => Instruction { opcode, mnemonic: "LDH A,(a8)", size: 2, cycles: 12, _fn: Instruction::op_00f0 },
            0x00F1 => Instruction { opcode, mnemonic: "POP AF", size: 1, cycles: 12, _fn: Instruction::op_00f1 },
            0x00F2 => Instruction { opcode, mnemonic: "LDH A,(C)", size: 1, cycles: 8, _fn: Instruction::op_00f2 },
            0x00F3 => Instruction { opcode, mnemonic: "DI", size: 1, cycles: 4, _fn: Instruction::op_00f3 },
            0x00F4 => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00F5 => Instruction { opcode, mnemonic: "PUSH AF", size: 1, cycles: 16, _fn: Instruction::op_00f5 },
            0x00F6 => Instruction { opcode, mnemonic: "OR d8", size: 2, cycles: 8, _fn: Instruction::op_00f6 },
            0x00F7 => Instruction { opcode, mnemonic: "RST 30H", size: 1, cycles: 16, _fn: Instruction::op_00f7 },
            0x00F8 => Instruction { opcode, mnemonic: "LD HL,SP+r8", size: 2, cycles: 12, _fn: Instruction::op_00f8 },
            0x00F9 => Instruction { opcode, mnemonic: "LD SP,HL", size: 1, cycles: 8, _fn: Instruction::op_00f9 },
            0x00FA => Instruction { opcode, mnemonic: "LD A,(a16)", size: 3, cycles: 16, _fn: Instruction::op_00fa },
            0x00FB => Instruction { opcode, mnemonic: "EI", size: 1, cycles: 4, _fn: Instruction::op_00fb },
            0x00FC => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00FD => Instruction { opcode, mnemonic: "Invalid", size: 1, cycles: -1, _fn: Instruction::invalid },
            0x00FE => Instruction { opcode, mnemonic: "CP d8", size: 2, cycles: 8, _fn: Instruction::op_00fe },
            0x00FF => Instruction { opcode, mnemonic: "RST 38H", size: 1, cycles: 16, _fn: Instruction::op_00ff },

            // ==================================== CB PREFIXED ====================================

            0xCB00 => Instruction { opcode, mnemonic: "RLC B", size: 2, cycles: 8, _fn: Instruction::op_cb00 },
            0xCB01 => Instruction { opcode, mnemonic: "RLC C", size: 2, cycles: 8, _fn: Instruction::op_cb01 },
            0xCB02 => Instruction { opcode, mnemonic: "RLC D", size: 2, cycles: 8, _fn: Instruction::op_cb02 },
            0xCB03 => Instruction { opcode, mnemonic: "RLC E", size: 2, cycles: 8, _fn: Instruction::op_cb03 },
            0xCB04 => Instruction { opcode, mnemonic: "RLC H", size: 2, cycles: 8, _fn: Instruction::op_cb04 },
            0xCB05 => Instruction { opcode, mnemonic: "RLC L", size: 2, cycles: 8, _fn: Instruction::op_cb05 },
            0xCB06 => Instruction { opcode, mnemonic: "RLC (HL)", size: 2, cycles: 16, _fn: Instruction::op_cb06 },
            0xCB07 => Instruction { opcode, mnemonic: "RLC A", size: 2, cycles: 8, _fn: Instruction::op_cb07 },
            0xCB08 => Instruction { opcode, mnemonic: "RRC B", size: 2, cycles: 8, _fn: Instruction::op_cb08 },
            0xCB09 => Instruction { opcode, mnemonic: "RRC C", size: 2, cycles: 8, _fn: Instruction::op_cb09 },
            0xCB0A => Instruction { opcode, mnemonic: "RRC D", size: 2, cycles: 8, _fn: Instruction::op_cb0a },
            0xCB0B => Instruction { opcode, mnemonic: "RRC E", size: 2, cycles: 8, _fn: Instruction::op_cb0b },
            0xCB0C => Instruction { opcode, mnemonic: "RRC H", size: 2, cycles: 8, _fn: Instruction::op_cb0c },
            0xCB0D => Instruction { opcode, mnemonic: "RRC L", size: 2, cycles: 8, _fn: Instruction::op_cb0d },
            0xCB0E => Instruction { opcode, mnemonic: "RRC (HL)", size: 2, cycles: 16, _fn: Instruction::op_cb0e },
            0xCB0F => Instruction { opcode, mnemonic: "RRC A", size: 2, cycles: 8, _fn: Instruction::op_cb0f },

            0xCB10 => Instruction { opcode, mnemonic: "RL B", size: 2, cycles: 8, _fn: Instruction::op_cb10 },
            0xCB11 => Instruction { opcode, mnemonic: "RL C", size: 2, cycles: 8, _fn: Instruction::op_cb11 },
            0xCB12 => Instruction { opcode, mnemonic: "RL D", size: 2, cycles: 8, _fn: Instruction::op_cb12 },
            0xCB13 => Instruction { opcode, mnemonic: "RL E", size: 2, cycles: 8, _fn: Instruction::op_cb13 },
            0xCB14 => Instruction { opcode, mnemonic: "RL H", size: 2, cycles: 8, _fn: Instruction::op_cb14 },
            0xCB15 => Instruction { opcode, mnemonic: "RL L", size: 2, cycles: 8, _fn: Instruction::op_cb15 },
            0xCB16 => Instruction { opcode, mnemonic: "RL (HL)", size: 2, cycles: 16, _fn: Instruction::op_cb16 },
            0xCB17 => Instruction { opcode, mnemonic: "RL A", size: 2, cycles: 8, _fn: Instruction::op_cb17 },
            0xCB18 => Instruction { opcode, mnemonic: "RR B", size: 2, cycles: 8, _fn: Instruction::op_cb18 },
            0xCB19 => Instruction { opcode, mnemonic: "RR C", size: 2, cycles: 8, _fn: Instruction::op_cb19 },
            0xCB1A => Instruction { opcode, mnemonic: "RR D", size: 2, cycles: 8, _fn: Instruction::op_cb1a },
            0xCB1B => Instruction { opcode, mnemonic: "RR E", size: 2, cycles: 8, _fn: Instruction::op_cb1b },
            0xCB1C => Instruction { opcode, mnemonic: "RR H", size: 2, cycles: 8, _fn: Instruction::op_cb1c },
            0xCB1D => Instruction { opcode, mnemonic: "RR L", size: 2, cycles: 8, _fn: Instruction::op_cb1d },
            0xCB1E => Instruction { opcode, mnemonic: "RR (HL)", size: 2, cycles: 16, _fn: Instruction::op_cb1e },
            0xCB1F => Instruction { opcode, mnemonic: "RR A", size: 2, cycles: 8, _fn: Instruction::op_cb1f },

            0xCB20 => Instruction { opcode, mnemonic: "SLA B", size: 2, cycles: 8, _fn: Instruction::op_cb20 },
            0xCB21 => Instruction { opcode, mnemonic: "SLA C", size: 2, cycles: 8, _fn: Instruction::op_cb21 },
            0xCB22 => Instruction { opcode, mnemonic: "SLA D", size: 2, cycles: 8, _fn: Instruction::op_cb22 },
            0xCB23 => Instruction { opcode, mnemonic: "SLA E", size: 2, cycles: 8, _fn: Instruction::op_cb23 },
            0xCB24 => Instruction { opcode, mnemonic: "SLA H", size: 2, cycles: 8, _fn: Instruction::op_cb24 },
            0xCB25 => Instruction { opcode, mnemonic: "SLA L", size: 2, cycles: 8, _fn: Instruction::op_cb25 },
            0xCB26 => Instruction { opcode, mnemonic: "SLA (HL)", size: 2, cycles: 16, _fn: Instruction::op_cb26 },
            0xCB27 => Instruction { opcode, mnemonic: "SLA A", size: 2, cycles: 8, _fn: Instruction::op_cb27 },
            0xCB28 => Instruction { opcode, mnemonic: "SRA B", size: 2, cycles: 8, _fn: Instruction::op_cb28 },
            0xCB29 => Instruction { opcode, mnemonic: "SRA C", size: 2, cycles: 8, _fn: Instruction::op_cb29 },
            0xCB2A => Instruction { opcode, mnemonic: "SRA D", size: 2, cycles: 8, _fn: Instruction::op_cb2a },
            0xCB2B => Instruction { opcode, mnemonic: "SRA E", size: 2, cycles: 8, _fn: Instruction::op_cb2b },
            0xCB2C => Instruction { opcode, mnemonic: "SRA H", size: 2, cycles: 8, _fn: Instruction::op_cb2c },
            0xCB2D => Instruction { opcode, mnemonic: "SRA L", size: 2, cycles: 8, _fn: Instruction::op_cb2d },
            0xCB2E => Instruction { opcode, mnemonic: "SRA (HL)", size: 2, cycles: 16, _fn: Instruction::op_cb2e },
            0xCB2F => Instruction { opcode, mnemonic: "SRA A", size: 2, cycles: 8, _fn: Instruction::op_cb2f },

            0xCB30 => Instruction { opcode, mnemonic: "SWAP B", size: 2, cycles: 8, _fn: Instruction::op_cb30 },
            0xCB31 => Instruction { opcode, mnemonic: "SWAP C", size: 2, cycles: 8, _fn: Instruction::op_cb31 },
            0xCB32 => Instruction { opcode, mnemonic: "SWAP D", size: 2, cycles: 8, _fn: Instruction::op_cb32 },
            0xCB33 => Instruction { opcode, mnemonic: "SWAP E", size: 2, cycles: 8, _fn: Instruction::op_cb33 },
            0xCB34 => Instruction { opcode, mnemonic: "SWAP H", size: 2, cycles: 8, _fn: Instruction::op_cb34 },
            0xCB35 => Instruction { opcode, mnemonic: "SWAP L", size: 2, cycles: 8, _fn: Instruction::op_cb35 },
            0xCB36 => Instruction { opcode, mnemonic: "SWAP (HL)", size: 2, cycles: 16, _fn: Instruction::op_cb36 },
            0xCB37 => Instruction { opcode, mnemonic: "SWAP A", size: 2, cycles: 8, _fn: Instruction::op_cb37 },
            0xCB38 => Instruction { opcode, mnemonic: "SRL B", size: 2, cycles: 8, _fn: Instruction::op_cb38 },
            0xCB39 => Instruction { opcode, mnemonic: "SRL C", size: 2, cycles: 8, _fn: Instruction::op_cb39 },
            0xCB3A => Instruction { opcode, mnemonic: "SRL D", size: 2, cycles: 8, _fn: Instruction::op_cb3a },
            0xCB3B => Instruction { opcode, mnemonic: "SRL E", size: 2, cycles: 8, _fn: Instruction::op_cb3b },
            0xCB3C => Instruction { opcode, mnemonic: "SRL H", size: 2, cycles: 8, _fn: Instruction::op_cb3c },
            0xCB3D => Instruction { opcode, mnemonic: "SRL L", size: 2, cycles: 8, _fn: Instruction::op_cb3d },
            0xCB3E => Instruction { opcode, mnemonic: "SRL (HL)", size: 2, cycles: 16, _fn: Instruction::op_cb3e },
            0xCB3F => Instruction { opcode, mnemonic: "SRL A", size: 2, cycles: 8, _fn: Instruction::op_cb3f },

            0xCB40 => Instruction { opcode, mnemonic: "BIT 0,B", size: 2, cycles: 8, _fn: Instruction::op_cb40 },
            0xCB41 => Instruction { opcode, mnemonic: "BIT 0,C", size: 2, cycles: 8, _fn: Instruction::op_cb41 },
            0xCB42 => Instruction { opcode, mnemonic: "BIT 0,D", size: 2, cycles: 8, _fn: Instruction::op_cb42 },
            0xCB43 => Instruction { opcode, mnemonic: "BIT 0,E", size: 2, cycles: 8, _fn: Instruction::op_cb43 },
            0xCB44 => Instruction { opcode, mnemonic: "BIT 0,H", size: 2, cycles: 8, _fn: Instruction::op_cb44 },
            0xCB45 => Instruction { opcode, mnemonic: "BIT 0,L", size: 2, cycles: 8, _fn: Instruction::op_cb45 },
            0xCB46 => Instruction { opcode, mnemonic: "BIT 0,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb46 },
            0xCB47 => Instruction { opcode, mnemonic: "BIT 0,A", size: 2, cycles: 8, _fn: Instruction::op_cb47 },
            0xCB48 => Instruction { opcode, mnemonic: "BIT 1,B", size: 2, cycles: 8, _fn: Instruction::op_cb48 },
            0xCB49 => Instruction { opcode, mnemonic: "BIT 1,C", size: 2, cycles: 8, _fn: Instruction::op_cb49 },
            0xCB4A => Instruction { opcode, mnemonic: "BIT 1,D", size: 2, cycles: 8, _fn: Instruction::op_cb4a },
            0xCB4B => Instruction { opcode, mnemonic: "BIT 1,E", size: 2, cycles: 8, _fn: Instruction::op_cb4b },
            0xCB4C => Instruction { opcode, mnemonic: "BIT 1,H", size: 2, cycles: 8, _fn: Instruction::op_cb4c },
            0xCB4D => Instruction { opcode, mnemonic: "BIT 1,L", size: 2, cycles: 8, _fn: Instruction::op_cb4d },
            0xCB4E => Instruction { opcode, mnemonic: "BIT 1,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb4e },
            0xCB4f => Instruction { opcode, mnemonic: "BIT 1,A", size: 2, cycles: 8, _fn: Instruction::op_cb4f },

            0xCB50 => Instruction { opcode, mnemonic: "BIT 2,B", size: 2, cycles: 8, _fn: Instruction::op_cb50 },
            0xCB51 => Instruction { opcode, mnemonic: "BIT 2,C", size: 2, cycles: 8, _fn: Instruction::op_cb51 },
            0xCB52 => Instruction { opcode, mnemonic: "BIT 2,D", size: 2, cycles: 8, _fn: Instruction::op_cb52 },
            0xCB53 => Instruction { opcode, mnemonic: "BIT 2,E", size: 2, cycles: 8, _fn: Instruction::op_cb53 },
            0xCB54 => Instruction { opcode, mnemonic: "BIT 2,H", size: 2, cycles: 8, _fn: Instruction::op_cb54 },
            0xCB55 => Instruction { opcode, mnemonic: "BIT 2,L", size: 2, cycles: 8, _fn: Instruction::op_cb55 },
            0xCB56 => Instruction { opcode, mnemonic: "BIT 2,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb56 },
            0xCB57 => Instruction { opcode, mnemonic: "BIT 2,A", size: 2, cycles: 8, _fn: Instruction::op_cb57 },
            0xCB58 => Instruction { opcode, mnemonic: "BIT 3,B", size: 2, cycles: 8, _fn: Instruction::op_cb58 },
            0xCB59 => Instruction { opcode, mnemonic: "BIT 3,C", size: 2, cycles: 8, _fn: Instruction::op_cb59 },
            0xCB5A => Instruction { opcode, mnemonic: "BIT 3,D", size: 2, cycles: 8, _fn: Instruction::op_cb5a },
            0xCB5B => Instruction { opcode, mnemonic: "BIT 3,E", size: 2, cycles: 8, _fn: Instruction::op_cb5b },
            0xCB5C => Instruction { opcode, mnemonic: "BIT 3,H", size: 2, cycles: 8, _fn: Instruction::op_cb5c },
            0xCB5D => Instruction { opcode, mnemonic: "BIT 3,L", size: 2, cycles: 8, _fn: Instruction::op_cb5d },
            0xCB5E => Instruction { opcode, mnemonic: "BIT 3,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb5e },
            0xCB5F => Instruction { opcode, mnemonic: "BIT 3,A", size: 2, cycles: 8, _fn: Instruction::op_cb5f },

            0xCB60 => Instruction { opcode, mnemonic: "BIT 4,B", size: 2, cycles: 8, _fn: Instruction::op_cb60 },
            0xCB61 => Instruction { opcode, mnemonic: "BIT 4,C", size: 2, cycles: 8, _fn: Instruction::op_cb61 },
            0xCB62 => Instruction { opcode, mnemonic: "BIT 4,D", size: 2, cycles: 8, _fn: Instruction::op_cb62 },
            0xCB63 => Instruction { opcode, mnemonic: "BIT 4,E", size: 2, cycles: 8, _fn: Instruction::op_cb63 },
            0xCB64 => Instruction { opcode, mnemonic: "BIT 4,H", size: 2, cycles: 8, _fn: Instruction::op_cb64 },
            0xCB65 => Instruction { opcode, mnemonic: "BIT 4,L", size: 2, cycles: 8, _fn: Instruction::op_cb65 },
            0xCB66 => Instruction { opcode, mnemonic: "BIT 4,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb66 },
            0xCB67 => Instruction { opcode, mnemonic: "BIT 4,A", size: 2, cycles: 8, _fn: Instruction::op_cb67 },
            0xCB68 => Instruction { opcode, mnemonic: "BIT 5,B", size: 2, cycles: 8, _fn: Instruction::op_cb68 },
            0xCB69 => Instruction { opcode, mnemonic: "BIT 5,C", size: 2, cycles: 8, _fn: Instruction::op_cb69 },
            0xCB6A => Instruction { opcode, mnemonic: "BIT 5,D", size: 2, cycles: 8, _fn: Instruction::op_cb6a },
            0xCB6B => Instruction { opcode, mnemonic: "BIT 5,E", size: 2, cycles: 8, _fn: Instruction::op_cb6b },
            0xCB6C => Instruction { opcode, mnemonic: "BIT 5,H", size: 2, cycles: 8, _fn: Instruction::op_cb6c },
            0xCB6D => Instruction { opcode, mnemonic: "BIT 5,L", size: 2, cycles: 8, _fn: Instruction::op_cb6d },
            0xCB6E => Instruction { opcode, mnemonic: "BIT 5,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb6e },
            0xCB6F => Instruction { opcode, mnemonic: "BIT 5,A", size: 2, cycles: 8, _fn: Instruction::op_cb6f },

            0xCB70 => Instruction { opcode, mnemonic: "BIT 6,B", size: 2, cycles: 8, _fn: Instruction::op_cb70 },
            0xCB71 => Instruction { opcode, mnemonic: "BIT 6,C", size: 2, cycles: 8, _fn: Instruction::op_cb71 },
            0xCB72 => Instruction { opcode, mnemonic: "BIT 6,D", size: 2, cycles: 8, _fn: Instruction::op_cb72 },
            0xCB73 => Instruction { opcode, mnemonic: "BIT 6,E", size: 2, cycles: 8, _fn: Instruction::op_cb73 },
            0xCB74 => Instruction { opcode, mnemonic: "BIT 6,H", size: 2, cycles: 8, _fn: Instruction::op_cb74 },
            0xCB75 => Instruction { opcode, mnemonic: "BIT 6,L", size: 2, cycles: 8, _fn: Instruction::op_cb75 },
            0xCB76 => Instruction { opcode, mnemonic: "BIT 6,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb76 },
            0xCB77 => Instruction { opcode, mnemonic: "BIT 6,A", size: 2, cycles: 8, _fn: Instruction::op_cb77 },
            0xCB78 => Instruction { opcode, mnemonic: "BIT 7,B", size: 2, cycles: 8, _fn: Instruction::op_cb78 },
            0xCB79 => Instruction { opcode, mnemonic: "BIT 7,C", size: 2, cycles: 8, _fn: Instruction::op_cb79 },
            0xCB7A => Instruction { opcode, mnemonic: "BIT 7,D", size: 2, cycles: 8, _fn: Instruction::op_cb7a },
            0xCB7B => Instruction { opcode, mnemonic: "BIT 7,E", size: 2, cycles: 8, _fn: Instruction::op_cb7b },
            0xCB7C => Instruction { opcode, mnemonic: "BIT 7,H", size: 2, cycles: 8, _fn: Instruction::op_cb7c },
            0xCB7D => Instruction { opcode, mnemonic: "BIT 7,L", size: 2, cycles: 8, _fn: Instruction::op_cb7d },
            0xCB7E => Instruction { opcode, mnemonic: "BIT 7,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb7e },
            0xCB7F => Instruction { opcode, mnemonic: "BIT 7,A", size: 2, cycles: 8, _fn: Instruction::op_cb7f },

            0xCB80 => Instruction { opcode, mnemonic: "RES 0,B", size: 2, cycles: 8, _fn: Instruction::op_cb80 },
            0xCB81 => Instruction { opcode, mnemonic: "RES 0,C", size: 2, cycles: 8, _fn: Instruction::op_cb81 },
            0xCB82 => Instruction { opcode, mnemonic: "RES 0,D", size: 2, cycles: 8, _fn: Instruction::op_cb82 },
            0xCB83 => Instruction { opcode, mnemonic: "RES 0,E", size: 2, cycles: 8, _fn: Instruction::op_cb83 },
            0xCB84 => Instruction { opcode, mnemonic: "RES 0,H", size: 2, cycles: 8, _fn: Instruction::op_cb84 },
            0xCB85 => Instruction { opcode, mnemonic: "RES 0,L", size: 2, cycles: 8, _fn: Instruction::op_cb85 },
            0xCB86 => Instruction { opcode, mnemonic: "RES 0,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb86 },
            0xCB87 => Instruction { opcode, mnemonic: "RES 0,A", size: 2, cycles: 8, _fn: Instruction::op_cb87 },
            0xCB88 => Instruction { opcode, mnemonic: "RES 1,B", size: 2, cycles: 8, _fn: Instruction::op_cb88 },
            0xCB89 => Instruction { opcode, mnemonic: "RES 1,C", size: 2, cycles: 8, _fn: Instruction::op_cb89 },
            0xCB8A => Instruction { opcode, mnemonic: "RES 1,D", size: 2, cycles: 8, _fn: Instruction::op_cb8a },
            0xCB8B => Instruction { opcode, mnemonic: "RES 1,E", size: 2, cycles: 8, _fn: Instruction::op_cb8b },
            0xCB8C => Instruction { opcode, mnemonic: "RES 1,F", size: 2, cycles: 8, _fn: Instruction::op_cb8c },
            0xCB8D => Instruction { opcode, mnemonic: "RES 1,H", size: 2, cycles: 8, _fn: Instruction::op_cb8d },
            0xCB8E => Instruction { opcode, mnemonic: "RES 1,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb8e },
            0xCB8F => Instruction { opcode, mnemonic: "RES 1,A", size: 2, cycles: 8, _fn: Instruction::op_cb8f },

            0xCB90 => Instruction { opcode, mnemonic: "RES 2,B", size: 2, cycles: 8, _fn: Instruction::op_cb90 },
            0xCB91 => Instruction { opcode, mnemonic: "RES 2,C", size: 2, cycles: 8, _fn: Instruction::op_cb91 },
            0xCB92 => Instruction { opcode, mnemonic: "RES 2,D", size: 2, cycles: 8, _fn: Instruction::op_cb92 },
            0xCB93 => Instruction { opcode, mnemonic: "RES 2,E", size: 2, cycles: 8, _fn: Instruction::op_cb93 },
            0xCB94 => Instruction { opcode, mnemonic: "RES 2,H", size: 2, cycles: 8, _fn: Instruction::op_cb94 },
            0xCB95 => Instruction { opcode, mnemonic: "RES 2,L", size: 2, cycles: 8, _fn: Instruction::op_cb95 },
            0xCB96 => Instruction { opcode, mnemonic: "RES 2,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb96 },
            0xCB97 => Instruction { opcode, mnemonic: "RES 2,A", size: 2, cycles: 8, _fn: Instruction::op_cb97 },
            0xCB98 => Instruction { opcode, mnemonic: "RES 3,B", size: 2, cycles: 8, _fn: Instruction::op_cb98 },
            0xCB99 => Instruction { opcode, mnemonic: "RES 3,C", size: 2, cycles: 8, _fn: Instruction::op_cb99 },
            0xCB9A => Instruction { opcode, mnemonic: "RES 3,D", size: 2, cycles: 8, _fn: Instruction::op_cb9a },
            0xCB9B => Instruction { opcode, mnemonic: "RES 3,E", size: 2, cycles: 8, _fn: Instruction::op_cb9b },
            0xCB9C => Instruction { opcode, mnemonic: "RES 3,F", size: 2, cycles: 8, _fn: Instruction::op_cb9c },
            0xCB9D => Instruction { opcode, mnemonic: "RES 3,H", size: 2, cycles: 8, _fn: Instruction::op_cb9d },
            0xCB9E => Instruction { opcode, mnemonic: "RES 3,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cb9e },
            0xCB9F => Instruction { opcode, mnemonic: "RES 3,A", size: 2, cycles: 8, _fn: Instruction::op_cb9f },

            0xCBA0 => Instruction { opcode, mnemonic: "RES 4,B", size: 2, cycles: 8, _fn: Instruction::op_cba0 },
            0xCBA1 => Instruction { opcode, mnemonic: "RES 4,C", size: 2, cycles: 8, _fn: Instruction::op_cba1 },
            0xCBA2 => Instruction { opcode, mnemonic: "RES 4,D", size: 2, cycles: 8, _fn: Instruction::op_cba2 },
            0xCBA3 => Instruction { opcode, mnemonic: "RES 4,E", size: 2, cycles: 8, _fn: Instruction::op_cba3 },
            0xCBA4 => Instruction { opcode, mnemonic: "RES 4,H", size: 2, cycles: 8, _fn: Instruction::op_cba4 },
            0xCBA5 => Instruction { opcode, mnemonic: "RES 4,L", size: 2, cycles: 8, _fn: Instruction::op_cba5 },
            0xCBA6 => Instruction { opcode, mnemonic: "RES 4,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cba6 },
            0xCBA7 => Instruction { opcode, mnemonic: "RES 4,A", size: 2, cycles: 8, _fn: Instruction::op_cba7 },
            0xCBA8 => Instruction { opcode, mnemonic: "RES 5,B", size: 2, cycles: 8, _fn: Instruction::op_cba8 },
            0xCBA9 => Instruction { opcode, mnemonic: "RES 5,C", size: 2, cycles: 8, _fn: Instruction::op_cba9 },
            0xCBAA => Instruction { opcode, mnemonic: "RES 5,D", size: 2, cycles: 8, _fn: Instruction::op_cbaa },
            0xCBAB => Instruction { opcode, mnemonic: "RES 5,E", size: 2, cycles: 8, _fn: Instruction::op_cbab },
            0xCBAC => Instruction { opcode, mnemonic: "RES 5,F", size: 2, cycles: 8, _fn: Instruction::op_cbac },
            0xCBAD => Instruction { opcode, mnemonic: "RES 5,H", size: 2, cycles: 8, _fn: Instruction::op_cbad },
            0xCBAE => Instruction { opcode, mnemonic: "RES 5,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbae },
            0xCBAF => Instruction { opcode, mnemonic: "RES 5,A", size: 2, cycles: 8, _fn: Instruction::op_cbaf },

            0xCBB0 => Instruction { opcode, mnemonic: "RES 6,B", size: 2, cycles: 8, _fn: Instruction::op_cbb0 },
            0xCBB1 => Instruction { opcode, mnemonic: "RES 6,C", size: 2, cycles: 8, _fn: Instruction::op_cbb1 },
            0xCBB2 => Instruction { opcode, mnemonic: "RES 6,D", size: 2, cycles: 8, _fn: Instruction::op_cbb2 },
            0xCBB3 => Instruction { opcode, mnemonic: "RES 6,E", size: 2, cycles: 8, _fn: Instruction::op_cbb3 },
            0xCBB4 => Instruction { opcode, mnemonic: "RES 6,H", size: 2, cycles: 8, _fn: Instruction::op_cbb4 },
            0xCBB5 => Instruction { opcode, mnemonic: "RES 6,L", size: 2, cycles: 8, _fn: Instruction::op_cbb5 },
            0xCBB6 => Instruction { opcode, mnemonic: "RES 6,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbb6 },
            0xCBB7 => Instruction { opcode, mnemonic: "RES 6,A", size: 2, cycles: 8, _fn: Instruction::op_cbb7 },
            0xCBB8 => Instruction { opcode, mnemonic: "RES 7,B", size: 2, cycles: 8, _fn: Instruction::op_cbb8 },
            0xCBB9 => Instruction { opcode, mnemonic: "RES 7,C", size: 2, cycles: 8, _fn: Instruction::op_cbb9 },
            0xCBBA => Instruction { opcode, mnemonic: "RES 7,D", size: 2, cycles: 8, _fn: Instruction::op_cbba },
            0xCBBB => Instruction { opcode, mnemonic: "RES 7,E", size: 2, cycles: 8, _fn: Instruction::op_cbbb },
            0xCBBC => Instruction { opcode, mnemonic: "RES 7,F", size: 2, cycles: 8, _fn: Instruction::op_cbbc },
            0xCBBD => Instruction { opcode, mnemonic: "RES 7,H", size: 2, cycles: 8, _fn: Instruction::op_cbbd },
            0xCBBE => Instruction { opcode, mnemonic: "RES 7,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbbe },
            0xCBBF => Instruction { opcode, mnemonic: "RES 7,A", size: 2, cycles: 8, _fn: Instruction::op_cbbf },

            0xCBC0 => Instruction { opcode, mnemonic: "SET 0,B", size: 2, cycles: 8, _fn: Instruction::op_cbc0 },
            0xCBC1 => Instruction { opcode, mnemonic: "SET 0,C", size: 2, cycles: 8, _fn: Instruction::op_cbc1 },
            0xCBC2 => Instruction { opcode, mnemonic: "SET 0,D", size: 2, cycles: 8, _fn: Instruction::op_cbc2 },
            0xCBC3 => Instruction { opcode, mnemonic: "SET 0,E", size: 2, cycles: 8, _fn: Instruction::op_cbc3 },
            0xCBC4 => Instruction { opcode, mnemonic: "SET 0,H", size: 2, cycles: 8, _fn: Instruction::op_cbc4 },
            0xCBC5 => Instruction { opcode, mnemonic: "SET 0,L", size: 2, cycles: 8, _fn: Instruction::op_cbc5 },
            0xCBC6 => Instruction { opcode, mnemonic: "SET 0,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbc6 },
            0xCBC7 => Instruction { opcode, mnemonic: "SET 0,A", size: 2, cycles: 8, _fn: Instruction::op_cbc7 },
            0xCBC8 => Instruction { opcode, mnemonic: "SET 1,B", size: 2, cycles: 8, _fn: Instruction::op_cbc8 },
            0xCBC9 => Instruction { opcode, mnemonic: "SET 1,C", size: 2, cycles: 8, _fn: Instruction::op_cbc9 },
            0xCBCA => Instruction { opcode, mnemonic: "SET 1,D", size: 2, cycles: 8, _fn: Instruction::op_cbca },
            0xCBCB => Instruction { opcode, mnemonic: "SET 1,E", size: 2, cycles: 8, _fn: Instruction::op_cbcb },
            0xCBCC => Instruction { opcode, mnemonic: "SET 1,F", size: 2, cycles: 8, _fn: Instruction::op_cbcc },
            0xCBCD => Instruction { opcode, mnemonic: "SET 1,H", size: 2, cycles: 8, _fn: Instruction::op_cbcd },
            0xCBCE => Instruction { opcode, mnemonic: "SET 1,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbce },
            0xCBCF => Instruction { opcode, mnemonic: "SET 1,A", size: 2, cycles: 8, _fn: Instruction::op_cbcf },

            0xCBD0 => Instruction { opcode, mnemonic: "SET 2,B", size: 2, cycles: 8, _fn: Instruction::op_cbd0 },
            0xCBD1 => Instruction { opcode, mnemonic: "SET 2,C", size: 2, cycles: 8, _fn: Instruction::op_cbd1 },
            0xCBD2 => Instruction { opcode, mnemonic: "SET 2,D", size: 2, cycles: 8, _fn: Instruction::op_cbd2 },
            0xCBD3 => Instruction { opcode, mnemonic: "SET 2,E", size: 2, cycles: 8, _fn: Instruction::op_cbd3 },
            0xCBD4 => Instruction { opcode, mnemonic: "SET 2,H", size: 2, cycles: 8, _fn: Instruction::op_cbd4 },
            0xCBD5 => Instruction { opcode, mnemonic: "SET 2,L", size: 2, cycles: 8, _fn: Instruction::op_cbd5 },
            0xCBD6 => Instruction { opcode, mnemonic: "SET 2,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbd6 },
            0xCBD7 => Instruction { opcode, mnemonic: "SET 2,A", size: 2, cycles: 8, _fn: Instruction::op_cbd7 },
            0xCBD8 => Instruction { opcode, mnemonic: "SET 3,B", size: 2, cycles: 8, _fn: Instruction::op_cbd8 },
            0xCBD9 => Instruction { opcode, mnemonic: "SET 3,C", size: 2, cycles: 8, _fn: Instruction::op_cbd9 },
            0xCBDA => Instruction { opcode, mnemonic: "SET 3,D", size: 2, cycles: 8, _fn: Instruction::op_cbda },
            0xCBDB => Instruction { opcode, mnemonic: "SET 3,E", size: 2, cycles: 8, _fn: Instruction::op_cbdb },
            0xCBDC => Instruction { opcode, mnemonic: "SET 3,F", size: 2, cycles: 8, _fn: Instruction::op_cbdc },
            0xCBDD => Instruction { opcode, mnemonic: "SET 3,H", size: 2, cycles: 8, _fn: Instruction::op_cbdd },
            0xCBDE => Instruction { opcode, mnemonic: "SET 3,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbde },
            0xCBDF => Instruction { opcode, mnemonic: "SET 3,A", size: 2, cycles: 8, _fn: Instruction::op_cbdf },

            0xCBE0 => Instruction { opcode, mnemonic: "SET 4,B", size: 2, cycles: 8, _fn: Instruction::op_cbe0 },
            0xCBE1 => Instruction { opcode, mnemonic: "SET 4,C", size: 2, cycles: 8, _fn: Instruction::op_cbe1 },
            0xCBE2 => Instruction { opcode, mnemonic: "SET 4,D", size: 2, cycles: 8, _fn: Instruction::op_cbe2 },
            0xCBE3 => Instruction { opcode, mnemonic: "SET 4,E", size: 2, cycles: 8, _fn: Instruction::op_cbe3 },
            0xCBE4 => Instruction { opcode, mnemonic: "SET 4,H", size: 2, cycles: 8, _fn: Instruction::op_cbe4 },
            0xCBE5 => Instruction { opcode, mnemonic: "SET 4,L", size: 2, cycles: 8, _fn: Instruction::op_cbe5 },
            0xCBE6 => Instruction { opcode, mnemonic: "SET 4,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbe6 },
            0xCBE7 => Instruction { opcode, mnemonic: "SET 4,A", size: 2, cycles: 8, _fn: Instruction::op_cbe7 },
            0xCBE8 => Instruction { opcode, mnemonic: "SET 5,B", size: 2, cycles: 8, _fn: Instruction::op_cbe8 },
            0xCBE9 => Instruction { opcode, mnemonic: "SET 5,C", size: 2, cycles: 8, _fn: Instruction::op_cbe9 },
            0xCBEA => Instruction { opcode, mnemonic: "SET 5,D", size: 2, cycles: 8, _fn: Instruction::op_cbea },
            0xCBEB => Instruction { opcode, mnemonic: "SET 5,E", size: 2, cycles: 8, _fn: Instruction::op_cbeb },
            0xCBEC => Instruction { opcode, mnemonic: "SET 5,F", size: 2, cycles: 8, _fn: Instruction::op_cbec },
            0xCBED => Instruction { opcode, mnemonic: "SET 5,H", size: 2, cycles: 8, _fn: Instruction::op_cbed },
            0xCBEE => Instruction { opcode, mnemonic: "SET 5,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbee },
            0xCBEF => Instruction { opcode, mnemonic: "SET 5,A", size: 2, cycles: 8, _fn: Instruction::op_cbef },

            0xCBF0 => Instruction { opcode, mnemonic: "SET 6,B", size: 2, cycles: 8, _fn: Instruction::op_cbf0 },
            0xCBF1 => Instruction { opcode, mnemonic: "SET 6,C", size: 2, cycles: 8, _fn: Instruction::op_cbf1 },
            0xCBF2 => Instruction { opcode, mnemonic: "SET 6,D", size: 2, cycles: 8, _fn: Instruction::op_cbf2 },
            0xCBF3 => Instruction { opcode, mnemonic: "SET 6,E", size: 2, cycles: 8, _fn: Instruction::op_cbf3 },
            0xCBF4 => Instruction { opcode, mnemonic: "SET 6,H", size: 2, cycles: 8, _fn: Instruction::op_cbf4 },
            0xCBF5 => Instruction { opcode, mnemonic: "SET 6,L", size: 2, cycles: 8, _fn: Instruction::op_cbf5 },
            0xCBF6 => Instruction { opcode, mnemonic: "SET 6,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbf6 },
            0xCBF7 => Instruction { opcode, mnemonic: "SET 6,A", size: 2, cycles: 8, _fn: Instruction::op_cbf7 },
            0xCBF8 => Instruction { opcode, mnemonic: "SET 7,B", size: 2, cycles: 8, _fn: Instruction::op_cbf8 },
            0xCBF9 => Instruction { opcode, mnemonic: "SET 7,C", size: 2, cycles: 8, _fn: Instruction::op_cbf9 },
            0xCBFA => Instruction { opcode, mnemonic: "SET 7,D", size: 2, cycles: 8, _fn: Instruction::op_cbfa },
            0xCBFB => Instruction { opcode, mnemonic: "SET 7,E", size: 2, cycles: 8, _fn: Instruction::op_cbfb },
            0xCBFC => Instruction { opcode, mnemonic: "SET 7,F", size: 2, cycles: 8, _fn: Instruction::op_cbfc },
            0xCBFD => Instruction { opcode, mnemonic: "SET 7,H", size: 2, cycles: 8, _fn: Instruction::op_cbfd },
            0xCBFE => Instruction { opcode, mnemonic: "SET 7,(HL)", size: 2, cycles: 16, _fn: Instruction::op_cbfe },
            0xCBFF => Instruction { opcode, mnemonic: "SET 7,A", size: 2, cycles: 8, _fn: Instruction::op_cbff },

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

    fn get_source_value_8(cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src) -> u8 {
        match source {
            Src::D8 => args[0],

            Src::A8 => mmu.read_8(0xFF00 | args[0] as u16),
            Src::A16 => mmu.read_8(((args[1] as u16) << 8) | (args[0] as u16)),

            Src::A => cpu.registers.get_byte(CpuRegIndex::A),
            Src::B => cpu.registers.get_byte(CpuRegIndex::B),
            Src::C => cpu.registers.get_byte(CpuRegIndex::C),
            Src::D => cpu.registers.get_byte(CpuRegIndex::D),
            Src::E => cpu.registers.get_byte(CpuRegIndex::E),
            Src::F => cpu.registers.get_byte(CpuRegIndex::F),
            Src::H => cpu.registers.get_byte(CpuRegIndex::H),
            Src::L => cpu.registers.get_byte(CpuRegIndex::L),

            Src::Ca => mmu.read_8(0xFF00 | cpu.registers.get_byte(CpuRegIndex::C) as u16),

            Src::AFa => mmu.read_8(cpu.registers.get_word(CpuRegIndex::AF)),
            Src::BCa => mmu.read_8(cpu.registers.get_word(CpuRegIndex::BC)),
            Src::DEa => mmu.read_8(cpu.registers.get_word(CpuRegIndex::DE)),
            Src::HLa => mmu.read_8(cpu.registers.get_word(CpuRegIndex::HL)),

            _ => panic!("Unsupported 8-bit source: {:?}", source),
        }
    }

    fn get_source_value_16(cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src) -> u16 {
        match source {
            Src::D16 => ((args[1] as u16) << 8) | (args[0] as u16),

            Src::AF => cpu.registers.get_word(CpuRegIndex::AF),
            Src::BC => cpu.registers.get_word(CpuRegIndex::BC),
            Src::DE => cpu.registers.get_word(CpuRegIndex::DE),
            Src::HL => cpu.registers.get_word(CpuRegIndex::HL),
            Src::PC => cpu.registers.get_word(CpuRegIndex::PC),
            Src::SP => cpu.registers.get_word(CpuRegIndex::SP),

            _ => panic!("Unsupported 16-bit source: {:?}", source),
        }
    }

    fn set_target_value_8(cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, value: u8) {
        match target {
            Src::A8 => mmu.write_8(0xFF00 | args[0] as u16, value),
            Src::A16 => mmu.write_8(((args[1] as u16) << 8) | (args[0] as u16), value),

            Src::A => cpu.registers.set_byte(CpuRegIndex::A, value),
            Src::B => cpu.registers.set_byte(CpuRegIndex::B, value),
            Src::C => cpu.registers.set_byte(CpuRegIndex::C, value),
            Src::D => cpu.registers.set_byte(CpuRegIndex::D, value),
            Src::E => cpu.registers.set_byte(CpuRegIndex::E, value),
            Src::F => cpu.registers.set_byte(CpuRegIndex::F, value),
            Src::H => cpu.registers.set_byte(CpuRegIndex::H, value),
            Src::L => cpu.registers.set_byte(CpuRegIndex::L, value),

            Src::Ca => mmu.write_8(0xFF00 | cpu.registers.get_byte(CpuRegIndex::C) as u16, value),

            Src::AFa => mmu.write_8(cpu.registers.get_word(CpuRegIndex::AF), value),
            Src::BCa => mmu.write_8(cpu.registers.get_word(CpuRegIndex::BC), value),
            Src::DEa => mmu.write_8(cpu.registers.get_word(CpuRegIndex::DE), value),
            Src::HLa => mmu.write_8(cpu.registers.get_word(CpuRegIndex::HL), value),

            _ => panic!("Unsupported 8-bit target: {:?}", target),
        }
    }

    fn set_target_value_16(cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, value: u16) {
        match target {
            Src::A16 => mmu.write_16(((args[1] as u16) << 8) | (args[0] as u16), value, Endianness::BIG),

            Src::AF => cpu.registers.set_word(CpuRegIndex::AF, value),
            Src::BC => cpu.registers.set_word(CpuRegIndex::BC, value),
            Src::DE => cpu.registers.set_word(CpuRegIndex::DE, value),
            Src::HL => cpu.registers.set_word(CpuRegIndex::HL, value),
            Src::PC => cpu.registers.set_word(CpuRegIndex::PC, value),
            Src::SP => cpu.registers.set_word(CpuRegIndex::SP, value),

            _ => panic!("Unsupported 16-bit target: {:?}", target),
        }
    }

    /// PUSH
    fn push(cpu: &mut Cpu, mmu: &mut Mmu, source_register: CpuRegIndex) {
        // SP=SP-2
        cpu.registers.decrement(CpuRegIndex::SP, 2);
        // (SP)=r16
        let value = cpu.registers.get_word(source_register);
        let target_address = cpu.registers.get_word(CpuRegIndex::SP);
        mmu.write_16(target_address, value, Endianness::BIG);
    }

    /// POP
    fn pop(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, target_register: CpuRegIndex) -> i16 {
        // r16=(SP)
        let source_address = cpu.registers.get_word(CpuRegIndex::SP);
        let value = mmu.read_16(source_address, Endianness::BIG);
        cpu.registers.set_word(target_register, value);
        // SP=SP+2
        cpu.registers.increment(CpuRegIndex::SP, 2);
        self.cycles
    }

    /// JUMP
    fn jump(cpu: &mut Cpu, address: u16) {
        cpu.registers.set_word(CpuRegIndex::PC, address);
    }

    fn relative_jump(&mut self, cpu: &mut Cpu, d8: u8) {
        let jump_by = alu::signed_8(d8);

        if jump_by < 0 {
            cpu.registers.decrement(CpuRegIndex::PC, (-1 * jump_by) as u16);
        } else {
            cpu.registers.increment(CpuRegIndex::PC, jump_by as u16);
        }

        self.cycles += 4;
    }

    /// CALL
    pub(crate) fn call(cpu: &mut Cpu, mmu: &mut Mmu, address: u16) {
        Instruction::push(cpu, mmu, CpuRegIndex::PC);
        Instruction::jump(cpu, address);
    }

    /// LD
    fn ld_8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let value = Instruction::get_source_value_8(cpu, mmu, args, source);
        Instruction::set_target_value_8(cpu, mmu, args, target, value);
        self.cycles
    }

    fn ld_16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let value = Instruction::get_source_value_16(cpu, mmu, args, source);
        Instruction::set_target_value_16(cpu, mmu, args, target, value);
        self.cycles
    }

    /// ADD
    fn add_8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let a = Instruction::get_source_value_8(cpu, mmu, args, target);
        let b = Instruction::get_source_value_8(cpu, mmu, args, source);
        let (result, flags) = alu::add_8(a, b);
        Instruction::set_target_value_8(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    fn add_hl(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src) -> i16 {
        let a = Instruction::get_source_value_16(cpu, mmu, args, Src::HL);
        let b = Instruction::get_source_value_16(cpu, mmu, args, source);
        let original_zero = cpu.registers.get_flags().zero;
        let (result, mut flags) = alu::add_16(a, b);
        flags.zero = original_zero;
        Instruction::set_target_value_16(cpu, mmu, args, Src::HL, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    fn adc_8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let a = Instruction::get_source_value_8(cpu, mmu, args, target);
        let b = Instruction::get_source_value_8(cpu, mmu, args, source);
        let original_flags = cpu.registers.get_flags();
        let (result, flags) = alu::adc_8(a, b, original_flags);
        Instruction::set_target_value_8(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// SUB
    fn sub_8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let a = Instruction::get_source_value_8(cpu, mmu, args, target);
        let b = Instruction::get_source_value_8(cpu, mmu, args, source);
        let (result, flags) = alu::subtract_8(a, b);
        Instruction::set_target_value_8(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    fn sbc_8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src, source: Src) -> i16 {
        let a = Instruction::get_source_value_8(cpu, mmu, args, target);
        let b = Instruction::get_source_value_8(cpu, mmu, args, source);
        let original_flags = cpu.registers.get_flags();
        let (result, flags) = alu::sbc_8(a, b, original_flags);
        Instruction::set_target_value_8(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// INC/DEC 8 bit -- Affects all flags except carry flag
    fn increment_8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src) -> i16 {
        let value = Instruction::get_source_value_8(cpu, mmu, args, target);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::increment_8(value, original_carry);
        Instruction::set_target_value_8(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    fn decrement_8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src) -> i16 {
        let value = Instruction::get_source_value_8(cpu, mmu, args, target);
        let original_carry = cpu.registers.get_flags().carry;
        let (result, flags) = alu::decrement_8(value, original_carry);
        Instruction::set_target_value_8(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// INC/DEC 16 bit -- Does not affect flags
    fn increment_16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src) -> i16 {
        let value = Instruction::get_source_value_16(cpu, mmu, args, target);
        let result = alu::increment_16(value);
        Instruction::set_target_value_16(cpu, mmu, args, target, result);
        self.cycles
    }

    fn decrement_16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src) -> i16 {
        let value = Instruction::get_source_value_16(cpu, mmu, args, target);
        let result = alu::decrement_16(value);
        Instruction::set_target_value_16(cpu, mmu, args, target, result);
        self.cycles
    }

    /// AND
    fn and(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let b = Instruction::get_source_value_8(cpu, mmu, args, source);
        let (result, flags) = alu::and_8(a, b);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// OR
    fn or(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let b = Instruction::get_source_value_8(cpu, mmu, args, source);
        let (result, flags) = alu::or_8(a, b);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// XOR
    fn xor(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let b = Instruction::get_source_value_8(cpu, mmu, args, source);
        let (result, flags) = alu::xor_8(a, b);
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// SLA, SRA, SRL
    fn shift(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], target: Src,
            is_left: bool, is_arithmetic: bool) -> i16 {
        let value = Instruction::get_source_value_8(cpu, mmu, args, target);
        let (result, flags) = if is_left {
            alu::shift_left(value)
        } else {
            alu::shift_right(value, is_arithmetic)
        };
        Instruction::set_target_value_8(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// RL, RR
    /// RLA, RRA
    fn rotate(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8],
            target: Src, is_left: bool, reset_zero: bool) -> i16 {
        let value = Instruction::get_source_value_8(cpu, mmu, args, target);
        let original_flags = cpu.registers.get_flags();
        let (result, mut flags) = if is_left {
            alu::rotate_left(value, original_flags.carry)
        } else {
            alu::rotate_right(value, original_flags.carry)
        };
        if reset_zero {
            flags.zero = false;
        }
        Instruction::set_target_value_8(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// RLC, RRC
    /// RLCA, RRCA
    fn rotate_circular(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8],
            target: Src, is_left: bool, reset_zero: bool) -> i16 {
        let value = Instruction::get_source_value_8(cpu, mmu, args, target);
        let (result, mut flags) = if is_left {
            alu::rotate_left_circular(value)
        } else {
            alu::rotate_right_circular(value)
        };
        if reset_zero {
            flags.zero = false;
        }
        Instruction::set_target_value_8(cpu, mmu, args, target, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// BIT
    fn bit(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src, n: u8) -> i16 {
        let value = Instruction::get_source_value_8(cpu, mmu, args, source);
        let mut flags = cpu.registers.get_flags();
        flags.zero = value & (1 << n) == 0;
        flags.subtract = false;
        flags.half_carry = true;
        cpu.registers.set_flags(flags);
        self .cycles
    }

    /// SET, RES
    fn set(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src, n: u8, set: bool) -> i16 {
        let mut value = Instruction::get_source_value_8(cpu, mmu, args, source);
        value ^= value & (1 << n);
        value |= (set as u8) << n;
        Instruction::set_target_value_8(cpu, mmu, args, source, value);
        self .cycles
    }

    /// SWAP
    fn swap(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src) -> i16 {
        let mut value = Instruction::get_source_value_8(cpu, mmu, args, source);
        value = ((value & 0xF0) >> 4) | ((value & 0x0F) << 4);
        Instruction::set_target_value_8(cpu, mmu, args, source, value);
        let flags = Flags{
            zero: value == 0,
            subtract: false,
            half_carry: false,
            carry: false,
        };
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// CP
    fn cp(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8], source: Src) -> i16 {
        let a = cpu.registers.get_byte(CpuRegIndex::A);
        let b = Instruction::get_source_value_8(cpu, mmu, args, source);
        let (_, flags) = alu::subtract_8(a, b);
        cpu.registers.set_flags(flags);
        self.cycles
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
        self.ld_16(cpu, mmu, args, Src::BC, Src::D16)
    }

    /// LD (BC),A
    /// 1 8
    /// - - - -
    fn op_0002(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::BCa, Src::A)
    }

    /// INC BC
    /// 1 8
    /// - - - -
    fn op_0003(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_16(cpu, mmu, args, Src::BC)
    }

    /// INC B
    /// 1 4
    /// Z 0 H -
    fn op_0004(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_8(cpu, mmu, args, Src::B)
    }

    /// DEC B
    /// 1 4
    /// Z 1 H -
    fn op_0005(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_8(cpu, mmu, args, Src::B)
    }

    /// LD B,d8
    /// 2 8
    /// - - - -
    fn op_0006(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::B, Src::D8)
    }

    /// RLCA
    /// 1 4
    /// 0 0 0 C
    fn op_0007(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::A, true, true)
    }

    /// LD (a16),SP
    /// 3 20
    /// - - - -
    fn op_0008(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_16(cpu, mmu, args, Src::A16, Src::SP)
    }

    /// ADD HL,BC
    /// 1 8
    /// - 0 H C
    fn op_0009(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_hl(cpu, mmu, args, Src::BC)
    }

    /// LD A,(BC)
    /// 1  8
    /// - - - -
    fn op_000a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::BCa)
    }

    /// DEC BC 
    /// 1 8 
    /// - - - -
    fn op_000b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_16(cpu, mmu, args, Src::BC)
    }
    
    /// INC C
    /// 1 4
    /// Z 0 H -
    fn op_000c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_8(cpu, mmu, args, Src::C)
    }

    /// DEC C
    /// 1 4
    /// Z 1 H -
    fn op_000d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_8(cpu, mmu, args, Src::C)
    }

    /// LD C,d8
    /// 2 8
    /// - - - -
    fn op_000e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::C, Src::D8)
    }

    /// RRCA
    /// 1 4
    /// 0 0 0 C
    fn op_000f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::A, false, true)
    }

    /// STOP
    /// 2 4
    /// - - - -
    fn op_0010(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cycles
    }

    /// LD DE,d16
    /// 3 12
    /// - - - -
    fn op_0011(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_16(cpu, mmu, args, Src::DE, Src::D16)
    }

    /// LD (DE),A
    /// 1 8
    /// - - - -
    fn op_0012(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::DEa, Src::A)
    }

    /// INC DE
    /// 1 8
    /// - - - -
    fn op_0013(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_16(cpu, mmu, args, Src::DE)
    }

    /// INC D
    /// 1 4
    /// Z 0 H -
    fn op_0014(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_8(cpu, mmu, args, Src::D)
    }

    /// DEC D
    /// 1 4
    /// Z 1 H -
    fn op_0015(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_8(cpu, mmu, args, Src::D)
    }

    /// LD D,d8
    /// 2 8
    /// - - - -
    fn op_0016(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::D, Src::D8)
    }

    /// RLA
    /// 1 4
    /// Z 0 0 C
    fn op_0017(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::A, true, true)
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
        self.add_hl(cpu, mmu, args, Src::DE)
    }

    /// LD A,(DE)
    /// 1 8
    /// - - - -
    fn op_001a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::DEa)
    }

    /// DEC DE
    /// 1 8
    /// - - - -
    fn op_001b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_16(cpu, mmu, args, Src::DE)
    }

    /// INC E
    /// 1 4
    /// Z 0 H -
    fn op_001c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_8(cpu, mmu, args, Src::E)
    }

    /// DEC E
    /// 1 4
    /// Z 1 H -
    fn op_001d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_8(cpu, mmu, args, Src::E)
    }

    /// LD E,d8
    /// 2 8
    /// - - - -
    fn op_001e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::E, Src::D8)
    }

    /// RRA
    /// 1 4
    /// 0 0 0 C
    fn op_001f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::A, false, true)
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
        self.ld_16(cpu, mmu, args, Src::HL, Src::D16)
    }

    /// LDI (HL),A
    /// aka LD (HL+),A
    /// 1 8
    /// - - - -
    fn op_0022(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::A);
        cpu.registers.increment(CpuRegIndex::HL, 1);
        self.cycles
    }

    /// INC HL
    /// 1 8
    /// - - - -
    fn op_0023(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_16(cpu, mmu, args, Src::HL)
    }

    /// INC H
    /// 1 4
    /// Z 0 H -
    fn op_0024(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
         self.increment_8(cpu, mmu, args, Src::H)
    }

    /// DEC H
    /// 1 4
    /// Z 1 H -
    fn op_0025(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_8(cpu, mmu, args, Src::H)
    }

    /// LD H,d8
    /// 2 8
    /// - - - -
    fn op_0026(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::H, Src::D8)
    }

    /// DAA
    /// 1 4
    /// Z - 0 C
    fn op_0027(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let (result, flags) = alu::daa(cpu.registers.get_byte(CpuRegIndex::A), cpu.registers.get_flags());
        cpu.registers.set_byte(CpuRegIndex::A, result);
        cpu.registers.set_flags(flags);
        self.cycles
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
        self.add_hl(cpu, mmu, args, Src::HL)
    }

    /// LDI A,(HL)
    /// aka LD A, (HL+)
    /// 1 8
    /// - - - -
    fn op_002a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::HLa);
        cpu.registers.increment(CpuRegIndex::HL, 1);
        self.cycles
    }

    /// DEC HL
    /// 1 8
    /// - - - -
    fn op_002b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_16(cpu, mmu, args, Src::HL)
    }

    /// INC L
    /// 1 4
    /// Z 0 H -
    fn op_002c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_8(cpu, mmu, args, Src::L)
    }

    /// DEC L
    /// 1 4
    /// Z 1 H -
    fn op_002d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_8(cpu, mmu, args, Src::L)
    }

    /// LD L,d8
    /// 2 8
    /// - - - -
    fn op_002e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::L, Src::D8)
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
        self.ld_16(cpu, mmu, args, Src::SP, Src::D16)
    }

    /// LDD (HL),A
    /// aka LD (HL-),A
    /// 1 8
    /// - - - -
    fn op_0032(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::A);
        cpu.registers.decrement(CpuRegIndex::HL, 1);
        self.cycles
    }

    /// INC SP
    /// 1 8
    /// - - - -
    fn op_0033(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_16(cpu, mmu, args, Src::SP)
    }

    /// INC (HL)
    /// 1 12
    /// Z 0 H -
    fn op_0034(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_8(cpu, mmu, args, Src::HLa)
    }

    /// DEC (HL)
    /// 1 12
    /// Z 1 H -
    fn op_0035(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_8(cpu, mmu, args, Src::HLa)
    }

    /// LD (HL),d8
    /// 2 12
    /// - - - -
    fn op_0036(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::D8)
    }

    /// SCF
    /// 1 4
    /// - 0 0 1
    fn op_0037(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let original_flags = cpu.registers.get_flags();
        let flags = Flags {
            zero: original_flags.zero,
            subtract: false,
            half_carry: false,
            carry: true,
        };
        cpu.registers.set_flags(flags);
        self.cycles
    }

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
        self.add_hl(cpu, mmu, args, Src::SP)
    }

    /// LDD A,(HL)
    /// aka LD A,(HL-)
    /// 1 8
    /// - - - -
     fn op_003a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::HLa);
        cpu.registers.decrement(CpuRegIndex::HL, 1);
        self.cycles
    }

    /// DEC SP
    /// 1 8
    /// - - - -
    fn op_003b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_16(cpu, mmu, args, Src::SP)
    }

    /// INC A
    /// 1 4
    /// Z 0 H -
    fn op_003c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.increment_8(cpu, mmu, args, Src::A)
    }

    /// DEC A
    /// 1 4
    /// Z 1 H -
    fn op_003d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.decrement_8(cpu, mmu, args, Src::A)
    }

    /// LD A,d8
    /// 2 8
    /// - - - -
    fn op_003e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::D8)
    }

    /// CCF
    /// 1 4
    /// - 0 0 C
    fn op_003f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let original_flags = cpu.registers.get_flags();
        let flags = Flags {
            zero: original_flags.zero,
            subtract: false,
            half_carry: false,
            carry: !original_flags.carry,
        };
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// LD B,B
    /// 1 4
    /// - - - -
    fn op_0040(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::B, Src::B)
    }

    /// LD B,C
    /// 1 4
    /// - - - -
    fn op_0041(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::B, Src::C)
    }

    /// LD B,D
    /// 1 4
    /// - - - -
    fn op_0042(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::B, Src::D)
    }

    /// LD B,E
    /// 1 4
    /// - - - -
    fn op_0043(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::B, Src::E)
    }

    /// LD B,H
    /// 1 4
    /// - - - -
    fn op_0044(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::B, Src::H)
    }

    /// LD B,L
    /// 1 4
    /// - - - -
    fn op_0045(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::B, Src::L)
    }

    /// LD B,(HL)
    /// 1 8
    /// - - - -
    fn op_0046(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::B, Src::HLa)
    }

    /// LD B,A
    /// 1 4
    /// - - - -
    fn op_0047(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::B, Src::A)
    }

    /// LD C,B
    /// 1 4
    /// - - - -
    fn op_0048(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::C, Src::B)
    }

    /// LD C,C
    /// 1 4
    /// - - - -
    fn op_0049(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::C, Src::C)
    }

    /// LD C,D
    /// 1 4
    /// - - - -
    fn op_004a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::C, Src::D)
    }

    /// LD C,E
    /// 1 4
    /// - - - -
    fn op_004b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::C, Src::E)
    }

    /// LD C,H
    /// 1 4
    /// - - - -
    fn op_004c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::C, Src::H)
    }

    /// LD C,L
    /// 1 4
    /// - - - -
    fn op_004d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::C, Src::L)
    }

    /// LD C,(HL)
    /// 1 8
    /// - - - -
    fn op_004e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::C, Src::HLa)
    }

    /// LD C,A
    /// 1 4
    /// - - - -
    fn op_004f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::C, Src::A)
    }

    /// LD D,B
    /// 1 4
    /// - - - -
    fn op_0050(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::D, Src::B)
    }

    /// LD D,C
    /// 1 4
    /// - - - -
    fn op_0051(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::D, Src::C)
    }

    /// LD D,D
    /// 1 4
    /// - - - -
    fn op_0052(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::D, Src::D)
    }

    /// LD D,E
    /// 1 4
    /// - - - -
    fn op_0053(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::D, Src::E)
    }

    /// LD D,H
    /// 1 4
    /// - - - -
    fn op_0054(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::D, Src::H)
    }

    /// LD D,L
    /// 1 4
    /// - - - -
    fn op_0055(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::D, Src::L)
    }

    /// LD D,(HL)
    /// 1 8
    /// - - - -
    fn op_0056(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::D, Src::HLa)
    }

    /// LD D,A
    /// 1 4
    /// - - - -
    fn op_0057(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::D, Src::A)
    }

    /// LD E,B
    /// 1 4
    /// - - - -
    fn op_0058(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::E, Src::B)
    }

    /// LD E,C
    /// 1 4
    /// - - - -
    fn op_0059(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::E, Src::C)
    }

    /// LD E,D
    /// 1 4
    /// - - - -
    fn op_005a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::E, Src::D)
    }

    /// LD E,E
    /// 1 4
    /// - - - -
    fn op_005b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::E, Src::E)
    }

    /// LD E,H
    /// 1 4
    /// - - - -
    fn op_005c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::E, Src::H)
    }

    /// LD E,L
    /// 1 4
    /// - - - -
    fn op_005d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::E, Src::L)
    }

    /// LD E,(HL)
    /// 1 8
    /// - - - -
    fn op_005e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::E, Src::HLa)
    }

    /// LD E,A
    /// 1 4
    /// - - - -
    fn op_005f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::E, Src::A)
    }

    /// LD H,B
    /// 1 4
    /// - - - -
    fn op_0060(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::H, Src::B)
    }

    /// LD H,C
    /// 1 4
    /// - - - -
    fn op_0061(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::H, Src::C)
    }

    /// LD H,D
    /// 1 4
    /// - - - -
    fn op_0062(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::H, Src::D)
    }

    /// LD H,E
    /// 1 4
    /// - - - -
    fn op_0063(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::H, Src::E)
    }

    /// LD H,H
    /// 1 4
    /// - - - -
    fn op_0064(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::H, Src::H)
    }

    /// LD H,L
    /// 1 4
    /// - - - -
    fn op_0065(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::H, Src::L)
    }

    /// LD H,(HL)
    /// 1 8
    /// - - - -
    fn op_0066(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::H, Src::HLa)
    }

    /// LD H,A
    /// 1 4
    /// - - - -
    fn op_0067(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::H, Src::A)
    }

    /// LD L,B
    /// 1 4
    /// - - - -
    fn op_0068(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::L, Src::B)
    }

    /// LD L,C
    /// 1 4
    /// - - - -
    fn op_0069(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::L, Src::C)
    }

    /// LD L,D
    /// 1 4
    /// - - - -
    fn op_006a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::L, Src::D)
    }

    /// LD L,E
    /// 1 4
    /// - - - -
    fn op_006b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::L, Src::E)
    }

    /// LD L,H
    /// 1 4
    /// - - - -
    fn op_006c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::L, Src::H)
    }

    /// LD L,L
    /// 1 4
    /// - - - -
    fn op_006d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::L, Src::L)
    }

    /// LD L,(HL)
    /// 1 8
    /// - - - -
    fn op_006e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::L, Src::HLa)
    }

    /// LD L,A
    /// 1 4
    /// - - - -
    fn op_006f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::L, Src::A)
    }

    /// LD (HL),B
    /// 1 8
    /// - - - -
    fn op_0070(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::B)
    }

    /// LD (HL),C
    /// 1 8
    /// - - - -
    fn op_0071(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::C)
    }

    /// LD (HL),D
    /// 1 8
    /// - - - -
    fn op_0072(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::D)
    }

    /// LD (HL),E
    /// 1 8
    /// - - - -
    fn op_0073(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::E)
    }

    /// LD (HL),H
    /// 1 8
    /// - - - -
    fn op_0074(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::H)
    }

    /// LD (HL),L
    /// 1 8
    /// - - - -
    fn op_0075(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::L)
    }

    /// HALT
    /// 1 4
    /// - - - -
    fn op_0076(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.halt();
        self.cycles
    }

    /// LD (HL),A
    /// 1 8
    /// - - - -
    fn op_0077(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::HLa, Src::A)
    }

    /// LD A,B
    /// 1 4
    /// - - - -
    fn op_0078(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::B)
    }

    /// LD A,C
    /// 1 4
    /// - - - -
    fn op_0079(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::C)
    }

    /// LD A,D
    /// 1 4
    /// - - - -
    fn op_007a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::D)
    }

    /// LD A,E
    /// 1 4
    /// - - - -
    fn op_007b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::E)
    }

    /// LD A,H
    /// 1 4
    /// - - - -
    fn op_007c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::H)
    }

    /// LD A,L
    /// 1 4
    /// - - - -
    fn op_007d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::L)
    }

    /// LD A,(HL)
    /// 1 8
    /// - - - -
    fn op_007e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::HLa)
    }

    /// LD A,A
    /// 1 4
    /// - - - -
    fn op_007f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::A)
    }

    /// ADD A,B
    /// 1 4
    /// Z 0 H C
    fn op_0080(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::B)
    }

    /// ADD A,C
    /// 1 4
    /// Z 0 H C
    fn op_0081(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::C)
    }

    /// ADD A,D
    /// 1 4
    /// Z 0 H C
    fn op_0082(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::D)
    }

    /// ADD A,E
    /// 1 4
    /// Z 0 H C
    fn op_0083(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::E)
    }

    /// ADD A,H
    /// 1 4
    /// Z 0 H C
    fn op_0084(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::H)
    }

    /// ADD A,L
    /// 1 4
    /// Z 0 H C
    fn op_0085(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::L)
    }

    /// ADD A,(HL)
    /// 1 8
    /// Z 0 H C
    fn op_0086(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::HLa)
    }

    /// ADD A,A
    /// 1 4
    /// Z 0 H C
    fn op_0087(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::A)
    }

    /// ADC A,B
    /// 1 4
    /// Z 0 H C
    fn op_0088(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.adc_8(cpu, mmu, args, Src::A, Src::B)
    }

    /// ADC A,C
    /// 1 4
    /// Z 0 H C
    fn op_0089(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.adc_8(cpu, mmu, args, Src::A, Src::C)
    }

    /// ADC A,D
    /// 1 4
    /// Z 0 H C
    fn op_008a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.adc_8(cpu, mmu, args, Src::A, Src::D)
    }

    /// ADC A,E
    /// 1 4
    /// Z 0 H C
    fn op_008b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.adc_8(cpu, mmu, args, Src::A, Src::E)
    }

    /// ADC A,H
    /// 1 4
    /// Z 0 H C
    fn op_008c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.adc_8(cpu, mmu, args, Src::A, Src::H)
    }

    /// ADC A,L
    /// 1 4
    /// Z 0 H C
    fn op_008d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.adc_8(cpu, mmu, args, Src::A, Src::L)
    }

    /// ADC A,(HL)
    /// 1 8
    /// Z 0 H C
    fn op_008e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.adc_8(cpu, mmu, args, Src::A, Src::HLa)
    }

    /// ADC A,A
    /// 1 4
    /// Z 0 H C
    fn op_008f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.adc_8(cpu, mmu, args, Src::A, Src::A)
    }

    /// SUB A,B
    /// 1 4
    /// Z 1 H C
    fn op_0090(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::B)
    }

    /// SUB A,C
    /// 1 4
    /// Z 1 H C
    fn op_0091(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::C)
    }

    /// SUB A,D
    /// 1 4
    /// Z 1 H C
    fn op_0092(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::D)
    }

    /// SUB A,E
    /// 1 4
    /// Z 1 H C
    fn op_0093(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::E)
    }

    /// SUB A,H
    /// 1 4
    /// Z 1 H C
    fn op_0094(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::H)
    }

    /// SUB A,L
    /// 1 4
    /// Z 1 H C
    fn op_0095(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::L)
    }

    /// SUB A,(HL)
    /// 1 8
    /// Z 1 H C
    fn op_0096(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::HLa)
    }

    /// SUB A,A
    /// 1 4
    /// Z 1 H C
    fn op_0097(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::A)
    }

    /// SBC A,B
    /// 1 4
    /// Z 1 H C
    fn op_0098(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sbc_8(cpu, mmu, args, Src::A, Src::B)
    }

    /// SBC A,C
    /// 1 4
    /// Z 1 H C
    fn op_0099(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sbc_8(cpu, mmu, args, Src::A, Src::C)
    }

    /// SBC A,D
    /// 1 4
    /// Z 1 H C
    fn op_009a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sbc_8(cpu, mmu, args, Src::A, Src::D)
    }

    /// SBC A,E
    /// 1 4
    /// Z 1 H C
    fn op_009b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sbc_8(cpu, mmu, args, Src::A, Src::E)
    }


    /// SBC A,H
    /// 1 4
    /// Z 1 H C
    fn op_009c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sbc_8(cpu, mmu, args, Src::A, Src::H)
    }

    /// SBC A,L
    /// 1 4
    /// Z 1 H C
    fn op_009d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sbc_8(cpu, mmu, args, Src::A, Src::L)
    }

    /// SBC A,(HL)
    /// 1 8
    /// Z 1 H C
    fn op_009e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sbc_8(cpu, mmu, args, Src::A, Src::HLa)
    }

    /// SBC A,A
    /// 1 4
    /// Z 1 H C
    fn op_009f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sbc_8(cpu, mmu, args, Src::A, Src::A)
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

    /// XOR B
    /// 1 4
    /// Z 0 0 0
    fn op_00a8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.xor(cpu, mmu, args, Src::B)
    }

    /// XOR C
    /// 1 4
    /// Z 0 0 0
    fn op_00a9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.xor(cpu, mmu, args, Src::C)
    }

    /// XOR D
    /// 1 4
    /// Z 0 0 0
    fn op_00aa(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.xor(cpu, mmu, args, Src::D)
    }

    /// XOR E
    /// 1 4
    /// Z 0 0 0
    fn op_00ab(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.xor(cpu, mmu, args, Src::E)
    }

    /// XOR H
    /// 1 4
    /// Z 0 0 0
    fn op_00ac(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.xor(cpu, mmu, args, Src::H)
    }

    /// XOR L
    /// 1 4
    /// Z 0 0 0
    fn op_00ad(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.xor(cpu, mmu, args, Src::L)
    }

    /// XOR (HL)
    /// 1 4
    /// Z 0 0 0
    fn op_00ae(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.xor(cpu, mmu, args, Src::HLa)
    }

    /// XOR A
    /// 1 4
    /// Z 0 0 0
    fn op_00af(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.xor(cpu, mmu, args, Src::A)
    }

    /// OR B
    /// 1 4
    /// Z 0 0 0
    fn op_00b0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.or(cpu, mmu, args, Src::B)
    }

    /// OR C
    /// 1 4
    /// Z 0 0 0
    fn op_00b1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.or(cpu, mmu, args, Src::C)
    }

    /// OR D
    /// 1 4
    /// Z 0 0 0
    fn op_00b2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.or(cpu, mmu, args, Src::D)
    }

    /// OR E
    /// 1 4
    /// Z 0 0 0
    fn op_00b3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.or(cpu, mmu, args, Src::E)
    }

    /// OR H
    /// 1 4
    /// Z 0 0 0
    fn op_00b4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.or(cpu, mmu, args, Src::H)
    }

    /// OR L
    /// 1 4
    /// Z 0 0 0
    fn op_00b5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.or(cpu, mmu, args, Src::L)
    }

    /// OR (HL)
    /// 1 4
    /// Z 0 0 0
    fn op_00b6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.or(cpu, mmu, args, Src::HLa)
    }

    /// OR A
    /// 1 4
    /// Z 0 0 0
    fn op_00b7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.or(cpu, mmu, args, Src::A)
    }

    /// CP B
    /// 1 4
    /// Z 1 H C
    fn op_00b8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cp(cpu, mmu, args, Src::B)
    }

    /// CP C
    /// 1 4
    /// Z 1 H C
    fn op_00b9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cp(cpu, mmu, args, Src::C)
    }

    /// CP D
    /// 1 4
    /// Z 1 H C
    fn op_00ba(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cp(cpu, mmu, args, Src::D)
    }

    /// CP E
    /// 1 4
    /// Z 1 H C
    fn op_00bb(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cp(cpu, mmu, args, Src::E)
    }

    /// CP H
    /// 1 4
    /// Z 1 H C
    fn op_00bc(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cp(cpu, mmu, args, Src::H)
    }

    /// CP L
    /// 1 4
    /// Z 1 H C
    fn op_00bd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cp(cpu, mmu, args, Src::L)
    }

    /// CP (HL)
    /// 1 8
    /// Z 1 H C
    fn op_00be(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cp(cpu, mmu, args, Src::HLa)
    }

    /// CP A
    /// 1 4
    /// Z 1 H C
    fn op_00bf(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cp(cpu, mmu, args, Src::A)
    }

    /// RET NZ
    /// 1 8/20
    /// - - - -
    fn op_00c0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !cpu.registers.get_flags().zero {
            self.pop(cpu, mmu, CpuRegIndex::PC);
            self.cycles += 12;
        }
        self.cycles
    }

    /// POP BC
    /// 1 12
    /// - - - -
    fn op_00c1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.pop(cpu, mmu, CpuRegIndex::BC)
    }

    /// JP NZ,a16
    /// 3 12/16
    /// - - - -
    fn op_00c2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !cpu.registers.get_flags().zero {
            let address = ((args[1] as u16) << 8) | (args[0] as u16);
            Instruction::jump(cpu, address);
            self.cycles += 4;
        }
        self.cycles
    }

    /// JP a16
    /// 3 16
    /// - - - -
    fn op_00c3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = ((args[1] as u16) << 8) | (args[0] as u16);
        Instruction::jump(cpu, address);
        self.cycles
    }

    /// CALL NZ,a16
    /// 3 12/24
    /// - - - -
    fn op_00c4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !cpu.registers.get_flags().zero {
            let address = ((args[1] as u16) << 8) | (args[0] as u16);
            Instruction::call(cpu, mmu, address);
            self.cycles += 12;
        }
        self.cycles
    }

    /// PUSH BC
    /// 1 16
    /// - - - -
    fn op_00c5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::push(cpu, mmu, CpuRegIndex::BC);
        self.cycles
    }

    /// ADD A,d8
    /// 2 8
    /// Z 0 H C
    fn op_00c6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.add_8(cpu, mmu, args, Src::A, Src::D8)
    }

    /// RST 00H
    /// 1 16
    /// - - - -
    fn op_00c7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::call(cpu, mmu, 0x00);
        self.cycles
    }

    /// RET Z
    /// 1 8/20
    /// - - - -
    fn op_00c8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if cpu.registers.get_flags().zero {
            self.pop(cpu, mmu, CpuRegIndex::PC);
            self.cycles += 12;
        }
        self.cycles
    }

    /// RET
    /// 1 16
    /// - - - -
    fn op_00c9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.pop(cpu, mmu, CpuRegIndex::PC)
    }

    /// JP Z,a16
    /// 3 12/16
    /// - - - -
    fn op_00ca(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if cpu.registers.get_flags().zero {
            let address = ((args[1] as u16) << 8) | (args[0] as u16);
            Instruction::jump(cpu, address);
            self.cycles += 4;
        }
        self.cycles
    }

    /// CALL Z,a16
    /// 3 12/24
    /// - - - -
    fn op_00cc(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if cpu.registers.get_flags().zero {
            let address = ((args[1] as u16) << 8) | (args[0] as u16);
            Instruction::call(cpu, mmu, address);
            self.cycles += 12;
        }
        self.cycles
    }

    /// CALL a16
    /// 3 24
    /// - - - -
    fn op_00cd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let address = ((args[1] as u16) << 8) | (args[0] as u16);
        Instruction::call(cpu, mmu, address);
        self.cycles
    }

    /// ADC A,d8
    /// 2 8
    /// Z 0 H C
    fn op_00ce(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.adc_8(cpu, mmu, args, Src::A, Src::D8)
    }

    /// RST 08H
    /// 1 16
    /// - - - -
    fn op_00cf(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::call(cpu, mmu, 0x08);
        self.cycles
    }

    /// RET NC
    /// 1 8/20
    /// - - - -
    fn op_00d0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !cpu.registers.get_flags().carry {
            self.pop(cpu, mmu, CpuRegIndex::PC);
            self.cycles += 12;
        }
        self.cycles
    }

    /// POP DE
    /// 1 12
    /// - - - -
    fn op_00d1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.pop(cpu, mmu, CpuRegIndex::DE)
    }

    /// JP NC,a16
    /// 3 12/16
    /// - - - -
    fn op_00d2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !cpu.registers.get_flags().carry {
            let address = ((args[1] as u16) << 8) | (args[0] as u16);
            Instruction::jump(cpu, address);
            self.cycles += 4;
        }
        self.cycles
    }

    /// CALL NC,a16
    /// 3 12/24
    /// - - - -
    fn op_00d4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if !cpu.registers.get_flags().carry {
            let address = ((args[1] as u16) << 8) | (args[0] as u16);
            Instruction::call(cpu, mmu, address);
            self.cycles += 12;
        }
        self.cycles
    }

    /// PUSH DE
    /// 1 16
    /// - - - -
    fn op_00d5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::push(cpu, mmu, CpuRegIndex::DE);
        self.cycles
    }

    /// SUB d8
    /// 2 8
    /// Z 1 H C
    fn op_00d6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sub_8(cpu, mmu, args, Src::A, Src::D8)
    }

    /// RST 10H
    /// 1 16
    /// - - - -
    fn op_00d7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::call(cpu, mmu, 0x10);
        self.cycles
    }

    /// RET C
    /// 1 8/20
    /// - - - -
    fn op_00d8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if cpu.registers.get_flags().carry {
            self.pop(cpu, mmu, CpuRegIndex::PC);
            self.cycles += 12;
        }
        self.cycles
    }

    /// RET I
    /// 1 16
    /// - - - -
    fn op_00d9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.interrupts.set_ime(true);
        self.pop(cpu, mmu, CpuRegIndex::PC)
    }

    /// JP C,a16
    /// 3 12/16
    /// - - - -
    fn op_00da(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if cpu.registers.get_flags().carry {
            let address = ((args[1] as u16) << 8) | (args[0] as u16);
            Instruction::jump(cpu, address);
            self.cycles += 4;
        }
        self.cycles
    }

    /// CALL C,a16
    /// 3 12/24
    /// - - - -
    fn op_00dc(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        if cpu.registers.get_flags().carry {
            let address = ((args[1] as u16) << 8) | (args[0] as u16);
            Instruction::call(cpu, mmu, address);
            self.cycles += 12;
        }
        self.cycles
    }

    /// SBC A,d8
    /// 2 8
    /// Z 1 H C
    fn op_00de(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.sbc_8(cpu, mmu, args, Src::A, Src::D8)
    }

    /// RST 18H
    /// 1 16
    /// - - - -
    fn op_00df(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::call(cpu, mmu, 0x18);
        self.cycles
    }

    /// LDH (a8),A
    /// aka LD ($FF00+a8),A
    /// 2 12
    /// - - - -
    fn op_00e0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A8, Src::A)
    }

    /// POP HL
    /// 1 12
    /// - - - -
    fn op_00e1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.pop(cpu, mmu, CpuRegIndex::HL)
    }

    /// LDH (C),A
    /// aka LD ($FF00+C),A
    /// 1 8
    /// - - - -
    fn op_00e2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::Ca, Src::A)
    }

    /// PUSH HL
    /// 1 16
    /// - - - -
    fn op_00e5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::push(cpu, mmu, CpuRegIndex::HL);
        self.cycles
    }

    /// AND d8
    /// 2 8
    /// Z 0 1 0
    fn op_00e6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.and(cpu, mmu, args, Src::D8)
    }

    /// RST 20H
    /// 1 16
    /// - - - -
    fn op_00e7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::call(cpu, mmu, 0x20);
        self.cycles
    }

    /// ADD SP,r8
    /// 2 16
    /// 0 0 H C
    fn op_00e8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        // SP = SP +/- dd ; dd is 8-bit signed number
        let sp = Instruction::get_source_value_16(cpu, mmu, args, Src::SP);
        let d8 = Instruction::get_source_value_8(cpu, mmu, args, Src::D8);
        let signed = alu::signed_8(d8);

        let (result, flags) = if signed < 0 {
            let a = sp as i32;
            let b = -signed as i32;
            let half_carry = (a & 0x0F) < (b & 0x0F);
            let carry = (a - b) < 0;
            let result = (a - b) as u16;
            (result, Flags {
                zero: result == 0,
                subtract: false,
                half_carry,
                carry,
            })
        } else {
            let a = sp as i32;
            let b = signed as i32;
            let half_carry = ((a & 0x0F) + (b & 0x0F)) > 0x0F;
            let carry = (a + b) > 0xFF;
            let result = (a + b) as u16;
            (result, Flags {
                zero: result == 0,
                subtract: false,
                half_carry,
                carry,
            })
        };

        Instruction::set_target_value_16(cpu, mmu, args, Src::SP, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// JP HL
    /// 1 4
    /// - - - -
    fn op_00e9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::jump(cpu, cpu.registers.get_word(CpuRegIndex::HL));
        self.cycles
    }


    /// LD (a16),A 
    /// 3 16 
    /// - - - -
    fn op_00ea(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A16, Src::A)
    }

    /// XOR d8
    /// 2 8
    /// Z 0 0 0
    fn op_00ee(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.xor(cpu, mmu, args, Src::D8)
    }

    /// RST 28H
    /// 1 16
    /// - - - -
    fn op_00ef(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::call(cpu, mmu, 0x28);
        self.cycles
    }

    /// LDH A,(a8)
    /// aka LD A,($FF00+a8)
    /// 2 12
    /// - - - -
    fn op_00f0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::A8)
    }

    /// POP AF
    /// 1 12
    /// Z N H C
    fn op_00f1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.pop(cpu, mmu, CpuRegIndex::AF);
        let f = cpu.registers.get_byte(CpuRegIndex::F) & 0xF0;
        cpu.registers.set_byte(CpuRegIndex::F, f);
        self.cycles
    }

    /// LDH A,(C)
    /// aka LD A,($FF00+C)
    /// 1 8
    /// - - - -
    fn op_00f2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::Ca)
    }

    /// DI
    /// 1 4
    /// - - - -
    fn op_00f3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.interrupts.set_ime(false);
        self.cycles
    }

    /// PUSH AF
    /// 1 16
    /// - - - -
    fn op_00f5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::push(cpu, mmu, CpuRegIndex::AF);
        self.cycles
    }

    /// OR d8
    /// 2 8
    /// Z 0 0 0
    fn op_00f6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.or(cpu, mmu, args, Src::D8)
    }

    /// RST 30H
    /// 1 16
    /// - - - -
    fn op_00f7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::call(cpu, mmu, 0x30);
        self.cycles
    }

    /// LD HL,SP+r8
    /// 2 12
    /// 0 0 H C
    /// HL = SP +/- dd ; dd is 8-bit signed number
    fn op_00f8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        let a = cpu.registers.get_word(CpuRegIndex::SP);
        let b = alu::signed_8(args[0]);
        let (result, mut flags) = if b < 0 {
             alu::subtract_16(a, (-b) as u16)
        } else {
            alu::add_16(a, b as u16)
        };
        flags.zero = false;
        flags.subtract = false;
        cpu.registers.set_word(CpuRegIndex::HL, result);
        cpu.registers.set_flags(flags);
        self.cycles
    }

    /// LD SP,HL
    /// 1 8
    /// - - - -
    fn op_00f9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_16(cpu, mmu, args, Src::SP, Src::HL)
    }

    /// LD A,(a16)
    /// 3 16
    /// - - - -
    fn op_00fa(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.ld_8(cpu, mmu, args, Src::A, Src::A16)
    }

    /// EI
    /// 1 4
    /// - - - -
    fn op_00fb(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        cpu.interrupts.set_ime(true);
        self.cycles
    }

    /// CP d8
    /// 2 8
    /// Z 1 H C
    fn op_00fe(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.cp(cpu, mmu, args, Src::D8)
    }

    /// RST 38H
    /// 1 16
    /// - - - -
    fn op_00ff(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        Instruction::call(cpu, mmu, 0x38);
        self.cycles
    }

    /// =================== CB PREFIXED ===================

    /// RLC B
    /// 2 8
    /// Z 0 0 C
    fn op_cb00(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::B, true, false)
    }

    /// RLC C
    /// 2 8
    /// Z 0 0 C
    fn op_cb01(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::C, true, false)
    }

    /// RLC D
    /// 2 8
    /// Z 0 0 C
    fn op_cb02(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::D, true, false)
    }

    /// RLC E
    /// 2 8
    /// Z 0 0 C
    fn op_cb03(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::E, true, false)
    }

    /// RLC H
    /// 2 8
    /// Z 0 0 C
    fn op_cb04(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::H, true, false)
    }

    /// RLC L
    /// 2 8
    /// Z 0 0 C
    fn op_cb05(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::L, true, false)
    }

    /// RLC (HL)
    /// 2 16
    /// Z 0 0 C
    fn op_cb06(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::HLa, true, false)
    }

    /// RLC A
    /// 2 8
    /// Z 0 0 C
    fn op_cb07(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::A, true, false)
    }

    /// RRC B
    /// 2 8
    /// Z 0 0 C
    fn op_cb08(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::B, false, false)
    }

    /// RRC C
    /// 2 8
    /// Z 0 0 C
    fn op_cb09(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::C, false, false)
    }

    /// RRC D
    /// 2 8
    /// Z 0 0 C
    fn op_cb0a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::D, false, false)
    }

    /// RRC E
    /// 2 8
    /// Z 0 0 C
    fn op_cb0b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::E, false, false)
    }

    /// RRC H
    /// 2 8
    /// Z 0 0 C
    fn op_cb0c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::H, false, false)
    }

    /// RRC L
    /// 2 8
    /// Z 0 0 C
    fn op_cb0d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::L, false, false)
    }

    /// RRC (HL)
    /// 2 16
    /// Z 0 0 C
    fn op_cb0e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::HLa, false, false)
    }

    /// RRC A
    /// 2 8
    /// Z 0 0 C
    fn op_cb0f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate_circular(cpu, mmu, args, Src::A, false, false)
    }

    /// RL B
    /// 2 8
    /// Z 0 0 C
    fn op_cb10(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::B, true, false)
    }

    /// RL C
    /// 2 8
    /// Z 0 0 C
    fn op_cb11(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::C, true, false)
    }

    /// RL D
    /// 2 8
    /// Z 0 0 C
    fn op_cb12(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::D, true, false)
    }

    /// RL E
    /// 2 8
    /// Z 0 0 C
    fn op_cb13(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::E, true, false)
    }

    /// RL H
    /// 2 8
    /// Z 0 0 C
    fn op_cb14(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::H, true, false)
    }

    /// RL L
    /// 2 8
    /// Z 0 0 C
    fn op_cb15(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::L, true, false)
    }

    /// RL (HL)
    /// 2 16
    /// Z 0 0 C
    fn op_cb16(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::HLa, true, false)
    }

    /// RL A
    /// 2 8
    /// Z 0 0 C
    fn op_cb17(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::A, true, false)
    }

    /// RR B
    /// 2 8
    /// Z 0 0 C
    fn op_cb18(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::B, false, false)
    }

    /// RR C
    /// 2 8
    /// Z 0 0 C
    fn op_cb19(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::C, false, false)
    }

    /// RR D
    /// 2 8
    /// Z 0 0 C
    fn op_cb1a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::D, false, false)
    }

    /// RR E
    /// 2 8
    /// Z 0 0 C
    fn op_cb1b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::E, false, false)
    }

    /// RR H
    /// 2 8
    /// Z 0 0 C
    fn op_cb1c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::H, false, false)
    }

    /// RR L
    /// 2 8
    /// Z 0 0 C
    fn op_cb1d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::L, false, false)
    }

    /// RR (HL)
    /// 2 16
    /// Z 0 0 C
    fn op_cb1e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::HLa, false, false)
    }

    /// RR A
    /// 2 8
    /// Z 0 0 C
    fn op_cb1f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.rotate(cpu, mmu, args, Src::A, false, false)
    }

    /// SLA B
    /// 2 8
    /// Z 0 0 C
    fn op_cb20(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::B, true, true)
    }

    /// SLA C
    /// 2 8
    /// Z 0 0 C
    fn op_cb21(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::C, true, true)
    }

    /// SLA D
    /// 2 8
    /// Z 0 0 C
    fn op_cb22(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::D, true, true)
    }

    /// SLA E
    /// 2 8
    /// Z 0 0 C
    fn op_cb23(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::E, true, true)
    }

    /// SLA H
    /// 2 8
    /// Z 0 0 C
    fn op_cb24(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::H, true, true)
    }

    /// SLA L
    /// 2 8
    /// Z 0 0 C
    fn op_cb25(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::L, true, true)
    }

    /// SLA (HL)
    /// 2 16
    /// Z 0 0 C
    fn op_cb26(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::HLa, true, true)
    }

    /// SLA A
    /// 2 8
    /// Z 0 0 C
    fn op_cb27(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::A, true, true)
    }

    /// SRA B
    /// 2 8
    /// Z 0 0 C
    fn op_cb28(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::B, false, true)
    }

    /// SRA C
    /// 2 8
    /// Z 0 0 C
    fn op_cb29(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::C, false, true)
    }

    /// SRA D
    /// 2 8
    /// Z 0 0 C
    fn op_cb2a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::D, false, true)
    }

    /// SRA E
    /// 2 8
    /// Z 0 0 C
    fn op_cb2b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::E, false, true)
    }

    /// SRA H
    /// 2 8
    /// Z 0 0 C
    fn op_cb2c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::H, false, true)
    }

    /// SRA L
    /// 2 8
    /// Z 0 0 C
    fn op_cb2d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::L, false, true)
    }

    /// SRA (HL)
    /// 2 16
    /// Z 0 0 C
    fn op_cb2e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::HLa, false, true)
    }

    /// SRA A
    /// 2 8
    /// Z 0 0 C
    fn op_cb2f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::A, false, true)
    }

    /// SWAP B
    /// 2 8
    /// Z 0 0 0
    fn op_cb30(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.swap(cpu, mmu, args, Src::B)
    }

    /// SWAP C
    /// 2 8
    /// Z 0 0 0
    fn op_cb31(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.swap(cpu, mmu, args, Src::C)
    }

    /// SWAP D
    /// 2 8
    /// Z 0 0 0
    fn op_cb32(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.swap(cpu, mmu, args, Src::D)
    }

    /// SWAP E
    /// 2 8
    /// Z 0 0 0
    fn op_cb33(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.swap(cpu, mmu, args, Src::E)
    }

    /// SWAP H
    /// 2 8
    /// Z 0 0 0
    fn op_cb34(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.swap(cpu, mmu, args, Src::H)
    }

    /// SWAP L
    /// 2 8
    /// Z 0 0 0
    fn op_cb35(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.swap(cpu, mmu, args, Src::L)
    }

    /// SWAP (HL)
    /// 2 16
    /// Z 0 0 0
    fn op_cb36(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.swap(cpu, mmu, args, Src::HLa)
    }

    /// SWAP A
    /// 2 8
    /// Z 0 0 0
    fn op_cb37(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.swap(cpu, mmu, args, Src::A)
    }

    /// SRL B
    /// 2 8
    /// Z 0 0 C
    fn op_cb38(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::B, false, false)
    }

    /// SRL C
    /// 2 8
    /// Z 0 0 C
    fn op_cb39(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::C, false, false)
    }

    /// SRL D
    /// 2 8
    /// Z 0 0 C
    fn op_cb3a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::D, false, false)
    }

    /// SRL E
    /// 2 8
    /// Z 0 0 C
    fn op_cb3b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::E, false, false)
    }

    /// SRL H
    /// 2 8
    /// Z 0 0 C
    fn op_cb3c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::H, false, false)
    }

    /// SRL L
    /// 2 8
    /// Z 0 0 C
    fn op_cb3d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::L, false, false)
    }

    /// SRL (HL)
    /// 2 16
    /// Z 0 0 C
    fn op_cb3e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::HLa, false, false)
    }

    /// SRL A
    /// 2 8
    /// Z 0 0 C
    fn op_cb3f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.shift(cpu, mmu, args, Src::A, false, false)
    }

    /// BIT 0,B
    /// 2 8
    /// Z 0 1 -
    fn op_cb40(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::B, 0)
    }

    /// BIT 0,C
    /// 2 8
    /// Z 0 1 -
    fn op_cb41(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::C, 0)
    }

    /// BIT 0,D
    /// 2 8
    /// Z 0 1 -
    fn op_cb42(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::D, 0)
    }

    /// BIT 0,E
    /// 2 8
    /// Z 0 1 -
    fn op_cb43(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::E, 0)
    }

    /// BIT 0,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb44(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::H, 0)
    }

    /// BIT 0,L
    /// 2 8
    /// Z 0 1 -
    fn op_cb45(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::L, 0)
    }

    /// BIT 0,(HL)
    /// 2 16
    /// Z 0 1 -
    fn op_cb46(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::HLa, 0)
    }

    /// BIT 0,A
    /// 2 8
    /// Z 0 1 -
    fn op_cb47(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::A, 0)
    }

    /// BIT 1,B
    /// 2 8
    /// Z 0 1 -
    fn op_cb48(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::B, 1)
    }

    /// BIT 1,C
    /// 2 8
    /// Z 0 1 -
    fn op_cb49(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::C, 1)
    }

    /// BIT 1,D
    /// 2 8
    /// Z 0 1 -
    fn op_cb4a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::D, 1)
    }

    /// BIT 1,E
    /// 2 8
    /// Z 0 1 -
    fn op_cb4b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::E, 1)
    }

    /// BIT 1,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb4c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::H, 1)
    }

    /// BIT 1,L
    /// 2 8
    /// Z 0 1 -
    fn op_cb4d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::L, 1)
    }

    /// BIT 1,(HL)
    /// 2 16
    /// Z 0 1 -
    fn op_cb4e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::HLa, 1)
    }

    /// BIT 1,A
    /// 2 8
    /// Z 0 1 -
    fn op_cb4f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::A, 1)
    }

    /// BIT 2,B
    /// 2 8
    /// Z 0 1 -
    fn op_cb50(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::B, 2)
    }

    /// BIT 2,C
    /// 2 8
    /// Z 0 1 -
    fn op_cb51(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::C, 2)
    }

    /// BIT 2,D
    /// 2 8
    /// Z 0 1 -
    fn op_cb52(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::D, 2)
    }

    /// BIT 2,E
    /// 2 8
    /// Z 0 1 -
    fn op_cb53(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::E, 2)
    }

    /// BIT 2,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb54(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::H, 2)
    }

    /// BIT 2,L
    /// 2 8
    /// Z 0 1 -
    fn op_cb55(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::L, 2)
    }

    /// BIT 2,(HL)
    /// 2 16
    /// Z 0 1 -
    fn op_cb56(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::HLa, 2)
    }

    /// BIT 2,A
    /// 2 8
    /// Z 0 1 -
    fn op_cb57(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::A, 2)
    }

    /// BIT 3,B
    /// 2 8
    /// Z 0 1 -
    fn op_cb58(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::B, 3)
    }

    /// BIT 3,C
    /// 2 8
    /// Z 0 1 -
    fn op_cb59(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::C, 3)
    }

    /// BIT 3,D
    /// 2 8
    /// Z 0 1 -
    fn op_cb5a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::D, 3)
    }

    /// BIT 3,E
    /// 2 8
    /// Z 0 1 -
    fn op_cb5b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::E, 3)
    }

    /// BIT 3,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb5c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::H, 3)
    }

    /// BIT 3,L
    /// 2 8
    /// Z 0 1 -
    fn op_cb5d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::L, 3)
    }

    /// BIT 3,(HL)
    /// 2 16
    /// Z 0 1 -
    fn op_cb5e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::HLa, 3)
    }

    /// BIT 3,A
    /// 2 8
    /// Z 0 1 -
    fn op_cb5f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::A, 3)
    }

    /// BIT 4,B
    /// 2 8
    /// Z 0 1 -
    fn op_cb60(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::B, 4)
    }

    /// BIT 4,C
    /// 2 8
    /// Z 0 1 -
    fn op_cb61(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::C, 4)
    }

    /// BIT 4,D
    /// 2 8
    /// Z 0 1 -
    fn op_cb62(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::D, 4)
    }

    /// BIT 4,E
    /// 2 8
    /// Z 0 1 -
    fn op_cb63(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::E, 4)
    }

    /// BIT 4,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb64(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::H, 4)
    }

    /// BIT 4,L
    /// 2 8
    /// Z 0 1 -
    fn op_cb65(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::L, 4)
    }

    /// BIT 4,(HL)
    /// 2 16
    /// Z 0 1 -
    fn op_cb66(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::HLa, 4)
    }

    /// BIT 4,A
    /// 2 8
    /// Z 0 1 -
    fn op_cb67(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::A, 4)
    }

    /// BIT 5,B
    /// 2 8
    /// Z 0 1 -
    fn op_cb68(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::B, 5)
    }

    /// BIT 5,C
    /// 2 8
    /// Z 0 1 -
    fn op_cb69(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::C, 5)
    }

    /// BIT 5,D
    /// 2 8
    /// Z 0 1 -
    fn op_cb6a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::D, 5)
    }

    /// BIT 5,E
    /// 2 8
    /// Z 0 1 -
    fn op_cb6b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::E, 5)
    }

    /// BIT 5,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb6c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::H, 5)
    }

    /// BIT 5,L
    /// 2 8
    /// Z 0 1 -
    fn op_cb6d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::L, 5)
    }

    /// BIT 5,(HL)
    /// 2 16
    /// Z 0 1 -
    fn op_cb6e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::HLa, 5)
    }

    /// BIT 5,A
    /// 2 8
    /// Z 0 1 -
    fn op_cb6f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::A, 5)
    }

    /// BIT 6,B
    /// 2 8
    /// Z 0 1 -
    fn op_cb70(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::B, 6)
    }

    /// BIT 6,C
    /// 2 8
    /// Z 0 1 -
    fn op_cb71(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::C, 6)
    }

    /// BIT 6,D
    /// 2 8
    /// Z 0 1 -
    fn op_cb72(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::D, 6)
    }

    /// BIT 6,E
    /// 2 8
    /// Z 0 1 -
    fn op_cb73(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::E, 6)
    }

    /// BIT 6,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb74(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::H, 6)
    }

    /// BIT 6,L
    /// 2 8
    /// Z 0 1 -
    fn op_cb75(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::L, 6)
    }

    /// BIT 6,(HL)
    /// 2 16
    /// Z 0 1 -
    fn op_cb76(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::HLa, 6)
    }

    /// BIT 6,A
    /// 2 8
    /// Z 0 1 -
    fn op_cb77(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::A, 6)
    }

    /// BIT 7,B
    /// 2 8
    /// Z 0 1 -
    fn op_cb78(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::B, 7)
    }

    /// BIT 7,C
    /// 2 8
    /// Z 0 1 -
    fn op_cb79(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::C, 7)
    }

    /// BIT 7,D
    /// 2 8
    /// Z 0 1 -
    fn op_cb7a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::D, 7)
    }

    /// BIT 7,E
    /// 2 8
    /// Z 0 1 -
    fn op_cb7b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::E, 7)
    }

    /// BIT 7,H
    /// 2 8
    /// Z 0 1 -
    fn op_cb7c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::H, 7)
    }

    /// BIT 7,L
    /// 2 8
    /// Z 0 1 -
    fn op_cb7d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::L, 7)
    }

    /// BIT 7,(HL)
    /// 2 16
    /// Z 0 1 -
    fn op_cb7e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::HLa, 7)
    }

    /// BIT 7,A
    /// 2 8
    /// Z 0 1 -
    fn op_cb7f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.bit(cpu, mmu, args, Src::A, 7)
    }

    /// RES 0,B
    /// 2 8
    /// - - - -
    fn op_cb80(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 0, false)
    }

    /// RES 0,C
    /// 2 8
    /// - - - -
    fn op_cb81(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 0, false)
    }

    /// RES 0,D
    /// 2 8
    /// - - - -
    fn op_cb82(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 0, false)
    }

    /// RES 0,E
    /// 2 8
    /// - - - -
    fn op_cb83(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 0, false)
    }

    /// RES 0,H
    /// 2 8
    /// - - - -
    fn op_cb84(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 0, false)
    }

    /// RES 0,L
    /// 2 8
    /// - - - -
    fn op_cb85(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 0, false)
    }

    /// RES 0,(HL)
    /// 2 16
    /// - - - -
    fn op_cb86(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 0, false)
    }

    /// RES 0,A
    /// 2 8
    /// - - - -
    fn op_cb87(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 0, false)
    }

    /// RES 1,B
    /// 2 8
    /// - - - -
    fn op_cb88(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 1, false)
    }

    /// RES 1,C
    /// 2 8
    /// - - - -
    fn op_cb89(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 1, false)
    }

    /// RES 1,D
    /// 2 8
    /// - - - -
    fn op_cb8a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 1, false)
    }

    /// RES 1,E
    /// 2 8
    /// - - - -
    fn op_cb8b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 1, false)
    }

    /// RES 1,H
    /// 2 8
    /// - - - -
    fn op_cb8c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 1, false)
    }

    /// RES 1,L
    /// 2 8
    /// - - - -
    fn op_cb8d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 1, false)
    }

    /// RES 1,(HL)
    /// 2 16
    /// - - - -
    fn op_cb8e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 1, false)
    }

    /// RES 1,A
    /// 2 8
    /// - - - -
    fn op_cb8f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 1, false)
    }

    /// RES 2,B
    /// 2 8
    /// - - - -
    fn op_cb90(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 2, false)
    }

    /// RES 2,C
    /// 2 8
    /// - - - -
    fn op_cb91(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 2, false)
    }

    /// RES 2,D
    /// 2 8
    /// - - - -
    fn op_cb92(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 2, false)
    }

    /// RES 2,E
    /// 2 8
    /// - - - -
    fn op_cb93(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 2, false)
    }

    /// RES 2,H
    /// 2 8
    /// - - - -
    fn op_cb94(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 2, false)
    }

    /// RES 2,L
    /// 2 8
    /// - - - -
    fn op_cb95(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 2, false)
    }

    /// RES 2,(HL)
    /// 2 16
    /// - - - -
    fn op_cb96(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 2, false)
    }

    /// RES 2,A
    /// 2 8
    /// - - - -
    fn op_cb97(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 2, false)
    }

    /// RES 3,B
    /// 2 8
    /// - - - -
    fn op_cb98(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 3, false)
    }

    /// RES 3,C
    /// 2 8
    /// - - - -
    fn op_cb99(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 3, false)
    }

    /// RES 3,D
    /// 2 8
    /// - - - -
    fn op_cb9a(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 3, false)
    }

    /// RES 3,E
    /// 2 8
    /// - - - -
    fn op_cb9b(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 3, false)
    }

    /// RES 3,H
    /// 2 8
    /// - - - -
    fn op_cb9c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 3, false)
    }

    /// RES 3,L
    /// 2 8
    /// - - - -
    fn op_cb9d(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 3, false)
    }

    /// RES 3,(HL)
    /// 2 16
    /// - - - -
    fn op_cb9e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 3, false)
    }

    /// RES 3,A
    /// 2 8
    /// - - - -
    fn op_cb9f(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 3, false)
    }

    /// RES 4,B
    /// 2 8
    /// - - - -
    fn op_cba0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 4, false)
    }

    /// RES 4,C
    /// 2 8
    /// - - - -
    fn op_cba1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 4, false)
    }

    /// RES 4,D
    /// 2 8
    /// - - - -
    fn op_cba2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 4, false)
    }

    /// RES 4,E
    /// 2 8
    /// - - - -
    fn op_cba3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 4, false)
    }

    /// RES 4,H
    /// 2 8
    /// - - - -
    fn op_cba4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 4, false)
    }

    /// RES 4,L
    /// 2 8
    /// - - - -
    fn op_cba5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 4, false)
    }

    /// RES 4,(HL)
    /// 2 16
    /// - - - -
    fn op_cba6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 4, false)
    }

    /// RES 4,A
    /// 2 8
    /// - - - -
    fn op_cba7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 4, false)
    }

    /// RES 5,B
    /// 2 8
    /// - - - -
    fn op_cba8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 5, false)
    }

    /// RES 5,C
    /// 2 8
    /// - - - -
    fn op_cba9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 5, false)
    }

    /// RES 5,D
    /// 2 8
    /// - - - -
    fn op_cbaa(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 5, false)
    }

    /// RES 5,E
    /// 2 8
    /// - - - -
    fn op_cbab(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 5, false)
    }

    /// RES 5,H
    /// 2 8
    /// - - - -
    fn op_cbac(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 5, false)
    }

    /// RES 5,L
    /// 2 8
    /// - - - -
    fn op_cbad(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 5, false)
    }

    /// RES 5,(HL)
    /// 2 16
    /// - - - -
    fn op_cbae(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 5, false)
    }

    /// RES 5,A
    /// 2 8
    /// - - - -
    fn op_cbaf(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 5, false)
    }

    /// RES 6,B
    /// 2 8
    /// - - - -
    fn op_cbb0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 6, false)
    }

    /// RES 6,C
    /// 2 8
    /// - - - -
    fn op_cbb1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 6, false)
    }

    /// RES 6,D
    /// 2 8
    /// - - - -
    fn op_cbb2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 6, false)
    }

    /// RES 6,E
    /// 2 8
    /// - - - -
    fn op_cbb3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 6, false)
    }

    /// RES 6,H
    /// 2 8
    /// - - - -
    fn op_cbb4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 6, false)
    }

    /// RES 6,L
    /// 2 8
    /// - - - -
    fn op_cbb5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 6, false)
    }

    /// RES 6,(HL)
    /// 2 16
    /// - - - -
    fn op_cbb6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 6, false)
    }

    /// RES 6,A
    /// 2 8
    /// - - - -
    fn op_cbb7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 6, false)
    }

    /// RES 7,B
    /// 2 8
    /// - - - -
    fn op_cbb8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 7, false)
    }

    /// RES 7,C
    /// 2 8
    /// - - - -
    fn op_cbb9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 7, false)
    }

    /// RES 7,D
    /// 2 8
    /// - - - -
    fn op_cbba(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 7, false)
    }

    /// RES 7,E
    /// 2 8
    /// - - - -
    fn op_cbbb(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 7, false)
    }

    /// RES 7,H
    /// 2 8
    /// - - - -
    fn op_cbbc(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 7, false)
    }

    /// RES 7,L
    /// 2 8
    /// - - - -
    fn op_cbbd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 7, false)
    }

    /// RES 7,(HL)
    /// 2 16
    /// - - - -
    fn op_cbbe(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 7, false)
    }

    /// RES 7,A
    /// 2 8
    /// - - - -
    fn op_cbbf(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 7, false)
    }

    /// SET 0,B
    /// 2 8
    /// - - - -
    fn op_cbc0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 0, true)
    }

    /// SET 0,C
    /// 2 8
    /// - - - -
    fn op_cbc1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 0, true)
    }

    /// SET 0,D
    /// 2 8
    /// - - - -
    fn op_cbc2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 0, true)
    }

    /// SET 0,E
    /// 2 8
    /// - - - -
    fn op_cbc3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 0, true)
    }

    /// SET 0,H
    /// 2 8
    /// - - - -
    fn op_cbc4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 0, true)
    }

    /// SET 0,L
    /// 2 8
    /// - - - -
    fn op_cbc5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 0, true)
    }

    /// SET 0,(HL)
    /// 2 16
    /// - - - -
    fn op_cbc6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 0, true)
    }

    /// SET 0,A
    /// 2 8
    /// - - - -
    fn op_cbc7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 0, true)
    }

    /// SET 1,B
    /// 2 8
    /// - - - -
    fn op_cbc8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 1, true)
    }

    /// SET 1,C
    /// 2 8
    /// - - - -
    fn op_cbc9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 1, true)
    }

    /// SET 1,D
    /// 2 8
    /// - - - -
    fn op_cbca(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 1, true)
    }

    /// SET 1,E
    /// 2 8
    /// - - - -
    fn op_cbcb(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 1, true)
    }

    /// SET 1,H
    /// 2 8
    /// - - - -
    fn op_cbcc(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 1, true)
    }

    /// SET 1,L
    /// 2 8
    /// - - - -
    fn op_cbcd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 1, true)
    }

    /// SET 1,(HL)
    /// 2 16
    /// - - - -
    fn op_cbce(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 1, true)
    }

    /// SET 1,A
    /// 2 8
    /// - - - -
    fn op_cbcf(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 1, true)
    }

    /// SET 2,B
    /// 2 8
    /// - - - -
    fn op_cbd0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 2, true)
    }

    /// SET 2,C
    /// 2 8
    /// - - - -
    fn op_cbd1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 2, true)
    }

    /// SET 2,D
    /// 2 8
    /// - - - -
    fn op_cbd2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 2, true)
    }

    /// SET 2,E
    /// 2 8
    /// - - - -
    fn op_cbd3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 2, true)
    }

    /// SET 2,H
    /// 2 8
    /// - - - -
    fn op_cbd4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 2, true)
    }

    /// SET 2,L
    /// 2 8
    /// - - - -
    fn op_cbd5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 2, true)
    }

    /// SET 2,(HL)
    /// 2 16
    /// - - - -
    fn op_cbd6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 2, true)
    }

    /// SET 2,A
    /// 2 8
    /// - - - -
    fn op_cbd7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 2, true)
    }

    /// SET 3,B
    /// 2 8
    /// - - - -
    fn op_cbd8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 3, true)
    }

    /// SET 3,C
    /// 2 8
    /// - - - -
    fn op_cbd9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 3, true)
    }

    /// SET 3,D
    /// 2 8
    /// - - - -
    fn op_cbda(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 3, true)
    }

    /// SET 3,E
    /// 2 8
    /// - - - -
    fn op_cbdb(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 3, true)
    }

    /// SET 3,H
    /// 2 8
    /// - - - -
    fn op_cbdc(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 3, true)
    }

    /// SET 3,L
    /// 2 8
    /// - - - -
    fn op_cbdd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 3, true)
    }

    /// SET 3,(HL)
    /// 2 16
    /// - - - -
    fn op_cbde(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 3, true)
    }

    /// SET 3,A
    /// 2 8
    /// - - - -
    fn op_cbdf(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 3, true)
    }
    
    /// SET 4,B
    /// 2 8
    /// - - - -
    fn op_cbe0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 4, true)
    }

    /// SET 4,C
    /// 2 8
    /// - - - -
    fn op_cbe1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 4, true)
    }

    /// SET 4,D
    /// 2 8
    /// - - - -
    fn op_cbe2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 4, true)
    }

    /// SET 4,E
    /// 2 8
    /// - - - -
    fn op_cbe3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 4, true)
    }

    /// SET 4,H
    /// 2 8
    /// - - - -
    fn op_cbe4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 4, true)
    }

    /// SET 4,L
    /// 2 8
    /// - - - -
    fn op_cbe5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 4, true)
    }

    /// SET 4,(HL)
    /// 2 16
    /// - - - -
    fn op_cbe6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 4, true)
    }

    /// SET 4,A
    /// 2 8
    /// - - - -
    fn op_cbe7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 4, true)
    }

    /// SET 5,B
    /// 2 8
    /// - - - -
    fn op_cbe8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 5, true)
    }

    /// SET 5,C
    /// 2 8
    /// - - - -
    fn op_cbe9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 5, true)
    }

    /// SET 5,D
    /// 2 8
    /// - - - -
    fn op_cbea(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 5, true)
    }

    /// SET 5,E
    /// 2 8
    /// - - - -
    fn op_cbeb(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 5, true)
    }

    /// SET 5,H
    /// 2 8
    /// - - - -
    fn op_cbec(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 5, true)
    }

    /// SET 5,L
    /// 2 8
    /// - - - -
    fn op_cbed(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 5, true)
    }

    /// SET 5,(HL)
    /// 2 16
    /// - - - -
    fn op_cbee(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 5, true)
    }

    /// SET 5,A
    /// 2 8
    /// - - - -
    fn op_cbef(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 5, true)
    }

    /// SET 6,B
    /// 2 8
    /// - - - -
    fn op_cbf0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 6, true)
    }

    /// SET 6,C
    /// 2 8
    /// - - - -
    fn op_cbf1(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 6, true)
    }

    /// SET 6,D
    /// 2 8
    /// - - - -
    fn op_cbf2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 6, true)
    }

    /// SET 6,E
    /// 2 8
    /// - - - -
    fn op_cbf3(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 6, true)
    }

    /// SET 6,H
    /// 2 8
    /// - - - -
    fn op_cbf4(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 6, true)
    }

    /// SET 6,L
    /// 2 8
    /// - - - -
    fn op_cbf5(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 6, true)
    }

    /// SET 6,(HL)
    /// 2 16
    /// - - - -
    fn op_cbf6(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 6, true)
    }

    /// SET 6,A
    /// 2 8
    /// - - - -
    fn op_cbf7(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 6, true)
    }

    /// SET 7,B
    /// 2 8
    /// - - - -
    fn op_cbf8(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::B, 7, true)
    }

    /// SET 7,C
    /// 2 8
    /// - - - -
    fn op_cbf9(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::C, 7, true)
    }

    /// SET 7,D
    /// 2 8
    /// - - - -
    fn op_cbfa(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::D, 7, true)
    }

    /// SET 7,E
    /// 2 8
    /// - - - -
    fn op_cbfb(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::E, 7, true)
    }

    /// SET 7,H
    /// 2 8
    /// - - - -
    fn op_cbfc(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::H, 7, true)
    }

    /// SET 7,L
    /// 2 8
    /// - - - -
    fn op_cbfd(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::L, 7, true)
    }

    /// SET 7,(HL)
    /// 2 16
    /// - - - -
    fn op_cbfe(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::HLa, 7, true)
    }

    /// SET 7,A
    /// 2 8
    /// - - - -
    fn op_cbff(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, args: &[u8]) -> i16 {
        self.set(cpu, mmu, args, Src::A, 7, true)
    }
}
