use std::{fs, io};
use std::path::Path;
use std::process::{ExitCode, Termination};
use anyhow::Error;
use crate::{evaluator, layout, parsing, resolver};
use crate::encoder::{InstructionEncoder};
use crate::printer::AssemblyPrinter;
use crate::symbols::instruction::Instruction;

pub enum AssemblerStatus {
    Success,
    Failure
}

impl Termination for AssemblerStatus {
    fn report(self) -> ExitCode {
        match self {
            AssemblerStatus::Success => ExitCode::SUCCESS,
            AssemblerStatus::Failure => ExitCode::FAILURE
        }
    }
}

pub struct Assembler {
    program: Vec<Instruction>,
    input_files: Vec<String>
}

impl Assembler {
    pub fn new(input_files: Vec<String>) -> Assembler {
        Assembler {
            program: Vec::new(),
            input_files
        }
    }

    pub fn assemble(&mut self) -> anyhow::Result<()> {
        for file in &self.input_files {
            // there could be multiple .asm files, but we compile to one binary
            let code = parsing::parse_file(&file);
            match code {
                Ok(code) => {
                    self.program.extend(code);
                },
                Err(e) => {
                    error!("Failed to parse file: {}", file);
                    return Err(e)
                }
            }
        }

        layout::layout_program(&mut self.program);

        let test = resolver::resolve_program(&mut self.program);
        match test {
            Ok(_) => {
                debug!("Program resolved successfully");
            },
            Err(e) => {
                error!("Failed to resolve program");
                return Err(Error::from(e));
            }
        }

        info!("Evaluating program...");
        let test = evaluator::evaluate_program(&mut self.program);
        match test {
            Ok(_) => {
                debug!("Program evaluated successfully");
            },
            Err(e) => {
                error!("Failed to evaluate program");
                return Err(Error::from(e));
            }
        }

        let mut encoder = InstructionEncoder::new();
        // Mutates the Instructions in the Vec<Instruction>
        let test = encoder.encode_program(&mut self.program);

        match test {
            Ok(_) => {
                debug!("Program encoded successfully");
            },
            Err(e) => {
                error!("Failed to encode program");
                return Err(Error::from(e));
            }
        }

        Ok(())
    }

    pub fn print_assembly(&self) {
        let mut printer = AssemblyPrinter::new(&self.program);
        info!("Printing assembly: \n{}", printer.print());
    }

    pub fn write_binary(&self, size: Option<u16>, output: String) -> anyhow::Result<()> {
        let binary: Vec<u8> = self.convert_program_to_bytes(size)?;

        info!("Writing output file...");
        let test = fs::write(&output, &binary);
        match test {
            Ok(_) => {
                info!("Output file written to {}", Path::new(&output).display());
                Ok(())
            },
            Err(e) => {
                error!("Failed to write output file: {}", output);
                Err(Error::from(e))
            }
        }
    }

    fn convert_program_to_bytes(&self, size: Option<u16>) -> Result<Vec<u8>, io::Error> {
        debug!("Creating binary");
        let mut binary: Vec<u8> = if size.is_some() {
            Vec::with_capacity(size.unwrap() as usize)
        } else {
            Vec::new()
        };

        for instruction in &self.program {
            if let Some(encoding) = instruction.encoding {
                let bytes: [u8; 2] = encoding.to_le_bytes();
                binary.push(bytes[0]);
                binary.push(bytes[1]);
            }
        }

        if size.is_some() {
            if binary.len() > size.unwrap() as usize {
                error!("Binary size ({}) exceeds specified size ({})", binary.len(), size.unwrap());
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Binary size exceeds specified size"));
            }
            while binary.len() < size.unwrap() as usize {
                binary.push(0);
            }
        }

        trace!("Binary created");
        Ok(binary)
    }

    pub fn hex_dump(&self) -> anyhow::Result<()> {
        let bin = self.convert_program_to_bytes(None)?;
        let mut i = 0;
        for byte in bin {
            if i % 16 == 0 {
                println!("\n{:04X} |", i);
            }
            print!("{:02X} ", byte);
            i += 1;
        }
        Ok(())
    }
}