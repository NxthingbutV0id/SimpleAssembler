use crate::assembler::error::EncodingError;
use crate::symbols::{opcodes::Opcode, opcodes::Opcode::*, instruction::Instruction};
use crate::symbols::operands::immediate::Immediate;
use crate::symbols::operands::Operand;
use crate::symbols::operands::register::Register::R0;

pub struct InstructionEncoder {
    encoding: Option<u16>,
    total: u16
}

impl InstructionEncoder {
    pub fn new() -> InstructionEncoder {
        InstructionEncoder {
            encoding: None,
            total: 0
        }
    }

    pub fn encode_program(&mut self, program: &mut Vec<Instruction>) -> Result<(), EncodingError> {
        for i in 0..program.len() {
            self.encoding = Some(0);
            trace!("Encoding instruction: {}", program[i]);
            self.encode_instruction(&mut program[i])?;
            program[i].encoding = self.encoding;
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
        match operand {
            Operand::Register(reg) => {
                self.encode_bits(8, 4, *reg as u16)?;
                Ok(())
            },
            _ => {
                Err(EncodingError::InvalidOperand{
                    expected: "register".to_string(),
                    found: operand.clone(),
                })
            }
        }
    }

    fn encode_b(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        match operand { 
            Operand::Register(reg) => {
                self.encode_bits(4, 4, *reg as u16)?;
                Ok(())
            },
            _ => {
                Err(EncodingError::InvalidOperand{
                    expected: "register".to_string(),
                    found: operand.clone(),
                })
            }
        }
    }

    fn encode_c(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        match operand { 
            Operand::Register(reg) => {
                self.encode_bits(0, 4, *reg as u16)?;
                Ok(())
            },
            _ => {
                Err(EncodingError::InvalidOperand{
                    expected: "register".to_string(),
                    found: operand.clone(),
                })
            }
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
                        return Err(EncodingError::UnboundDefinition(definition.name.clone()));
                    }
                };
                self.encode_bits(0, 8, imm.value() as u16)?;
                Ok(())
            }
            Operand::Port(port) => {
                self.encode_bits(0, 8, *port as u16)?;
                Ok(())
            },
            Operand::Character(ch) => {
                self.encode_bits(0, 8, *ch as u16)?;
                Ok(())
            },
            _ => {
                Err(EncodingError::InvalidOperand{
                    expected: "immediate".to_string(),
                    found: operand.clone(),
                })
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
                Err(EncodingError::InvalidOperand{
                    expected: "condition".to_string(),
                    found: operand.clone(),
                })
            }
        }
    }

    fn encode_addr(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        match operand { 
            Operand::Address(addr) => {
                self.encode_bits(0, 10, addr.value())?;
                Ok(())
            },
            Operand::Label(label) => {
                let o = label.clone();
                let b = match o.binding {
                    Some(b) => b,
                    None => {
                        error!("Offset {} does not have a binding", o.name);
                        return Err(EncodingError::UnboundOffset(label.clone()));
                    }
                };

                let a = match b.address {
                    Some(a) => a,
                    None => {
                        error!("Offset {} does not have an address", o.name);
                        return Err(EncodingError::InvalidOffset(label.clone()));
                    }
                };


                self.encode_bits(0, 10, a.value())?;
                Ok(())
            }
            _ => {
                Err(EncodingError::InvalidOperand {
                    expected: "address".to_string(),
                    found: operand.clone(),
                })
            }
        }
    }

    fn encode_offset(&mut self, operand: Option<&Operand>) -> Result<(), EncodingError> {
        match operand {
            Some(Operand::Offset(value)) => {
                self.encode_bits(0, 4, value.value() as u16)?;
                Ok(())
            },
            Some(_) => {
                Err(EncodingError::InvalidOperand{
                    expected: "optional immediate".to_string(),
                    found: operand.unwrap().clone(),
                })
            },
            None => {
                self.encode_bits(0, 4, 0)?;
                Ok(())
            }
        }
    }

    fn encode_bits(&mut self, offset: u16, length: u16, value: u16) -> Result<(), EncodingError> {
        if offset + length > 16 {
            return Err(EncodingError::ValueOutOfBounds(value));
        }
        if value >= 1 << length {
            debug!("Errored after {} instructions", self.total);
            return Err(EncodingError::ValueOverflow{ value, length });
        }

        let mask: u16 = ((1 << length) - 1) << offset;
        let mut current_encoding = self.encoding.unwrap();
        current_encoding &= !mask;
        current_encoding |= value << offset;
        self.encoding = Some(current_encoding);
        Ok(())
    }
}