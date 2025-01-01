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

use clap::Parser;
use crate::assembler::{Assembler, AssemblerStatus};
use crate::cli::CLI;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

// NOTE: This whole thing is based off of mattbatwing's minecraft CPU
fn main() -> AssemblerStatus {
    let args = CLI::parse();

    if args.debug {
        std::env::set_var("RUST_LOG", "trace");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    let mut assembler = Assembler::new(args.input_files);

    let status = assembler.assemble();
    if status.is_err() {
        error!("Failed to assemble program: {}", status.err().unwrap());
        return AssemblerStatus::Failure;
    }

    if args.print_assembly {
        assembler.print_assembly();
    }

    if args.output.is_some() {
        let test = assembler.write_binary(args.size, args.output.clone().unwrap());
        if test.is_err() {
            error!("Failed to write binary: {}", test.err().unwrap());
            return AssemblerStatus::Failure;
        }
    }

    if args.print_binary || args.output.is_none() {
        let test = assembler.hex_dump();
        if test.is_err() {
            error!("Failed to print hex dump: {}", test.err().unwrap());
            return AssemblerStatus::Failure;
        }
    }

    AssemblerStatus::Success
}