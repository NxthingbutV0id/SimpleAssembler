use std::collections::HashMap;
use crate::symbols::instruction::Instruction;
use crate::symbols::opcodes::Opcode::_Label;
use crate::symbols::operands::{Operand};

pub struct Resolver {
    pub labels: HashMap<String, Instruction>,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            labels: HashMap::new(),
        }
    }

    pub fn resolve_program(&mut self, program: &mut Vec<Instruction>) {
        info!("Resolving program...");
        for i in 0..program.len() {
            if program[i].opcode == _Label {
                let label: String = program[i].operands[0].to_string();
                if self.labels.contains_key(&label) {
                    error!("Label .{} already defined", label);
                    panic!();
                }

                trace!("Found label at address {:04X}: .{}", program[i].clone().address.unwrap(), label);
                self.labels.insert(label, program[i].clone());
            }
        }
        trace!("All labels found");
        trace!("Labels: {:?}", self.labels.keys());
        trace!("Binding labels to offsets");

        for i in 0..program.len() {
            if program[i].opcode == _Label {
                trace!("Skipping label definition");
                continue;
            }
            for j in 0..program[i].operands.len() {
                let operand = &mut program[i].operands[j];
                if let Operand::Offset(offset) = operand {
                    let label_inst = self.labels.get(&offset.name);
                    if label_inst.is_some() {
                        if label_inst.unwrap().address.is_none() {
                            error!("Instruction {} is missing address, skipping", label_inst.unwrap());
                            continue;
                        }
                        trace!("Found binding: .{}", offset.name);
                        offset.binding = Some(label_inst.unwrap().clone());
                    } else {
                        error!("Label .{} not found", offset.name);
                        panic!("LabelNotFound");
                    }
                }
            }
        }
        trace!("Program resolved");
    }
}