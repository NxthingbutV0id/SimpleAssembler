use crate::symbols::{opcodes::Opcode, instruction::Instruction};
use crate::symbols::operands::address::Address;

pub fn layout_program(program: &mut [Instruction]) {
    info!("Laying out program...");
    let mut current_address = 0;
    for instruction in program {
        match Address::new(current_address) {
            Some(addr) => {
                match instruction.opcode { 
                    Opcode::_Label => {
                        instruction.address = Some(addr);
                        continue;
                    },
                    Opcode::_Definition => {
                        continue;
                    },
                    _ => {
                        instruction.address = Some(addr);
                        current_address += 2;
                    }
                }
            },
            None => panic!("Attempted to assign a invalid address: {:04X}", current_address)
        }
    }
    debug!("Program size: {} bytes", current_address);
}