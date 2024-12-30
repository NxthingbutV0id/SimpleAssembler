use std::fs;
use std::path::Path;
use anyhow::Error;
use custom_error::custom_error;
use crate::symbols::instruction::Instruction;
use crate::parsing::parsers::parse_program;
use crate::parsing::ParsingError::*;
use crate::symbols::opcodes::Opcode;

pub(crate) mod parsers;
pub(crate) mod helper;

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

custom_error! {pub ParsingError
    FileNotFound = "File not found",
    InvalidFile = "Invalid file",
    InvalidExtension = "Invalid file extension",
    NoInstructions = "No instructions found in file"
}

pub fn parse_file(file: &str) -> anyhow::Result<Vec<Instruction>> {
    info!("Parsing {}...", file);
    let path = Path::new(file);
    if !path.exists() {
        error!("File not found: {}", file);
        return Err(Error::from(FileNotFound));
    }

    if !path.is_file() {
        error!("Not a file: {}", file);
        return Err(Error::from(InvalidFile));
    }

    if path.extension().unwrap() != "asm" &&
        path.extension().unwrap() != "as" &&
        path.extension().unwrap() != "s" {
        error!("Not an assembly file (.asm/.as/.s): {:?}", path.extension().unwrap());
        return Err(Error::from(InvalidExtension));
    }


    let input = fs::read_to_string(path);
    match input {
        Ok(input) => {
            if input.is_empty() {
                warn!("File is empty: {}", file);
                return Ok(Vec::new());
            }

            let program = parse_program(&input)
                .expect("Failed to parse program").1;

            trace!("Finished parsing file: {}", file);
            if program.is_empty() {
                warn!("No instructions found in file: {}", file);
                return Ok(Vec::new());
            }

            for instruction in program.iter() {
                if instruction.opcode != Opcode::_Label && instruction.opcode != Opcode::_Definition {
                    return Ok(program);
                }
            }
            Err(Error::from(NoInstructions))
        },
        Err(e) => {
            error!("Failed to read file: {}", file);
            Err(Error::from(e))
        }
    }
}