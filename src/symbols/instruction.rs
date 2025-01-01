use std::fmt::Display;
use crate::symbols::opcodes::Opcode;
use crate::symbols::operands::address::Address;
use crate::symbols::operands::condition::Condition;
use crate::symbols::operands::definition::Definition;
use crate::symbols::operands::immediate::Immediate;
use crate::symbols::operands::offset::Offset;
use crate::symbols::operands::Operand;
use crate::symbols::operands::port::Port;
use crate::symbols::operands::register::Register;

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
    pub address: Option<u16>,
    pub encoding: Option<u16>
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode,
            operands: Vec::new(),
            address: None,
            encoding: None
        }
    }

    pub fn add_operand(&mut self, operand: Operand) {
        self.operands.push(operand);
    }

    pub fn add_register(&mut self, reg: Register) {
        self.add_operand(Operand::Register(reg));
    }
    
    pub fn add_condition(&mut self, cond: Condition) {
        self.add_operand(Operand::Condition(cond));
    }

    pub fn add_immediate(&mut self, imm: Immediate) {
        self.add_operand(Operand::Immediate(imm));
    }

    pub fn add_label(&mut self, label: String) {
        self.add_operand(Operand::Label(label));
    }

    pub fn add_definition(&mut self, def: Definition) {
        self.add_operand(Operand::Definition(def));
    }
    
    pub fn add_offset(&mut self, off: Offset) {
        self.add_operand(Operand::Offset(off));
    }
    
    pub fn add_port(&mut self, port: Port) {
        self.add_operand(Operand::Port(port));
    }
    
    pub fn add_address(&mut self, addr: Address) {
        self.add_operand(Operand::Address(addr));
    }
    
    pub fn add_character(&mut self, ch: char) {
        self.add_operand(Operand::Character(ch));
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut text: String = String::new();

        if self.opcode == Opcode::_Definition {
            text.push_str("define");
        } else if self.opcode == Opcode::_Label {
            text.push_str(".");
        } else {
            text.push_str(&format!("{}", self.opcode));
        }

        for op in &self.operands {
            let op_text = if let Operand::Label(label) = op {
                format!("{}", *label)
            } else {
                format!(" {}", *op)
            };
            text.push_str(&op_text);
        }
        write!(f, "{}", text)
    }
}