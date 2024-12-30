use std::collections::HashMap;
use custom_error::custom_error;
use crate::symbols::instruction::Instruction;
use crate::symbols::opcodes::Opcode;
use crate::symbols::operands::Operand;


custom_error! {pub EvaluatorError
    UnknownDefinition{name:String} = "Unknown definition: {name}",
    InvalidDefinition{name:String} = "Definition {name} is missing a value",
    UnknownOffset{name:String} = "Offset {name} not found",
    InvalidOffset{name:String} = "Offset {name} does not have a binding"
}

pub fn evaluate_program(program: &mut Vec<Instruction>) -> Result<(), EvaluatorError>{
    info!("Evaluating program...");
    let defined: HashMap<String, i16> = find_definitions(&program)?;

    for i in 0..program.len() {
        evaluate_instruction(&mut program[i], &defined)?;
    }

    Ok(())
}

fn find_definitions(program: &Vec<Instruction>) -> Result<HashMap<String, i16>, EvaluatorError> {
    let mut defined: HashMap<String, i16> = HashMap::new();
    for instruction in program.iter() {
        //let instruction = &program[i];
        if let Opcode::_Definition = instruction.opcode {
            if let Operand::Definition(def) = &instruction.operands[0] {
                let imm = def.value;

                if imm.is_none() {
                    return Err(EvaluatorError::InvalidDefinition{ name: def.name.clone() });
                }

                defined.insert(def.name.clone(), imm.unwrap());
                trace!("Found definition: {} = {}", def.name, imm.unwrap());
            }
        }
    }
    trace!("Definitions found: {:?}", defined.keys());
    Ok(defined)
}

fn evaluate_instruction(instruction: &mut Instruction, defined: &HashMap<String, i16>) -> Result<(), EvaluatorError> {
    for i in 0..instruction.operands.len() {
        let operand = &mut instruction.operands[i];
        if let Operand::Offset(offset) = operand {
            let start = offset.clone().binding;
            if start.is_none() {
                return Err(EvaluatorError::UnknownOffset{ name: offset.name.clone() });
            }
            let start = start.unwrap();

            if start.address.is_none() {
                return Err(EvaluatorError::InvalidOffset{ name: offset.name.clone() });
            }
            let start = start.address.unwrap();

            trace!("Setting offset .{} to {}", offset.name, start);
            offset.offset = Some(start);
        } else if Opcode::_Definition != instruction.opcode {
            if let Operand::Definition(def) = operand {
                let imm = defined.get(&def.name);
                if imm.is_none() {
                    return Err(EvaluatorError::UnknownDefinition{ name: def.name.clone() });
                }
                let imm = imm.unwrap().clone();
                trace!("Replacing {} with {}", def.name, imm);
                *operand = Operand::Immediate(imm);
            }
        }
    }

    Ok(())
}