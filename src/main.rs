/*
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unused_must_use)]
*/
mod cli;
mod printer;
mod layout;
mod encoder;
mod symbols;
mod resolver;
mod evaluator;
mod parsing;
mod assembler;

use std::path::PathBuf;
use clap::Parser;
use crate::assembler::AssemblerStatus;
use crate::cli::CLI;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

// NOTE: This whole thing is based off of mattbatwing's minecraft CPU
fn main() -> AssemblerStatus {
    let args = CLI::parse();

    std::env::set_var("RUST_LOG", {
        match (args.verbose, args.debug) { 
            (true, true) => "trace",
            (false, true) => "debug",
            (true, false) => "info",
            (false, false) => "warn"
        }
    });

    pretty_env_logger::init();
    
    let output_path = args.output.unwrap_or_else(|| PathBuf::from("a.bin"));

    let status = assembler::assemble(&args.input_files, output_path, args.size);
    let program = if status.is_err() {
        error!("Failed to assemble program: {}", status.err().unwrap());
        return AssemblerStatus::Failure;
    } else {
        status.unwrap()
    };

    if args.print {
        assembler::print_assembly(&program);
    }

    if args.hex_dump {
        let test = assembler::hex_dump(&program);
        if test.is_err() {
            error!("Failed to print hex dump: {}", test.err().unwrap());
        }
    }

    AssemblerStatus::Success
}