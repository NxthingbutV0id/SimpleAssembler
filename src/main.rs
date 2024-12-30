mod cli;
mod printer;
mod layout;
mod encoder;
mod symbols;
mod resolver;
mod evaluator;
mod parsing;
mod tests;

use clap::Parser;
use std::fs;
use std::path::Path;
use anyhow::Error;
use crate::cli::CLI;
use crate::encoder::InstructionEncoder;
use crate::printer::AssemblyPrinter;
use crate::symbols::instruction::Instruction;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

// NOTE: This whole thing is based off of mattbatwing's minecraft CPU
fn main() -> anyhow::Result<()> {
    let args = CLI::parse();

    if args.debug {
        std::env::set_var("RUST_LOG", "trace");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    let mut program: Vec<Instruction> = Vec::new();

    for file in args.input_files {
        // there could be multiple .asm files, but we compile to one binary
        let code = parsing::parse_file(&file);
        match code {
            Ok(code) => {
                program.extend(code);
            },
            Err(e) => {
                error!("Failed to parse file: {}", file);
                return Err(Error::from(e));
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
    evaluator::evaluate_program(&mut program);

    let mut encoder = InstructionEncoder::new();
    // Mutates the Instructions in the Vec<Instruction>
    let test = encoder.encode_program(&mut program);
    if test.is_err() {
        error!("Failed to encode program");
        return Err(Error::from(test.err().unwrap()));
    }

    if args.print_assembly {
        // Reads from the Vec<Instruction>
        let mut printer = AssemblyPrinter::new(&program);
        info!("Printing assembly: \n{}", printer.print());
    }

    // Takes the Vec<Instruction> since it's not needed after this point
    let binary: Vec<u8> = convert_program_to_bytes(program, args.size);

    if args.output.is_some() {
        info!("Writing output file...");
        let output = args.output.clone().unwrap();
        let test = fs::write(&output, &binary);
        if test.is_err() {
            error!("Failed to write output file");
            return Err(Error::from(test.err().unwrap()));
        }
        info!("Output file written to {}", Path::new(&output).display());
    }

    if args.print_binary || args.output.is_none() {
        info!("Printing hex dump...");
        for (i, byte) in binary.iter().enumerate() {
            if i % 16 == 0 {
                print!("\n{:04X} | ", i);
            }
            print!("{:02X} ", byte);
        }
        print!("<end of file>\n\n");
        trace!("Finished printing binary");
    }

    info!("End of program");
    Ok(())
}

fn convert_program_to_bytes(program: Vec<Instruction>, size: Option<u16>) -> Vec<u8> {
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
            panic!("Failed to write binary");
        }
        while binary.len() < size.unwrap() as usize {
            binary.push(0);
        }
    }

    trace!("Binary created");
    binary
}