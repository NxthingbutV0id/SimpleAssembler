use std::collections::HashMap;
use thiserror::Error;
use crate::architecture::batpu2::instruction::Instruction;
use crate::architecture::batpu2::opcode::Opcode::_Label;
use crate::architecture::batpu2::operand::Operand;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("Invalid label: {0}")]
    InvalidLabel(String),
    #[error("Unknown label: {0}")]
    UnknownLabel(String),
    #[error("Missing address for label: {0}")]
    MissingAddress(String),
}

// This has to be done inline because the compiler is a bitch
pub fn resolve_program(program: &mut [Instruction]) -> Result<(), ResolveError> {
    let mut labels: HashMap<String, Instruction> = HashMap::new();
    info!("Resolving program...");
    for i in 0..program.len() {
        if program[i].opcode == _Label {
            let label: String = program[i].operands[0].to_string();
            if labels.contains_key(&label) {
                return Err(ResolveError::InvalidLabel(label));
            }

            trace!("Found label at address {}: .{}", program[i].clone().location.unwrap(), label);

            if program[i].location.is_none() {
                return Err(ResolveError::MissingAddress(label));
            }

            labels.insert(label, program[i].clone());
        }
    }
    trace!("All {} labels found", labels.len());
    trace!("Labels: {:?}", labels.keys());
    trace!("Binding labels");

    for i in 0..program.len() {
        if program[i].opcode == _Label {
            continue;
        }
        for j in 0..program[i].operands.len() {
            let operand = &mut program[i].operands[j];
            if let Operand::Label(label) = operand {
                match labels.get(&label.name) {
                    None => return Err(ResolveError::UnknownLabel(label.name.clone())),
                    Some(bind) => label.set_address(bind)
                }
            }
        }
    }
    trace!("All {} offsets bound", labels.len());
    Ok(())
}