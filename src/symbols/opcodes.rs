use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    NOP,
    HLT,
    ADD,
    SUB,
    NOR,
    AND,
    XOR,
    RSH,
    LDI,
    ADI,
    JMP,
    BRH,
    CAL,
    RET,
    LOD,
    STR,
    CMP,
    MOV,
    LSH,
    INC,
    DEC,
    NOT,
    NEG,
    _Label,
    _Definition
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Opcode::NOP => "nop",
            Opcode::HLT => "hlt",
            Opcode::ADD => "add",
            Opcode::SUB => "sub",
            Opcode::NOR => "nor",
            Opcode::AND => "and",
            Opcode::XOR => "xor",
            Opcode::RSH => "rsh",
            Opcode::LDI => "ldi",
            Opcode::ADI => "adi",
            Opcode::JMP => "jmp",
            Opcode::BRH => "brh",
            Opcode::CAL => "cal",
            Opcode::RET => "ret",
            Opcode::LOD => "lod",
            Opcode::STR => "str",
            Opcode::CMP => "cmp",
            Opcode::MOV => "mov",
            Opcode::LSH => "lsh",
            Opcode::INC => "inc",
            Opcode::DEC => "dec",
            Opcode::NOT => "not",
            Opcode::NEG => "neg",
            Opcode::_Label => ".",
            Opcode::_Definition => "define"
        };
        write!(f, "{}", text)
    }
}