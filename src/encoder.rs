use custom_error::custom_error;
use crate::symbols::{opcodes::Opcode, opcodes::Opcode::*, operands::Operand, instruction::Instruction};
use crate::symbols::operands::Register::R0;

pub struct InstructionEncoder {
    encoding: Option<u16>,
    total: u16
}

custom_error! {pub EncodingError
    ValueOutOfBounds = "Value out of bounds",
    ValueOverflow = "Value overflow",
    InvalidOperand = "Invalid operand",
    AddressOutOfBounds = "Address out of bounds"
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
                self.encode_imm8(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            },
            JMP | CAL => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_jmp_addr(&instruction.operands[0])?;
                self.total += 1;
                Ok(())
            },
            BRH => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_cond(&instruction.operands[0])?;
                self.encode_brh_addr(&instruction.operands[1])?;
                self.total += 1;
                Ok(())
            },
            LOD | STR => {
                self.encode_opcode(&instruction.opcode)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_b(&instruction.operands[1])?;
                self.encode_simm4(&instruction.operands[2])?;
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
                self.encode_imm8(&Operand::Immediate(1))?;
                self.total += 1;
                Ok(())
            }
            DEC => {
                self.encode_opcode(&ADI)?;
                self.encode_a(&instruction.operands[0])?;
                self.encode_imm8(&Operand::Immediate(-1))?;
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

    fn encode_imm8(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        let value = self.check_imm(operand, -128, 256)?;
        self.encode_bits(0, 8, value)?;
        Ok(())
    }

    fn check_imm(&self, operand: &Operand, min: i16, max: i16) -> Result<u16, EncodingError> {
        if let Operand::Address(addr) =  operand {
            let imm = *addr as i16;
            if imm < min || imm >= max {
                error!("Address out of bounds: {}", addr);
                return Err(EncodingError::AddressOutOfBounds);
            } else if imm & 1 == 1 {
                warn!("Memory Misalignment: {}, LSB is ignored", addr);
            }
            Ok(*addr)
        } else if let Operand::Immediate(imm) = operand {
            if *imm < min || *imm >= max {
                error!("Immediate value out of range: {}", imm);
                return Err(EncodingError::ValueOutOfBounds);
            }
            Ok((*imm & 0xFF) as u16)
        } else if let Operand::Offset(offset) = operand {
            let imm = offset.offset.unwrap();
            if imm >> 1 >= max as u16 {
                error!("Offset value out of range: {} (program too big?)", imm);
                return Err(EncodingError::ValueOutOfBounds);
            }
            Ok(imm >> 1)
        } else if let Operand::Port(port) = operand {
            Ok((*port as u8) as u16)
        } else if let Operand::Character(c) = operand {
            Ok((*c as u8) as u16)
            // While technically incorrect as the original CPU did not use ascii encoding
            // this is fine for now
        } else {
            error!("{} is not an immediate value", operand);
            Err(EncodingError::InvalidOperand)
        }
    }

    fn encode_cond(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        if let Operand::Condition(cond) = operand {
            self.encode_bits(10, 2, *cond as u16)?;
            Ok(())
        } else {
            error!("Expected condition, got {}", operand);
            Err(EncodingError::InvalidOperand)
        }
    }

    fn encode_jmp_addr(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        let value = self.check_imm(operand, 0, 1 << 12)?;
        self.encode_bits(0, 12, value)?;
        Ok(())
    }

    fn encode_brh_addr(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        let imm = self.check_imm(operand, 0, 1 << 10)?;
        self.encode_bits(0, 10, imm)?;
        Ok(())
    }

    fn encode_simm4(&mut self, operand: &Operand) -> Result<(), EncodingError> {
        let value = self.check_imm(operand, -8, 16)?;
        self.encode_bits(0, 4, value)?;
        Ok(())
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