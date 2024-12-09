mod cli;
mod assembly_parser;
mod output;
mod program;
mod parser;

use std::fs;
use std::io::{BufRead, Read};
use cli::CLI;
use clap::Parser;
use crate::assembly_parser::parse_assembly;
use crate::output::write_to_bin;

fn main() {
    let args = CLI::parse();
    println!("Input Files: {:?}", args.input_files);
    println!("Output Dir: {:?}", args.output_dir);
    for file in args.input_files {
        let content = fs::read_to_string(&file).expect("Failed to read file");
        println!("content of {}: {}", file, content);
        let program = parse_assembly(&content).expect("Failed to parse file");
        println!("Program: {:?}", program);
        let bin_path = format!("{}/{}.bin", args.output_dir, file);
        write_to_bin(program, &bin_path).expect("Failed to write to file");
    }
}
