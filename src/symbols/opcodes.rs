use std::fmt::Display;
use std::str::FromStr;

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

impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nop" => Ok(Opcode::NOP),
            "hlt" => Ok(Opcode::HLT),
            "add" => Ok(Opcode::ADD),
            "sub" => Ok(Opcode::SUB),
            "nor" => Ok(Opcode::NOR),
            "and" => Ok(Opcode::AND),
            "xor" => Ok(Opcode::XOR),
            "rsh" => Ok(Opcode::RSH),
            "ldi" => Ok(Opcode::LDI),
            "adi" => Ok(Opcode::ADI),
            "jmp" => Ok(Opcode::JMP),
            "brh" => Ok(Opcode::BRH),
            "cal" => Ok(Opcode::CAL),
            "ret" => Ok(Opcode::RET),
            "lod" => Ok(Opcode::LOD),
            "str" => Ok(Opcode::STR),
            "cmp" => Ok(Opcode::CMP),
            "mov" => Ok(Opcode::MOV),
            "lsh" => Ok(Opcode::LSH),
            "inc" => Ok(Opcode::INC),
            "dec" => Ok(Opcode::DEC),
            "not" => Ok(Opcode::NOT),
            "neg" => Ok(Opcode::NEG),
            _ => Err(())
        }
    }
}