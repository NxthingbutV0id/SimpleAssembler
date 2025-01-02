use std::collections::HashMap;
use crate::assembler::error::EvaluatorError;
use crate::symbols::instruction::Instruction;
use crate::symbols::opcodes::Opcode;
use crate::symbols::operands::Operand;

type Definitions = HashMap<String, i16>;

pub fn evaluate_program(program: &mut [Instruction]) -> Result<(), EvaluatorError>{
    info!("Evaluating program...");
    let defined: Definitions = find_definitions(&program)?;

    for i in 0..program.len() {
        evaluate_instruction(&mut program[i], &defined)?;
    }

    Ok(())
}

fn find_definitions(program: &[Instruction]) -> Result<Definitions, EvaluatorError> {
    let mut defined: Definitions = HashMap::new();
    for instruction in program.iter() {
        if let Opcode::_Definition = instruction.opcode {
            if let Operand::Definition(def) = &instruction.operands[0] {
                let imm = def.value;

                if imm.is_none() {
                    // This should never happen, as the parser should catch this
                    return Err(EvaluatorError::MissingDefinitionValue(def.name.clone()));
                }

                defined.insert(def.name.clone(), imm.unwrap());
                trace!("Found definition: {} = {}", def.name, imm.unwrap());
            }
        }
    }
    trace!("{} definitions found: {:?}", defined.len(), defined.keys());
    Ok(defined)
}

fn evaluate_instruction(instruction: &mut Instruction, defined: &Definitions) -> Result<(), EvaluatorError> {
    for i in 0..instruction.operands.len() {
        let operand = &mut instruction.operands[i];
        if let Operand::Label(offset) = operand {
            let start = offset.clone().binding;
            if start.is_none() {
                return Err(EvaluatorError::UnknownOffset(offset.name.clone()));
            }
            let start = start.unwrap();

            if start.address.is_none() {
                return Err(EvaluatorError::InvalidOffset(offset.name.clone()));
            }
            let start = start.address.unwrap();

            trace!("Setting offset .{} to {}", offset.name, start);
            offset.offset = Some(start);
        } else if Opcode::_Definition != instruction.opcode {
            if let Operand::Definition(def) = operand {
                match defined.get(&def.name) {
                    Some(i) => {
                        trace!("Replacing {} with {}", def.name, i);
                        def.value = Some(*i);
                    },
                    None => return Err(EvaluatorError::UnknownDefinition(def.name.clone()))
                }
            }
        }
    }

    Ok(())
}