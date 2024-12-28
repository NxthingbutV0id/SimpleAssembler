use crate::symbols::{opcodes::Opcode, opcodes::Opcode::*, operands::Operand, instruction::Instruction};
use crate::symbols::operands::Register::R0;

pub struct InstructionEncoder {
    encoding: Option<u16>
}

impl InstructionEncoder {
    pub fn new() -> InstructionEncoder {
        InstructionEncoder {
            encoding: None
        }
    }

    pub fn encode_program(&mut self, program: &mut Vec<Instruction>) {
        info!("Encoding program...");
        for i in 0..program.len() {
            self.encoding = Some(0);
            self.encode_instruction(&mut program[i]);
            program[i].encoding = self.encoding;
            trace!("Instruction encoded:  {}", program[i]);
        }
    }

    fn encode_instruction(&mut self, instruction: &mut Instruction) {
        let opcode = &instruction.opcode;
        match opcode {
            NOP | HLT | RET => {
                self.encode_opcode(&instruction.opcode);
            },
            ADD | SUB | NOR | AND | XOR => {
                self.encode_opcode(&instruction.opcode);
                self.encode_a(&instruction.operands[0]);
                self.encode_b(&instruction.operands[1]);
                self.encode_c(&instruction.operands[2]);
            },
            RSH => {
                self.encode_opcode(&instruction.opcode);
                self.encode_a(&instruction.operands[0]);
                self.encode_c(&instruction.operands[1]);
            },
            LDI | ADI => {
                self.encode_opcode(&instruction.opcode);
                self.encode_a(&instruction.operands[0]);
                self.encode_imm8(&instruction.operands[1]);
            },
            JMP | CAL => {
                self.encode_opcode(&instruction.opcode);
                self.encode_jmp_addr(&instruction.operands[0]);
            },
            BRH => {
                self.encode_opcode(&instruction.opcode);
                self.encode_cond(&instruction.operands[0]);
                self.encode_brh_addr(&instruction.operands[1]);
            },
            LOD | STR => {
                self.encode_opcode(&instruction.opcode);
                self.encode_a(&instruction.operands[0]);
                self.encode_b(&instruction.operands[1]);
                self.encode_simm4(&instruction.operands[2]);
            }
            CMP => {
                self.encode_opcode(&SUB);
                self.encode_a(&instruction.operands[0]);
                self.encode_b(&instruction.operands[1]);
                self.encode_c(&Operand::Register(R0));
            }
            MOV => {
                self.encode_opcode(&ADD);
                self.encode_a(&instruction.operands[0]);
                self.encode_b(&Operand::Register(R0));
                self.encode_c(&instruction.operands[1]);
            }
            LSH => {
                self.encode_opcode(&ADD);
                self.encode_a(&instruction.operands[0]);
                self.encode_b(&instruction.operands[0]);
                self.encode_c(&instruction.operands[1]);
            }
            INC => {
                self.encode_opcode(&ADI);
                self.encode_a(&instruction.operands[0]);
                self.encode_imm8(&Operand::Immediate(1));
            }
            DEC => {
                self.encode_opcode(&ADI);
                self.encode_a(&instruction.operands[0]);
                self.encode_imm8(&Operand::Immediate(-1));
            }
            NOT => {
                self.encode_opcode(&NOR);
                self.encode_a(&instruction.operands[0]);
                self.encode_b(&Operand::Register(R0));
                self.encode_c(&instruction.operands[1]);
            }
            NEG => {
                self.encode_opcode(&SUB);
                self.encode_a(&Operand::Register(R0));
                self.encode_b(&instruction.operands[0]);
                self.encode_c(&instruction.operands[1]);
            }
            _ => {
                self.encoding = None;
            }
        }
    }

    fn encode_opcode(&mut self, opcode: &Opcode) {
        self.encode_bits(12, 4, *opcode as u16);
    }

    fn panic_reg(operand: &Operand) {
        error!("Expected register, got {}", operand);
        panic!("InvalidOperand");
    }

    fn encode_a(&mut self, operand: &Operand) {
        if let Operand::Register(reg) = operand {
            self.encode_bits(8, 4, *reg as u16);
        } else {
            Self::panic_reg(operand);
        }
    }

    fn encode_b(&mut self, operand: &Operand) {
        if let Operand::Register(reg) = operand {
            self.encode_bits(4, 4, *reg as u16);
        } else {
            Self::panic_reg(operand);
        }
    }

    fn encode_c(&mut self, operand: &Operand) {
        if let Operand::Register(reg) = operand {
            self.encode_bits(0, 4, *reg as u16);
        } else {
            Self::panic_reg(operand);
        }
    }

    fn encode_imm8(&mut self, operand: &Operand) {
        let value = self.check_imm(operand, -128, 256);
        self.encode_bits(0, 8, value);
    }

    fn check_imm(&self, operand: &Operand, min: i16, max: i16) -> u16 {
        if let Operand::Address(addr) =  operand {
            let imm = *addr as i16;
            if imm < min || imm >= max {
                panic!("Address out of range: {}", addr);
            } else if imm & 1 == 1 {
                warn!("Memory Misalignment: {}, LSB is ignored", addr);
            }
            *addr
        } else if let Operand::Immediate(imm) = operand {
            if *imm < min || *imm >= max {
                panic!("Immediate value out of range: {}", imm);
            }
            (*imm & 0xFF) as u16
        } else if let Operand::Offset(offset) = operand {
            let imm = offset.offset.unwrap();
            if imm >> 1 >= max as u16 {
                panic!("Offset value out of range: {} (program too big?)", imm);
            }
            imm >> 1
        } else if let Operand::Port(port) = operand {
            (*port as u8) as u16
        } else if let Operand::Character(c) = operand {
            (*c as u8) as u16
        } else {
            error!("Cannot convert {} to immediate", operand);
            panic!("InvalidOperand");
        }
    }

    fn encode_cond(&mut self, operand: &Operand) {
        if let Operand::Condition(cond) = operand {
            self.encode_bits(10, 2, *cond as u16);
        } else {
            error!("Expected condition, got {}", operand);
            panic!("InvalidOperand");
        }
    }

    fn encode_jmp_addr(&mut self, operand: &Operand) {
        let value = self.check_imm(operand, 0, 1 << 12);
        self.encode_bits(0, 12, value);
    }

    fn encode_brh_addr(&mut self, operand: &Operand) {
        let imm = self.check_imm(operand, 0, 1 << 10);
        self.encode_bits(0, 10, imm);
    }

    fn encode_simm4(&mut self, operand: &Operand) {
        let value = self.check_imm(operand, -8, 16);
        self.encode_bits(0, 4, value);
    }

    fn encode_bits(&mut self, offset: u16, length: u16, value: u16) {
        if offset + length > 16 {
            error!("Unable to write outside 16-bit");
            panic!("ValueOutOfBounds");
        }
        if value >= 1 << length {
            error!("The value of {value} is too large to fit in {length} bits");
            panic!("ValueOverflow");
        }

        let mask: u16 = ((1 << length) - 1) << offset;
        let mut current_encoding = self.encoding.unwrap();
        current_encoding &= !mask;
        current_encoding |= value << offset;
        self.encoding = Some(current_encoding);
    }
}