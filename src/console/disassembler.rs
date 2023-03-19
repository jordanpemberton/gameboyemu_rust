use std::fs::File;
use std::io::Write;
use crate::cartridge::cartridge::Cartridge;
use crate::console::instructions::Instruction;

const PREFIX_BYTE: u8 = 0xCB;

pub(crate) fn disassemble_to_output_file(rom: &Vec<u8>, output_path: &str) {
    let s = format!("{}", disassemble(rom));
    let mut file = File::create(output_path)
        .expect("Failed to create file.");
    file.write_all(s.as_ref()).expect("\n\nSomething went wrong...");
}

pub(crate) fn disassemble(rom: &Vec<u8>) -> String {
    let mut output = String::new();
    let mut pc: usize = 0;
    let n = rom.len();

    while pc < n {
        let (s, new_pc) = disassemble_line(pc, rom);
        output += s.as_str();
        pc = new_pc;
    }

    output
}

fn disassemble_opcode(mut pc: usize, rom: &Vec<u8>) -> (u16, usize) {
    let mut opcode: u16;

    let mut byte = rom[pc];
    pc += 1;

    if byte == PREFIX_BYTE {
        opcode = (byte as u16) << 8;
        byte = rom[pc];
        pc += 1;
        opcode |= byte as u16;
    } else {
        opcode = byte as u16;
    }

    (opcode, pc)
}

fn disassemble_line(pc: usize, rom: &Vec<u8>) -> (String, usize) {
    let (opcode, mut new_pc) = disassemble_opcode(pc, rom);

    let instruction = Instruction::get_instruction(opcode);

    let mut arg1 = String::new();
    if instruction.size > 1
            && ((opcode & 0xFF00) >> 8) as u8 != PREFIX_BYTE
            && new_pc < rom.len() {
        arg1 = format!("{:#04X}", rom[new_pc]);
        new_pc += 1;
    };

    let mut arg2 = String::new();
    if instruction.size > 2
            && ((opcode & 0xFF00) >> 8) as u8 != PREFIX_BYTE
            && new_pc < rom.len() {
        arg2 = format!("{:#04X}", rom[new_pc]);
        new_pc += 1;
    };

    (
        format!("{:#06X}\t{:#06X}: '{}'\t{}\t{}\n",
            pc,
            opcode,
            instruction.mnemonic,
            arg1,
            arg2
        ),
        new_pc
    )
}
