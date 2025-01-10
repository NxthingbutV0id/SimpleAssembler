use std::collections::HashMap;
use thiserror::Error;
use crate::architecture::batpu2::instruction::Instruction;
use crate::architecture::batpu2::opcode::Opcode;
use crate::architecture::batpu2::operand::Operand;

type Definitions = HashMap<String, i16>;

#[derive(Debug, Error)]
pub enum EvaluatorError {
    #[error("Unknown definition: {0}")]
    UnknownDefinition(String),
    #[error("Unknown offset: {0}")]
    UnknownOffset(String),
    #[error("Invalid offset: {0}")]
    InvalidOffset(String),
    #[error("Missing definition value: {0}")]
    MissingDefinitionValue(String)
}

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
            if let Operand::Def(def) = &instruction.operands[0] {
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
        if Opcode::_Definition != instruction.opcode {
            if let Operand::Def(def) = operand {
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