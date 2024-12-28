use std::collections::HashMap;
use crate::symbols::instruction::Instruction;
use crate::symbols::opcodes::Opcode;
use crate::symbols::operands::Operand;

pub fn evaluate_program(program: &mut Vec<Instruction>) {
    info!("Evaluating program...");
    let defined: HashMap<String, i16> = find_definitions(&program);

    for i in 0..program.len() {
        evaluate_instruction(&mut program[i], &defined);
    }
}

fn find_definitions(program: &Vec<Instruction>) -> HashMap<String, i16> {
    let mut defined: HashMap<String, i16> = HashMap::new();
    for i in 0..program.len() {
        let instruction = &program[i];
        if let Opcode::_Definition = instruction.opcode {
            if let Operand::Definition(def) = &instruction.operands[0] {
                let imm = def.value.expect("Definition value missing");
                defined.insert(def.name.clone(), imm);
                trace!("Found definition: {} = {}", def.name, imm);
            }
        }
    }
    trace!("Definitions found: {:?}", defined.keys());
    defined
}

fn evaluate_instruction(instruction: &mut Instruction, defined: &HashMap<String, i16>) {
    for i in 0..instruction.operands.len() {
        let operand = &mut instruction.operands[i];
        if let Operand::Offset(offset) = operand {
            let start = offset.clone()
                .binding
                .expect("Offset binding not found")
                .address
                .expect("Offset binding found but address was not set");
            trace!("Setting offset .{} to {}", offset.name, start);
            offset.offset = Some(start);
        } else if Opcode::_Definition != instruction.opcode {
            if let Operand::Definition(def) = operand {
                let imm = defined.get(&def.name)
                    .expect(&format!("Definition {} not found", def.name))
                    .clone();
                trace!("Replacing {} with {}", def.name, imm);
                *operand = Operand::Immediate(imm);
            }
        }
    }
}