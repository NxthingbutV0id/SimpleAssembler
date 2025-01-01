use custom_error::custom_error;
use crate::symbols::{opcodes::Opcode, opcodes::Opcode::*, instruction::Instruction};
use crate::symbols::operands::immediate::Immediate;
use crate::symbols::operands::Operand;
use crate::symbols::operands::register::Register::R0;

pub struct InstructionEncoder {
    encoding: Option<u16>,
    total: u16
}

custom_error! {pub EncodingError
    ValueOutOfBounds = "Value out of bounds",
    ValueOverflow = "Value overflow",
    InvalidOperand = "Invalid operand",
    AddressOutOfBounds = "Address out of bounds",
    UnboundDefinition = "Unbound definition",
    UnboundOffset = "Unbound offset",
    InvalidOffset = "Invalid offset"
}

impl InstructionEncoder {
    pub fn new() -> InstructionEncoder {
        InstructionEncoder {
            encoding: None,
            total: 0
        }
    }

    pub fn encode_program(&mut self, program: &mut Vec<Instruction>) -> Result<(), EncodingError> {
        info!("Encoding program...");
        for i in 0..program.len() {
            self.encoding = Some(0);
            self.encode_instruction(&mut program[i])?;
            program[i].encoding = self.encoding;
            if program[i].encoding.is_some() {
                trace!("Instruction encoded: {}", program[i]);
            }
        }
        debug!("Encoded {} instructions", self.total);
        Ok(())
    }

    fn encode_instruction(&mut self, instruction: &mut Instruction) -> Result<(), EncodingError> {
        let opcode = &instruction.opcode;
        match opcode {
            NOP | HLT | RET => {
                self.encode_opcode(&instruction.opcode)?;
                self.total += 1;
                Ok(())
            },
            ADD | SUB | NOR | AND | XOR => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&instruction.operands[1])?;
                self.encode_c(&instruction.operands[2])?;
                self.total += 1;
                Ok(())
            },
            RSH => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_c(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            },
            LDI | ADI => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_imm(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            },
            JMP | CAL => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_addr(&instruction.operands[0])?;
                self.total += 1;
                Ok(())
            },
            BRH => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_cond(&instruction.operands[0])?;
                self.encode_addr(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            },
            LOD | STR => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&instruction.operands[1])?;
                self.encode_offset(instruction.operands.get(2))?;
                self.total += 1;
                Ok(())
            }
            CMP => {
                self.encode_opcode(&SUB)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&instruction.operands[1])?;
                self.encode_c(&Operand::Register(R0))?;
                self.total += 1;
                Ok(())
            }
            MOV => {
                self.encode_opcode(&ADD)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&Operand::Register(R0))?;
                self.encode_c(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            }
            LSH => {
                self.encode_opcode(&ADD)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&instruction.operands[0])?;
                self.encode_c(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            }
            INC => {
                self.encode_opcode(&ADI)?;
                self.encode_a(&instruction.operands[0])?;
                let imm = Immediate::new(1).unwrap();
                self.encode_imm(&Operand::from(imm))?;
                self.total += 1;
                Ok(())
            }
            DEC => {
                self.encode_opcode(&ADI)?;
                self.encode_a(&instruction.operands[0])?;
                let imm = Immediate::new(-1).unwrap();
                self.encode_imm(&Operand::from(imm))?;
                self.total += 1;
                Ok(())
            }
            NOT => {
                self.encode_opcode(&NOR)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&Operand::Register(R0))?;
                self.encode_c(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            }
            NEG => {
                self.encode_opcode(&SUB)?;
                self.encode_a(&Operand::Register(R0))?;
                self.encode_b(&instruction.operands[0])?;
                self.encode_c(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            }
            _ => {
                self.encoding = None;
                Ok(())
            }
        }
    }

    fn encode_opcode(&mut self, opcode: &Opcode) -> Result<(), EncodingError> {
        self.encode_bits(12, 4, *opcode as u16)?;
        Ok(())
    }

    fn encode_a(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        if let Operand::Register(reg) = operand {
            self.encode_bits(8, 4, *reg as u16)?;
            Ok(())
        } else {
            error!("Expected register, got {}", operand);
            Err(EncodingError::InvalidOperand)
        }
    }

    fn encode_b(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        if let Operand::Register(reg) = operand {
            self.encode_bits(4, 4, *reg as u16)?;
            Ok(())
        } else {
            error!("Expected register, got {}", operand);
            Err(EncodingError::InvalidOperand)
        }
    }

    fn encode_c(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        if let Operand::Register(reg) = operand {
            self.encode_bits(0, 4, *reg as u16)?;
            Ok(())
        } else {
            error!("Expected register, got {}", operand);
            Err(EncodingError::InvalidOperand)
        }
    }

    fn encode_imm(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        match operand { 
            Operand::Immediate(imm) => {
                self.encode_bits(0, 8, imm.value() as u16)?;
                Ok(())
            },
            Operand::Definition(definition) => {
                let imm = match definition.value {
                    Some(i) => i,
                    None => {
                        error!("Definition {} does not have a value", definition.name);
                        return Err(EncodingError::UnboundDefinition);
                    }
                };
                self.encode_bits(0, 8, imm.value() as u16)?;
                Ok(())
            }
            Operand::Port(port) => {
                self.encode_bits(0, 8, *port as u16)?;
                Ok(())
            },
            _ => {
                error!("Expected immediate or definition, got {}", operand);
                Err(EncodingError::InvalidOperand)
            }
        }
    }

    fn encode_cond(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        match operand { 
            Operand::Condition(cond) => {
                self.encode_bits(10, 2, *cond as u16)?;
                Ok(())
            },
            _ => {
                error!("Expected condition, got {}", operand);
                Err(EncodingError::InvalidOperand)
            }
        }
    }

    fn encode_addr(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        match operand { 
            Operand::Address(addr) => {
                self.encode_bits(0, 10, addr.value())?;
                Ok(())
            },
            Operand::Offset(offset) => {
                let o = offset.clone();
                let b = match o.binding {
                    Some(b) => b,
                    None => {
                        error!("Offset {} does not have a binding", o.name);
                        return Err(EncodingError::UnboundOffset);
                    }
                };

                let a = match b.address {
                    Some(a) => a,
                    None => {
                        error!("Offset {} does not have an address", o.name);
                        return Err(EncodingError::InvalidOffset);
                    }
                };


                self.encode_bits(0, 4, a)?;
                Ok(())
            },
            _ => {
                error!("Expected address, got {}", operand);
                Err(EncodingError::InvalidOperand)
            }
        }
    }

    fn encode_offset(&mut self, operand: Option<&Operand>) -> Result<(), EncodingError> {
        match operand {
            Some(Operand::Immediate(imm)) => {
                self.encode_bits(0, 4, imm.value() as u16)?;
                Ok(())
            },
            Some(_) => {
                error!("Expected immediate, got {}", operand.unwrap());
                Err(EncodingError::InvalidOperand)
            },
            None => {
                self.encode_bits(0, 4, 0)?;
                Ok(())
            }
        }
    }

    fn encode_bits(&mut self, offset: u16, length: u16, value: u16) -> Result<(), EncodingError> {
        if offset + length > 16 {
            error!("Unable to write outside 16-bit");
            return Err(EncodingError::ValueOutOfBounds);
        }
        if value >= 1 << length {
            error!("The value of {value} is too large to fit in {length} bits");
            return Err(EncodingError::ValueOverflow);
        }

        let mask: u16 = ((1 << length) - 1) << offset;
        let mut current_encoding = self.encoding.unwrap();
        current_encoding &= !mask;
        current_encoding |= value << offset;
        self.encoding = Some(current_encoding);
        Ok(())
    }
}