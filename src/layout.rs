use crate::architecture::batpu2::instruction::Instruction;
use crate::architecture::batpu2::opcode::Opcode;
use crate::architecture::batpu2::operand::immediate::Address;

pub fn layout_program(program: &mut [Instruction]) {
    info!("Laying out program...");
    let mut current_address = 0;
    for instruction in program {
        match Address::new(current_address) {
            Some(addr) => {
                match instruction.opcode {
                    Opcode::_Label => {
                        instruction.location = Some(addr);
                        continue;
                    },
                    Opcode::_Definition => {
                        continue;
                    },
                    _ => {
                        instruction.location = Some(addr);
                        current_address += 2;
                    }
                }
            },
            None => panic!("Attempted to assign a invalid address: {:04X}", current_address)
        }
    }
    debug!("Program size: {} bytes", current_address);
}