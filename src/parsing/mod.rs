use std::fs;
use std::path::Path;
use nom::error::convert_error;
use nom::Finish;
use crate::assembler::error::ParsingError;
use crate::symbols::instruction::Instruction;
use crate::parsing::parser::parse_program;
use crate::symbols::opcodes::Opcode;

pub(crate) mod basic;
pub(crate) mod helper;
mod complex;
mod parser;
mod token;

const KEYWORDS: [&str; 76] = [
    "define",
    // Opcodes (1-23)
    "nop", "hlt", "add", "sub",
    "nor", "and", "xor", "rsh",
    "ldi", "adi", "jmp", "brh",
    "cal", "ret", "lod", "str",
    "cmp", "mov", "lsh", "inc",
    "dec", "not", "neg",
    // Registers (24-39)
    "r0", "r1", "r2", "r3",
    "r4", "r5", "r6", "r7",
    "r8", "r9", "r10", "r11",
    "r12", "r13", "r14", "r15",
    // Conditions (40-55)
    "zs", "zc", "cs", "cc",
    "lt", "ge", "eq", "ne",
    "=", "!=", ">=", "<",
    "nc", "c", "z", "nz",
    "notcarry", "carry", "zero", "notzero",
    // Ports (56-75)
    "pixel_x", "pixel_y", "draw_pixel", "clear_pixel",
    "load_pixel", "buffer_screen", "clear_screen_buffer", "write_char", "buffer_chars",
    "clear_chars_buffer", "show_number", "clear_number", "signed_mode", "unsigned_mode",
    "rng", "controller_input"
];

pub fn parse_file(file: &str) -> Result<Vec<Instruction>, ParsingError> {
    info!("Parsing {}...", file);
    let path = Path::new(file);
    if !path.exists() {
        return Err(ParsingError::FileNotFound(file.to_string()));
    }

    if !path.is_file() {
        return Err(ParsingError::InvalidFile(file.to_string()));
    }

    if path.extension().unwrap() != "asm" &&
        path.extension().unwrap() != "as" &&
        path.extension().unwrap() != "s" {
        return Err(ParsingError::InvalidExtension(file.to_string()));
    }


    let input = fs::read_to_string(path);
    match input {
        Ok(input) => {
            if input.is_empty() {
                warn!("File is empty: {}", file);
                return Ok(Vec::new());
            }

            let program = parse_program(&input).finish();
            match program { 
                Ok((_, program)) => {
                    trace!("Parsed {} instructions", program.len());
                    if program.is_empty() {
                        warn!("No instructions found in file: {}\nReturning an empty list", file);
                        return Ok(Vec::new());
                    }

                    for instruction in program.iter() {
                        if instruction.opcode != Opcode::_Label && instruction.opcode != Opcode::_Definition {
                            return Ok(program);
                        }
                    }
                    Err(ParsingError::NoInstructions(file.to_string()))
                },
                Err(e) => {
                    Err(ParsingError::FailedToParse { 
                        file: file.to_string(), 
                        reason: convert_error(input.as_str(), e) })
                }
            }
        },
        Err(e) => {
            error!("Failed to read file: {}", file);
            Err(ParsingError::from(e))
        }
    }
}

#[cfg(test)]
mod test {
    use nom::error::convert_error;
    use nom::Finish;
    use crate::parsing::basic::*;
    use crate::parsing::complex::*;
    use crate::symbols::opcodes::Opcode;
    use crate::symbols::operands::address::Address;
    use crate::symbols::operands::condition::Condition;
    use crate::symbols::operands::immediate::Immediate;
    use crate::symbols::operands::label::Label;
    use crate::symbols::operands::Operand;
    use crate::symbols::operands::port::Port;
    use crate::symbols::operands::register::Register;

    #[test]
    fn labels_default() {
        let input = ".main_loop_start";
        let result = parse_labels(input).finish();
        assert!(result.is_ok(), "Failed to parse valid label: \n{}", convert_error(input, result.unwrap_err()));
        let (rest, label) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(label.operands[0], Operand::Name("main_loop_start".to_string()));
    }

    #[test]
    fn labels_newline() {
        let input = ".main_loop_start\r\n";
        let result = parse_labels(input);
        assert!(result.is_ok(), "Failed to parse valid label");
        let (_, label) = result.unwrap();
        assert_eq!(label.operands[0], Operand::Name("main_loop_start".to_string()));
    }

    #[test]
    fn labels_comment() {
        let input = ".main_loop_start // comment\r\n";
        let result = parse_labels(input);
        assert!(result.is_ok(), "Failed to parse valid label");
        let (_, label) = result.unwrap();
        assert_eq!(label.operands[0], Operand::Name("main_loop_start".to_string()));
    }

    #[test]
    fn definition() {
        let input = "define MY_CONST 10\r\n";
        let result = parse_definitions(input);
        assert!(result.is_ok(), "Failed to parse valid definition");
    }

    #[test]
    fn definition_missing_value() {
        let input = "define MY_CONST \r\n";
        let result = parse_definitions(input);
        assert!(result.is_err(), "Failed to error on invalid definition");
    }

    #[test]
    fn definition_invalid_name() {
        let input = "define 1 2 \r\n";
        let result = parse_definitions(input);
        assert!(result.is_err(), "Failed to error on invalid definition");
    }

    #[test]
    fn reserved_words() {
        let input = "define r11 22 // also comments\r\n";
        let result = parse_definitions(input);
        assert!(result.is_err(), "Failed to error on invalid definition");
    }

    #[test]
    fn end_of_file() {
        let input = "    ADD R1, R2, R3";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse valid instruction: \n{}", convert_error(input, result.unwrap_err()));
    }

    #[test]
    fn newline() {
        let input = "ADD R1 R2 R3\r\n";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse valid instruction: \n{}", convert_error(input, result.unwrap_err()));
    }

    #[test]
    fn semicolon() {
        let input = "ADD R1, R2, R3;\r\n";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse valid instruction: \n{}", convert_error(input, result.unwrap_err()));
    }
    
    #[test]
    fn no_space() {
        let input = "ADD R1,R2,R3\r\n";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse valid instruction: \n{}", convert_error(input, result.unwrap_err()));
    }
    
    #[test]
    fn weird_comment_1() {
        
        let input = "ADD/* \r\nThis should fail\r\n\r\n */R1, R2, R3 // comment\r\n";
        let result = parse_instruction(input).finish();
        assert!(result.is_err(), "Failed to error on invalid instruction");
        
        let err = result.unwrap_err();
        println!("\n{}", convert_error(input, err));
    }

    #[test]
    fn weird_comment_2() {
        let input = "NOP/* \r\nThis should pass\r\n\r\n */";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse valid instruction: \n{}", convert_error(input, result.unwrap_err()));
    }

    #[test]
    fn weird_comment_3() {
        let input = "ADD /* \r\nThis should fail\r\n\r\n */R1, R2, R3 // comment\r\n";
        let result = parse_instruction(input).finish();
        assert!(result.is_err(), "Failed to error on invalid instruction");

        let err = result.unwrap_err();
        println!("\n{}", convert_error(input, err));
    }

    #[test]
    fn weird_comment_4() {
        let input = "ADD/* \r\nThis should fail\r\n\r\n */ R1, R2, R3 // comment\r\n";
        let result = parse_instruction(input).finish();
        assert!(result.is_err(), "Failed to error on invalid instruction");

        let err = result.unwrap_err();
        println!("\n{}", convert_error(input, err));
    }

    #[test]
    fn invalid_opcode() {
        let input = "jabsr r1, r2, r3\r\n";
        let result = parse_instruction(input);
        assert!(result.is_err(), "Failed to error on invalid opcode");
    }
    
    #[test]
    fn operands_out_of_order() {
        let input = "BRH 0x3333 CS\r\n";
        let result = parse_instruction(input);
        assert!(result.is_err(), "Failed to error on invalid operands");
    }

    #[test]
    fn invalid_operands() {
        let input = "jmp \"Hello World\"\r\n";
        let result = parse_instruction(input);
        assert!(result.is_err(), "Failed to error on invalid operands");
    }
    
    #[test]
    fn weird_load_store_1() {
        let input = "    LOD R1, R2, 3";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse on valid operands: \n{}", convert_error(input, result.unwrap_err()));
    }

    #[test]
    fn weird_load_store_2() {
        let input = "    LOD R1, R3";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse on valid operands: \n{}", convert_error(input, result.unwrap_err()));
    }

    #[test]
    fn weird_load_store_3() {
        let input = "    LOD R1 R3 3";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse on valid operands: \n{}", convert_error(input, result.unwrap_err()));
    }

    #[test]
    fn weird_load_store_4() {
        let input = "    LOD R1 R3 // comment\r\n";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse on valid operands: \n{}", convert_error(input, result.unwrap_err()));
    }

    #[test]
    fn weird_load_store_5() {
        let input = "    LOD R1 R3;\r\n";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse on valid operands: \n{}", convert_error(input, result.unwrap_err()));
    }

    #[test]
    fn weird_load_store_6() {
        let input = "    LOD R1, R3,\r\n";
        let result = parse_instruction(input).finish();
        assert!(result.is_err(), "Failed to error on invalid operands");
    }
    
    #[test]
    fn weird_jump_case() {
        let input = "    jmp .start\r\n.start\r\n    nop";
        let result = parse_instruction(input).finish();
        assert!(result.is_ok(), "Failed to parse on valid operands: \n{}", convert_error(input, result.unwrap_err()));
        
        let (rest, instruction) = result.unwrap();
        println!("{:?}", instruction);
        assert_eq!(rest, ".start\r\n    nop");
        assert_eq!(instruction.opcode, Opcode::JMP);
        let offset = Label::new("start".to_string());
        assert_eq!(instruction.operands[0], Operand::Label(offset));
    }

    #[test]
    fn too_few_operands() {
        let input = "sub R1, R2";
        let result = parse_instruction(input);
        assert!(result.is_err(), "Failed to error on too few operands");
    }

    #[test]
    fn test_parse_character() {
        let input = "'a'";
        let result = character(input).finish();
        assert!(result.is_ok(), "Failed to parse valid character: \n{}", convert_error(input, result.unwrap_err()));
    }

    #[test]
    fn port_invalid() {
        let input = "clear_screen";
        let result = port(input);
        assert!(result.is_err(), "Failed to error on invalid port");
    }
    
    #[test]
    fn port_all_caps() {
        let input = "CLEAR_SCREEN_BUFFER";
        let result = port(input).finish();
        assert!(result.is_ok(), "Failed to parse valid port: \n{}", convert_error(input, result.unwrap_err()));

        let (_, port) = result.unwrap();
        assert_eq!(port, Port::ClearScreenBuffer);
    }
    
    #[test]
    fn port_lowercase() {
        let input = "clear_screen_buffer";
        let result = port(input).finish();
        assert!(result.is_ok(), "Failed to parse valid port: \n{}", convert_error(input, result.unwrap_err()));

        let (_, port) = result.unwrap();
        assert_eq!(port, Port::ClearScreenBuffer);
    }

    #[test]
    fn condition_test() {
        let input = "EQ,";
        let result = condition(input).finish();
        assert!(result.is_ok(), "Failed to parse valid condition: \n{}", convert_error(input, result.unwrap_err()));

        let (_, cond) = result.unwrap();
        assert_eq!(cond, Condition::Z1);
    }
    
    #[test]
    fn condition_test_invalid() {
        let input = "gt";
        let result = condition(input);
        assert!(result.is_err(), "Failed to error on invalid condition");
    }
    
    #[test]
    fn  condition_test_symbol() {
        let input = "!= .test_label";
        let result = condition(input).finish();
        assert!(result.is_ok(), "Failed to parse valid condition: \n{}", convert_error(input, result.unwrap_err()));

        let (_, cond) = result.unwrap();
        assert_eq!(cond, Condition::Z0);
    }

    #[test]
    fn offset_test() {
        let input = ".offset \r\n";
        let result = label_usage(input).finish();
        assert!(result.is_ok(), "Failed to parse valid offset: \n{}", convert_error(input, result.unwrap_err()));
    }
    
    #[test]
    fn offset_invalid() {
        let input = ".";
        let result = label_usage(input);
        assert!(result.is_err(), "Failed to error on invalid offset");
    }
    
    #[test]
    fn offset_error_spacing() {
        let input = ".   offset\r\n    nop";
        let result = label_usage(input).finish();
        assert!(result.is_err(), "Failed to error on invalid offset");
        
        let err = result.unwrap_err();
        println!("{}", convert_error(input, err));
    }

    #[test]
    fn test_parse_address() {
        let input = "0x0020\r\n";
        let result = address(input).finish();
        assert!(result.is_ok(), "Failed to parse valid address: \n{}", convert_error(input, result.unwrap_err()));

        let (_, addr) = result.unwrap();
        assert_eq!(addr, Address::new(0x0020).unwrap());
    }
    
    #[test]
    fn test_immediate() {
        let input = "42\r\n";
        let result = immediate(input).finish();
        assert!(result.is_ok(), "Failed to parse valid immediate value: \n{}", convert_error(input, result.unwrap_err()));

        let (_, imm) = result.unwrap();
        assert_eq!(imm, Immediate::new(42).unwrap());
    }
    
    #[test]
    fn test_immediate_negative() {
        let input = "-69; test with negative";
        let result = immediate(input).finish();
        assert!(result.is_ok(), "Failed to parse valid immediate value: \n{}", convert_error(input, result.unwrap_err()));

        let (_, imm) = result.unwrap();
        assert_eq!(imm, Immediate::new(-69).unwrap());
    }

    #[test]
    fn test_immediate_binary() {
        let input = "0b101010 // binary";
        let result = immediate(input).finish();
        assert!(result.is_ok(), "Failed to parse valid immediate value: \n{}", convert_error(input, result.unwrap_err()));

        let (_, imm) = result.unwrap();
        assert_eq!(imm, Immediate::new(42).unwrap());
    }

    #[test]
    fn test_immediate_invalid() {
        let input = "1000 \r\n";
        let result = immediate(input);
        assert!(result.is_err(), "Failed to error on invalid immediate value");
    }

    #[test]
    fn test_register_comma() {
        let input = "R1,";
        let result = register(input).finish();
        assert!(result.is_ok(), "Failed to parse valid register: \n{}", convert_error(input, result.unwrap_err()));

        let (_, reg) = result.unwrap();
        assert_eq!(reg, Register::R1);
    }
    
    #[test]
    fn test_register_semicolon() {
        let input = "r0; test :3";
        let result = register(input).finish();
        assert!(result.is_ok(), "Failed to parse valid register: \n{}", convert_error(input, result.unwrap_err()));

        let (_, reg) = result.unwrap();
        assert_eq!(reg, Register::R0);
    }
    
    #[test]
    fn test_register_newline() {
        let input = "r2\r\n";
        let result = register(input).finish();
        assert!(result.is_ok(), "Failed to parse valid register: \n{}", convert_error(input, result.unwrap_err()));

        let (_, reg) = result.unwrap();
        assert_eq!(reg, Register::R2);
    }
    
    #[test]
    fn test_register_space() {
        let input = "R3 ";
        let result = register(input).finish();
        assert!(result.is_ok(), "Failed to parse valid register: \n{}", convert_error(input, result.unwrap_err()));

        let (_, reg) = result.unwrap();
        assert_eq!(reg, Register::R3);
    }
    
    #[test]
    fn test_register_invalid() {
        let input = "r16,";
        let result = register(input);
        assert!(result.is_err(), "Failed to error on invalid register");
    }

    #[test]
    fn test_opcode_uppercase() {
        let input = "ADD";
        let result = opcode(input).finish();
        assert!(result.is_ok(), "Failed to parse valid opcode: \n{}", convert_error(input, result.unwrap_err()));

        let (_, op) = result.unwrap();
        assert_eq!(op, Opcode::ADD);
    }
    
    #[test]
    fn test_opcode_lowercase() {
        let input = "sub ";
        let result = opcode(input).finish();
        assert!(result.is_ok(), "Failed to parse valid opcode: \n{}", convert_error(input, result.unwrap_err()));

        let (_, op) = result.unwrap();
        assert_eq!(op, Opcode::SUB);
    }
    
    #[test]
    fn test_opcode_semicolon() {
        let input = "hlt;\r\n";
        let result = opcode(input).finish();
        assert!(result.is_ok(), "Failed to parse valid opcode: \n{}", convert_error(input, result.unwrap_err()));

        let (_, op) = result.unwrap();
        assert_eq!(op, Opcode::HLT);
    }
    
    #[test]
    fn test_opcode_invalid() {
        let input = "jabsr";
        let result = opcode(input);
        assert!(result.is_err(), "Failed to error on invalid opcode");
    }

    #[test]
    fn test_parse_label() {
        let input = ".my_label\r\n";
        let result = label_define(input);
        assert!(result.is_ok(), "Failed to parse valid label");
    }
}