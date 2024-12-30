use crate::symbols::{opcodes::Opcode, instruction::Instruction};

pub fn layout_program(program: &mut Vec<Instruction>) {
    info!("Laying out program...");
    let mut current_address = 0;
    for instruction in program {
        if instruction.opcode == Opcode::_Definition {
            continue;
        }

        if instruction.opcode == Opcode::_Label {
            instruction.address = Some(current_address);
            continue;
        }

        instruction.address = Some(current_address);
        current_address += 2;
    }
    debug!("Program size: {} bytes", current_address);
}