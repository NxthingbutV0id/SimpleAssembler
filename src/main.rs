mod cli;
mod printer;
mod layout;
mod encoder;
mod symbols;
mod resolver;
mod evaluator;
mod parsing;

use clap::Parser;
use std::{fs, io};
use std::path::Path;
use nom::bytes::complete::take;
use crate::cli::CLI;
use crate::encoder::InstructionEncoder;
use crate::layout::Layout;
use parsing::AssemblyParser;
use parsing::helper::ws;
use crate::printer::AssemblyPrinter;
use crate::resolver::Resolver;
use crate::symbols::instruction::Instruction;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

// NOTE: This whole thing is based off of mattbatwing's minecraft CPU
// This took so fucking long to get working, I'm so happy it's finally done
fn main() -> io::Result<()> {
    let args = CLI::parse();

    if args.debug {
        std::env::set_var("RUST_LOG", "trace");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    let mut parser = AssemblyParser::new();
    for file in args.input_files {
        parser.parse_file(&file)?;
    }

    let mut layout = Layout::new();
    layout.layout_program(&mut parser.program);

    let mut resolver = Resolver::new();
    resolver.resolve_program(&mut parser.program);

    evaluator::evaluate_program(&mut parser.program);

    let mut encoder = InstructionEncoder::new();
    encoder.encode_program(&mut parser.program);
    if args.print_assembly {
        let mut printer = AssemblyPrinter::new(&parser.program);
        info!("Printing assembly: \n{}", printer.print());
    }

    let binary: Vec<u8> = convert_program_to_bytes(&parser.program, args.size);

    if args.output.is_some() {
        info!("Writing output file...");
        let output = args.output.clone().unwrap();
        fs::write(&output, &binary)?;
        let expected = convert_mc_to_bin("test_data/tetris.mc");
        fs::write("test_data/tetris_expected.bin", &expected)?;
        info!("Output file written to {}", Path::new(&output).display());
    }

    if args.print_binary || args.output.is_none() {
        info!("Printing binary...");
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

fn convert_program_to_bytes(program: &Vec<Instruction>, size: Option<u16>) -> Vec<u8> {
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

fn convert_mc_to_bin(file: &str) -> Vec<u8> {
    let binding = fs::read_to_string(file)
        .expect("Failed to read input file");
    let mut file: &str = binding.as_str();
    let mut binary: Vec<u8> = Vec::new();

    while !file.is_empty() {
        let (rest, mc) = ws(take::<usize, &str, ()>(16usize))(file)
            .expect("Failed to take 16 bits");
        let bytes: u16 = u16::from_str_radix(mc, 2)
            .expect("Failed to convert machine code to byte");

        let lo: u8 = (bytes & 0xFF) as u8;
        let hi: u8 = (bytes >> 8) as u8;
        binary.push(lo);
        binary.push(hi);
        file = rest;
    }

    binary
}