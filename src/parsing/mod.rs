use std::{fs, io};
use crate::symbols::instruction::Instruction;
use crate::parsing::parsers::{parse_labels, parse_definitions, parse_instruction};
use crate::parsing::helper::skip;

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

pub struct AssemblyParser {
    current_input: Option<String>,
    current_file: Option<String>,
    current_content: Option<String>,
    pub program: Vec<Instruction>
}

impl AssemblyParser {
    pub fn new() -> AssemblyParser {
        AssemblyParser {
            current_input: None,
            current_file: None,
            current_content: None,
            program: Vec::new()
        }
    }

    pub fn parse_file(&mut self, file: &str) -> Result<(), io::Error> {
        info!("Parsing {}...", file);
        self.current_file = Some(file.to_string());
        self.current_input = Some(fs::read_to_string(file)?);
        self.current_content = Some(self.current_input.clone().unwrap());
        self.parse_program();
        self.current_file = None;
        self.current_input = None;
        self.current_content = None;
        trace!("Finished parsing file: {}", file);
        Ok(())
    }

    fn parse_program(&mut self) {
        let binding = self.current_input.clone().unwrap();
        let mut input: &str = binding.as_str();
        let mut line = 0;

        (input, _) = skip(input).unwrap();
        while !input.is_empty() {
            let (rest, instruction) = parse_labels(input)
                .or_else(|_| parse_definitions(input))
                .or_else(|_| parse_instruction(input))
                .expect("Failed to parse instruction");
            trace!("Parsed instruction: {:5}: {}", line, instruction);
            self.program.push(instruction);
            input = rest;
            (input, _) = skip(input).unwrap();
            line += 1;
        }
    }
}