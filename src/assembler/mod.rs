pub mod error;

use std::{fs, io};
use std::path::PathBuf;
use std::process::{ExitCode, Termination};
use anyhow::Error;
use crate::{evaluator, layout, parsing, resolver};
use crate::encoder::InstructionEncoder;
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

pub fn assemble(input_path: &[PathBuf], output_path: PathBuf, size: Option<u16>) -> anyhow::Result<Vec<Instruction>> {
    let mut program: Vec<Instruction> = Vec::new();
    let input_files: Vec<PathBuf> = input_path
        .iter()
        .filter(|p| p.is_file())
        .map(|p| p.clone())
        .collect();
    
    for file_path in input_files {
        // there could be multiple .asm files, but we compile to one binary
        let code = parsing::parse_file(file_path);
        match code {
            Ok(code) => {
                program.extend(code);
            },
            Err(e) => {
                return Err(Error::from(e))
            }
        }
    }

    layout::layout_program(&mut program);

    let test = resolver::resolve_program(&mut program);
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
    let test = evaluator::evaluate_program(&mut program);
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
    let test = encoder.encode_program(&mut program);

    match test {
        Ok(_) => {
            debug!("Program encoded successfully");
        },
        Err(e) => {
            error!("Failed to encode program");
            return Err(Error::from(e));
        }
    }

    write_binary(&program, size, output_path)?;
    Ok(program)
}

pub fn print_assembly(program: &[Instruction]) {
    let mut printer = AssemblyPrinter::new(&program);
    info!("Printing assembly: \n{}", printer.print());
}

fn write_binary(program: &[Instruction], size: Option<u16>, output: PathBuf) -> anyhow::Result<()> {
    let binary: Vec<u8> = convert_program_to_bytes(&program, size)?;

    info!("Writing output file...");
    let test = fs::write(&output, &binary);
    match test {
        Ok(_) => {
            info!("Output file written to {}", output.display());
            Ok(())
        },
        Err(e) => {
            error!("Failed to write output file: {}", output.display());
            Err(Error::from(e))
        }
    }
}

fn convert_program_to_bytes(program: &[Instruction], size: Option<u16>) -> Result<Vec<u8>, io::Error> {
    debug!("Creating binary");
    let mut binary: Vec<u8> = if size.is_some() {
        Vec::with_capacity(size.unwrap() as usize)
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

pub fn hex_dump(program: &[Instruction]) -> anyhow::Result<()> {
    let bin = convert_program_to_bytes(program, None)?;
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