use anyhow::anyhow;
use crate::parser::{parse_immediate, parse_instruction, parse_register};
use crate::program::{Instruction, Program};

pub fn preprocess(intput: &str) -> String {

}

pub fn encode_instruction(instr: &Instruction) -> u16 {
    match instr.operation.as_str() {
        "ldi" => {
            let (_, rd) = parse_register(&instr.arguments[0]).unwrap();
            let (_, imm) = parse_immediate(&instr.arguments[1]).unwrap();
        }
        // Add all instructions
        _ => panic!("Unknown instruction: {}", instr.operation)
    }
}

pub fn parse_assembly(input: &str) -> anyhow::Result<Program> {
    let mut instructions: Vec<Instruction> = vec![];

    for line in input.lines() {
        if line.trim().is_empty() ||
            line.starts_with('#') ||
            line.starts_with(';') ||
            line.starts_with("//") {
            continue;
        }

        match parse_instruction(line.trim()) {
            Ok((_, (operation, operands))) => {
                instructions.push(Instruction {
                    operation: operation.to_string(),
                    arguments: operands.into_iter().map(|s| s.to_string()).collect(),
                })
            }
            Err(_) => return Err(anyhow!("Failed to parse line: {}", line)),
        }
    }

    Ok(Program { instructions })
}