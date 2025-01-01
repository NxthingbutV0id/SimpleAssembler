use std::collections::HashMap;
use custom_error::custom_error;
use crate::symbols::instruction::Instruction;
use crate::symbols::opcodes::Opcode::_Label;
use crate::symbols::operands::Operand;

custom_error! {pub ResolveError
    LabelNotFound{name:String} = "Label .{name} not found in program",
    LabelAlreadyDefined{name:String} = "Label .{name} already defined",
    MissingAddress{instruction:Instruction} = "Instruction {instruction} is missing address"
}

// This has to be done inline because the compiler is a bitch
pub fn resolve_program(program: &mut Vec<Instruction>) -> Result<(), ResolveError> {
    let mut labels: HashMap<String, Instruction> = HashMap::new();
    info!("Resolving program...");
    for i in 0..program.len() {
        if program[i].opcode == _Label {
            let label: String = program[i].operands[0].to_string();
            if labels.contains_key(&label) {
                return Err(ResolveError::LabelAlreadyDefined { name: label });
            }

            trace!("Found label at address {:04X}: .{}", program[i].clone().address.unwrap(), label);

            if program[i].address.is_none() {
                return Err(ResolveError::MissingAddress { instruction: program[i].clone() });
            }

            labels.insert(label, program[i].clone());
        }
    }
    trace!("All {} labels found", labels.len());
    trace!("Labels: {:?}", labels.keys());
    trace!("Binding labels to offsets");

    for i in 0..program.len() {
        if program[i].opcode == _Label {
            continue;
        }
        for j in 0..program[i].operands.len() {
            let operand = &mut program[i].operands[j];
            if let Operand::Offset(offset) = operand {
                let label_inst = labels.get(&offset.name);
                if label_inst.is_some() {
                    if label_inst.unwrap().address.is_none() {
                        return Err(ResolveError::MissingAddress { instruction: label_inst.unwrap().clone() } );
                    }
                    trace!("Found binding: .{}", offset.name);
                    offset.binding = Some(label_inst.unwrap().clone());
                } else {
                    return Err(ResolveError::LabelNotFound { name: offset.name.clone() });
                }
            }
        }
    }
    trace!("All {} offsets bound", labels.len());
    Ok(())
}