use thiserror::Error;
use crate::architecture::batpu2::instruction::Instruction;
use crate::architecture::batpu2::opcode::Opcode;
use crate::architecture::batpu2::operand::immediate::{Immediate, Offset};
use crate::architecture::batpu2::operand::label::Label;
use crate::architecture::batpu2::operand::Operand;
use crate::architecture::batpu2::operand::register::Register::R0;

#[derive(Debug, Error)]
pub enum EncodingError {
    #[error("Value {value} is out of bounds for {length} bits")]
    ValueOutOfBounds {
        value: u16,
        length: u16
    },
    #[error("Value {value} is too large for {length} bits")]
    ValueOverflow {
        value: u16,
        length: u16
    },
    #[error("Operand {found} is not a valid {expected}")]
    InvalidOperand {
        expected: String,
        found: Operand
    },
    #[error("Definition {0} is not bound")]
    UnboundDefinition(String),
    #[error("Offset {0} is invalid (-8 <= offset <= 7)")]
    InvalidOffset(i8),
    #[error("Label {0} is invalid")]
    InvalidLabel(Label)
}

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

    pub fn encode_program(&mut self, program: &mut [Instruction]) -> Result<(), EncodingError> {
        for i in 0..program.len() {
            self.encoding = Some(0);
            match program[i].opcode {
                Opcode::_Definition | Opcode::_Label => {},
                _ => {
                    trace!("Encoding instruction: {}", program[i]);
                    self.encode_instruction(&mut program[i])?;
                    program[i].encoding = self.encoding;
                }
            }
        }
        debug!("Encoded {} instructions", self.total);
        Ok(())
    }

    fn encode_instruction(&mut self, instruction: &mut Instruction) -> Result<(), EncodingError> {
        let opcode = &instruction.opcode;
        match opcode {
            Opcode::NOP | Opcode::HLT | Opcode::RET => {
                self.encode_opcode(&instruction.opcode)?;
                self.total += 1;
                Ok(())
            },
            Opcode::ADD | Opcode::SUB | Opcode::NOR | Opcode::AND | Opcode::XOR => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&instruction.operands[1])?;
                self.encode_c(&instruction.operands[2])?;
                self.total += 1;
                Ok(())
            },
            Opcode::RSH => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_c(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            },
            Opcode::LDI | Opcode::ADI => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_imm(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            },
            Opcode::JMP | Opcode::CAL => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_addr(&instruction.operands[0])?;
                self.total += 1;
                Ok(())
            },
            Opcode::BRH => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_cond(&instruction.operands[0])?;
                self.encode_addr(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            },
            Opcode::LOD | Opcode::STR => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&instruction.operands[1])?;
                self.encode_offset(instruction.operands.get(2))?;
                self.total += 1;
                Ok(())
            }
            Opcode::CMP => {
                self.encode_opcode(&Opcode::SUB)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&instruction.operands[1])?;
                self.encode_c(&Operand::Reg(R0))?;
                self.total += 1;
                Ok(())
            }
            Opcode::MOV => {
                self.encode_opcode(&Opcode::ADD)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&Operand::Reg(R0))?;
                self.encode_c(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            }
            Opcode::LSH => {
                self.encode_opcode(&Opcode::ADD)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&instruction.operands[0])?;
                self.encode_c(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            }
            Opcode::INC => {
                self.encode_opcode(&Opcode::ADI)?;
                self.encode_a(&instruction.operands[0])?;
                let imm = Immediate::new(1).unwrap();
                self.encode_imm(&Operand::from(imm))?;
                self.total += 1;
                Ok(())
            }
            Opcode::DEC => {
                self.encode_opcode(&Opcode::ADI)?;
                self.encode_a(&instruction.operands[0])?;
                let imm = Immediate::new(-1).unwrap();
                self.encode_imm(&Operand::from(imm))?;
                self.total += 1;
                Ok(())
            }
            Opcode::NOT => {
                self.encode_opcode(&Opcode::NOR)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&Operand::Reg(R0))?;
                self.encode_c(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            }
            Opcode::NEG => {
                self.encode_opcode(&Opcode::SUB)?;
                self.encode_a(&Operand::Reg(R0))?;
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
            Operand::Reg(reg) => {
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
            Operand::Reg(reg) => {
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
            Operand::Reg(reg) => {
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
            Operand::Imm(imm) => {
                self.encode_bits(0, 8, imm.value() as u16)?;
                Ok(())
            },
            Operand::Def(definition) => {
                let imm = match definition.value {
                    Some(i) => i,
                    None => {
                        return Err(EncodingError::UnboundDefinition(definition.name.clone()));
                    }
                };
                self.encode_bits(0, 8, imm as u16)?;
                Ok(())
            }
            Operand::Port(port) => {
                self.encode_bits(0, 8, *port as u16)?;
                Ok(())
            },
            Operand::Char(ch) => {
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
            Operand::Cond(cond) => {
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
            Operand::Addr(addr) => {
                self.encode_bits(0, 10, addr.value())?;
                Ok(())
            },
            Operand::Label(label) => {
                let o = label.clone();

                let a = match label.get_address() {
                    Some(a) => a,
                    None => {
                        error!("Label {} does not have an address", o.name);
                        return Err(EncodingError::InvalidLabel(label.clone()));
                    }
                };

                self.encode_bits(0, 10, a.value())?;
                Ok(())
            },
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
                self.encode_bits(0, 4, value.encode() as u16)?;
                Ok(())
            },
            Some(Operand::Def(def)) => {
                let imm = match def.value {
                    Some(i) => i,
                    None => {
                        return Err(EncodingError::UnboundDefinition(def.name.clone()));
                    }
                };
                match Offset::new(imm as i8) {
                    Some(i) => {
                        self.encode_bits(0, 4, i.encode() as u16)?;
                        Ok(())
                    },
                    None => {
                        Err(EncodingError::InvalidOffset(imm as i8))
                    }
                }
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
            return Err(EncodingError::ValueOutOfBounds { value, length });
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