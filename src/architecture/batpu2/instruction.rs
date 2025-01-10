use std::fmt::Display;
use crate::architecture::batpu2::opcode::Opcode;
use crate::architecture::batpu2::operand::condition::Condition;
use crate::architecture::batpu2::operand::definition::Definition;
use crate::architecture::batpu2::operand::immediate::{Address, Immediate, Offset};
use crate::architecture::batpu2::operand::label::Label;
use crate::architecture::batpu2::operand::Operand;
use crate::architecture::batpu2::operand::port::Port;
use crate::architecture::batpu2::operand::register::Register;

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub(crate) operands: Vec<Operand>,
    pub location: Option<Address>,
    pub(crate) encoding: Option<u16>
}

/// Types of instructions:
/// {op} {reg}, {reg}, {reg}
/// {CMP|LSH|RSH|MOV} {reg}, {reg}
/// {LOD|STR} {reg}, {reg}, {def|offset}?
/// {LDI|ADI} {reg}, {imm|port|def|char}
/// {BRH} {cond}, {label|addr}
/// {INC|DEC} {reg}
/// {JMP} {label|addr}
/// {NOP|HLT|RET}
///
/// special cases:
/// .{identifier}
/// define {identifier} {value}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode,
            operands: Vec::new(),
            location: None,
            encoding: None
        }
    }

    pub fn add_operand(&mut self, operand: Operand) {
        self.operands.push(operand);
    }

    pub fn add_register(&mut self, reg: Register) {
        self.add_operand(Operand::Reg(reg));
    }

    pub fn add_condition(&mut self, cond: Condition) {
        self.add_operand(Operand::Cond(cond));
    }

    pub fn add_immediate(&mut self, imm: Immediate) {
        self.add_operand(Operand::Imm(imm));
    }

    pub fn add_label_name(&mut self, name: String) {
        self.add_operand(Operand::Name(name));
    }

    pub fn add_definition(&mut self, def: Definition) {
        self.add_operand(Operand::Def(def));
    }

    pub fn add_label(&mut self, label: Label) {
        self.add_operand(Operand::Label(label));
    }

    pub fn add_port(&mut self, port: Port) {
        self.add_operand(Operand::Port(port));
    }

    pub fn add_address(&mut self, addr: Address) {
        self.add_operand(Operand::Addr(addr));
    }

    pub fn add_character(&mut self, ch: char) {
        self.add_operand(Operand::Char(ch));
    }

    pub fn add_offset(&mut self, i4: Offset) {
        self.add_operand(Operand::Offset(i4));
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
            let op_text = if let Operand::Name(name) = op {
                format!("{}", *name)
            } else {
                format!(" {}", *op)
            };
            text.push_str(&op_text);
        }
        write!(f, "{}", text)
    }
}