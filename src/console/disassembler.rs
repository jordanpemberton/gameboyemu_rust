use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::console::instruction::Instruction;

const PREFIX_BYTE: u8 = 0xCB;

pub(crate) fn disassemble_to_output_file(rom: &Vec<u8>, output_path: &str, filename: &str) {
    let disassembled = disassemble(rom);

    // Create output dir if it doesn't exist
    if !Path::new(&output_path).exists() {
        fs::create_dir_all(&output_path)
            .expect(format!("Failed to create directory {}", output_path).as_str());
    }

    let filepath = Path::new(output_path).join(filename);

    let mut file = File::create(filepath)
        .expect(format!("Failed to create output file {}/{}.", output_path, filename).as_str());

    file.write_all(disassembled.as_ref())
        .expect("\n\nSomething went wrong...");
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

    let num_args = if instruction.is_cbprefixed() {
        if instruction.size > 1 { instruction.size - 2 } else { 0 }
    } else {
        if instruction.size > 0 { instruction.size - 1 } else { 0 }
    };

    let mut s = format!("{:#06X}\t{:#06X}\t{:<14}",
        pc,
        opcode,
        instruction.mnemonic);

    if num_args > 0 {
        s.push_str(format!("\t{:#04X}", rom[new_pc]).as_str());
        new_pc += 1;
    }
    if num_args > 1 {
        s.push_str(format!("\t{:#04X}", rom[new_pc]).as_str());
        new_pc += 1;
    }
    s.push('\n');

    (s, new_pc)
}
