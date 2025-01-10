pub mod architecture;
pub mod parser;
pub mod layout;
pub mod resolve;
pub mod eval;
pub mod encode;
pub mod print;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::{fs, io};
use std::path::PathBuf;
use anyhow::Error;
use crate::architecture::batpu2::instruction::Instruction;
use crate::encode::InstructionEncoder;
use crate::print::AssemblyPrinter;

pub struct Assembler {
    pub size: Option<u16>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            size: None
        }
    }

    pub fn assemble(&self, input_path: &[PathBuf], output_path: PathBuf) -> anyhow::Result<(Vec<Instruction>, Vec<u8>)> {
        let mut program: Vec<Instruction> = Vec::new();
        let input_files: Vec<PathBuf> = input_path
            .iter()
            .filter(|p| p.is_file())
            .map(|p| p.clone())
            .collect();

        for file_path in input_files {
            // there could be multiple .asm files, but we compile to one binary
            match parser::parse(file_path) {
                Ok(code) => program.extend(code),
                Err(e) => return Err(Error::from(e))
            };
        }

        layout::layout_program(&mut program);

        match resolve::resolve_program(&mut program) {
            Ok(_) => {
                debug!("Program resolved successfully");
            },
            Err(e) => {
                error!("Failed to resolve program");
                return Err(Error::from(e));
            }
        }

        match eval::evaluate_program(&mut program) {
            Ok(_) => {
                debug!("Program evaluated successfully");
            },
            Err(e) => {
                error!("Failed to evaluate program");
                return Err(Error::from(e));
            }
        }

        let mut encoder = InstructionEncoder::new();

        match encoder.encode_program(&mut program) {
            Ok(_) => {
                debug!("Program encoded successfully");
            },
            Err(e) => {
                error!("Failed to encode program");
                return Err(Error::from(e));
            }
        }

        let binary = match self.convert_program_to_bytes(&program) {
            Ok(b) => b,
            Err(e) => return Err(Error::from(e))
        };

        match fs::write(&output_path, &binary) {
            Ok(_) => {
                debug!("Binary written to file: {}", output_path.display());
            },
            Err(e) => {
                error!("Failed to write binary to file: {}", e);
                return Err(Error::from(e));
            }
        }

        Ok((program, binary))
    }

    fn convert_program_to_bytes(&self, program: &[Instruction]) -> Result<Vec<u8>, io::Error> {
        debug!("Creating binary");
        let mut binary: Vec<u8> = if self.size.is_some() {
            Vec::with_capacity(self.size.unwrap() as usize)
        } else {
            Vec::new()
        };

        for instruction in program {
            if let Some(encoding) = instruction.encoding {
                let bytes: [u8; 2] = encoding.to_le_bytes();
                binary.push(bytes[0]);
                binary.push(bytes[1]);
            }
        }

        if self.size.is_some() {
            if binary.len() > self.size.unwrap() as usize {
                error!("Binary size ({}) exceeds specified size ({})", binary.len(), self.size.unwrap());
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Binary size exceeds specified size"));
            }
            while binary.len() < self.size.unwrap() as usize {
                binary.push(0);
            }
        }

        trace!("Binary created");
        Ok(binary)
    }

    pub fn print_program(&self, program: &[Instruction]) {
        let mut printer = AssemblyPrinter::new(&program);
        info!("Printing assembly: \n{}", printer.print());
    }

    pub fn hex_dump(&self, bin: &[u8]) {
        let mut i = 0;
        for byte in bin {
            if i % 16 == 0 {
                println!("\n{:04X} |", i);
            }
            print!("{:02X} ", byte);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests { // TODO: Finish writing all the tests... again
    use super::*;
    
    #[test]
    fn ai_generated() {
        let input_path = vec![PathBuf::from("./test_data/ai_generated/test.asm")];
        let output_path = PathBuf::from("./test_data/ai_generated/test.asm");
        let assembler = Assembler::new();

        let test = assembler.assemble(&input_path, output_path);
        assert!(test.is_ok(), "Failed to assemble program: {}", test.err().unwrap());
    }

    #[test]
    fn different_files() {
        let input_path = vec![PathBuf::from("./test_data/different_files/alpha.asm")];
        let output_path = PathBuf::from("./test_data/different_files/alpha.bin");
        let assembler = Assembler::new();

        let test = assembler.assemble(&input_path, output_path);
        assert!(test.is_ok(), "Failed to assemble program: {}", test.err().unwrap());

        let (_, a) = test.unwrap();

        let input_path = vec![PathBuf::from("./test_data/different_files/beta.asm")];
        let output_path = PathBuf::from("./test_data/different_files/beta.bin");
        let test = assembler.assemble(&input_path, output_path);
        assert!(test.is_ok(), "Failed to assemble program: {}", test.err().unwrap());
        
        let (_, b) = test.unwrap(); 
        
        assert_eq!(a, b, "Binary files are not equal");
    }
}
