use std::fmt::Display;
use crate::symbols::opcodes::Opcode;
use crate::symbols::operands::Operand;
use crate::symbols::operands::Operand::{Label, Offset};

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
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut text: String = if self.address.is_some() {
            format!("{:04X} |", self.address.unwrap())
        } else {
            "---- |".to_string()
        };
        if self.encoding.is_some() {
            text.push_str(format!(" {:016b} |", self.encoding.unwrap()).as_str());
        } else {
            text.push_str(" ---------------- |");
        }

        if self.opcode == Opcode::_Definition {
            text.push_str("define");
        } else if self.opcode == Opcode::_Label {
            text.push_str(".");
        } else {
            text.push_str(&format!("    {}", self.opcode));
        }

        for op in &self.operands {
            let op_text = if let Label(label) = op {
                format!("{}", *label)
            } else if let Offset(offset) = op {
                format!(" .{}", *offset)
            } else if let Operand::Definition(def) = op {
                format!(" {}", *def)
            } else {
                format!(" {}", *op)
            };
            text.push_str(&op_text);
        }
        write!(f, "{}", text)
    }
}