use std::fmt::Display;

pub enum TokenType {
    // Single character tokens
    Period,
    Comma,
    Equal,
    LessThan,
    Semicolon,
    Hashtag,
    Percent,
    Dash,
    Character,
    EndOfFile,
    
    // Multi-character tokens
    DoubleSlash, 
    MultiLineCommentStart, 
    MultiLineCommentEnd, 
    BangEqual, 
    GreaterThanOrEqual, 
    DoubleEqual,
    
    // Literals
    Number, Identifier,
    
    // Keywords
    DEFINE, 
    // Opcodes
    NOP, HLT, ADD, SUB,
    NOR, AND, XOR, RSH,
    LDI, ADI, JMP, BRH, 
    CAL, RET, LOD, STR,
    CMP, MOV, LSH, INC,
    DEC, NOT, NEG,
    // Ports
    PixelX,
    PixelY,
    DrawPixel,
    ClearPixel,
    LoadPixel,
    BufferScreen,
    ClearScreenBuffer,
    WriteChar,
    BufferChars,
    ClearCharsBuffer,
    ShowNumber,
    ClearNumber,
    SignedMode,
    UnsignedMode,
    RNG,
    ControllerInput,
    // Registers
    R0, R1, R2, R3,
    R4, R5, R6, R7,
    R8, R9, R10, R11,
    R12, R13, R14, R15,
}

pub struct Token<T> {
    pub t_type: TokenType,
    pub lexeme: String,
    pub literal: Option<T>,
    pub line: usize,
    pub column: usize
}

impl<T: Display> Display for Token<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.t_type, self.lexeme, self.literal.as_ref().unwrap())
    }
}

// TODO: Trying something different here...(https://craftinginterpreters.com/scanning.html)

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenType::Period => write!(f, "."),
            TokenType::Comma => write!(f, ","),
            TokenType::Equal => write!(f, "="),
            TokenType::LessThan => write!(f, "<"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Hashtag => write!(f, "#"),
            TokenType::Percent => write!(f, "%"),
            TokenType::Dash => write!(f, "-"),
            TokenType::Character => write!(f, "Character"),
            TokenType::EndOfFile => write!(f, "EndOfFile"),
            TokenType::DoubleSlash => write!(f, "//"),
            TokenType::MultiLineCommentStart => write!(f, "/*"),
            TokenType::MultiLineCommentEnd => write!(f, "*/"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::GreaterThanOrEqual => write!(f, ">="),
            TokenType::DoubleEqual => write!(f, "=="),
            TokenType::Number => write!(f, "Number"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::DEFINE => write!(f, "DEFINE"),
            TokenType::NOP => write!(f, "NOP"),
            TokenType::HLT => write!(f, "HLT"),
            TokenType::ADD => write!(f, "ADD"),
            TokenType::SUB => write!(f, "SUB"),
            TokenType::NOR => write!(f, "NOR"),
            TokenType::AND => write!(f, "AND"),
            TokenType::XOR => write!(f, "XOR"),
            TokenType::RSH => write!(f, "RSH"),
            TokenType::LDI => write!(f, "LDI"),
            TokenType::ADI => write!(f, "ADI"),
            TokenType::JMP => write!(f, "JMP"),
            TokenType::BRH => write!(f, "BRH"),
            TokenType::CAL => write!(f, "CAL"),
            TokenType::RET => write!(f, "RET"),
            TokenType::LOD => write!(f, "LOD"),
            TokenType::STR => write!(f, "STR"),
            TokenType::CMP => write!(f, "CMP"),
            TokenType::MOV => write!(f, "MOV"),
            TokenType::LSH => write!(f, "LSH"),
            TokenType::INC => write!(f, "INC"),
            TokenType::DEC => write!(f, "DEC"),
            TokenType::NOT => write!(f, "NOT"),
            TokenType::NEG => write!(f, "NEG"),
            TokenType::PixelX => write!(f, "PixelX"),
            TokenType::PixelY => write!(f, "PixelY"),
            TokenType::DrawPixel => write!(f, "DrawPixel"),
            TokenType::ClearPixel => write!(f, "ClearPixel"),
            TokenType::LoadPixel => write!(f, "LoadPixel"),
            TokenType::BufferScreen => write!(f, "BufferScreen"),
            TokenType::ClearScreenBuffer => write!(f, "ClearScreenBuffer"),
            TokenType::WriteChar => write!(f, "WriteChar"),
            TokenType::BufferChars => write!(f, "BufferChars"),
            TokenType::ClearCharsBuffer => write!(f, "ClearCharsBuffer"),
            TokenType::ShowNumber => write!(f, "ShowNumber"),
            TokenType::ClearNumber => write!(f, "ClearNumber"),
            TokenType::SignedMode => write!(f, "SignedMode"),
            TokenType::UnsignedMode => write!(f, "UnsignedMode"),
            TokenType::RNG => write!(f, "RNG"),
            TokenType::ControllerInput => write!(f, "ControllerInput"),
            TokenType::R0 => write!(f, "R0"),
            TokenType::R1 => write!(f, "R1"),
            TokenType::R2 => write!(f, "R2"),
            TokenType::R3 => write!(f, "R3"),
            TokenType::R4 => write!(f, "R4"),
            TokenType::R5 => write!(f, "R5"),
            TokenType::R6 => write!(f, "R6"),
            TokenType::R7 => write!(f, "R7"),
            TokenType::R8 => write!(f, "R8"),
            TokenType::R9 => write!(f, "R9"),
            TokenType::R10 => write!(f, "R10"),
            TokenType::R11 => write!(f, "R11"),
            TokenType::R12 => write!(f, "R12"),
            TokenType::R13 => write!(f, "R13"),
            TokenType::R14 => write!(f, "R14"),
            TokenType::R15 => write!(f, "R15"),
        }
    }
}