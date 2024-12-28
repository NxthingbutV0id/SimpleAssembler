use crate::symbols::{opcodes::Opcode, instruction::Instruction};

pub struct Layout {
    current_address: u16,
}

impl Layout {
    pub fn new() -> Layout {
        Layout {
            current_address: 0
        }
    }

    pub fn layout_program(&mut self, program: &mut Vec<Instruction>) {
        info!("Laying out program...");
        for instruction in program {
            self.layout_instruction(instruction);
            trace!("{}", instruction);
        }
        debug!("Program size: {} bytes", self.current_address);
    }

    fn layout_instruction(&mut self, instruction: &mut Instruction) {
        if instruction.opcode == Opcode::_Definition {
            return;
        }

        if instruction.opcode == Opcode::_Label {
            instruction.address = Some(self.current_address);
            return;
        }

        instruction.address = Some(self.current_address);
        self.current_address += 2;
    }
}