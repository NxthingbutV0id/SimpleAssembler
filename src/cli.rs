use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "Assembler", version, author, about, long_about = None)]
pub struct CLI {
    /// Input files to assemble
    #[clap(short = 'i', long = "input", required = true)]
    pub input_files: Vec<PathBuf>,
    /// Output file to contain the machine code
    #[clap(short = 'o', long = "output")]
    pub output: Option<PathBuf>,
    /// Size of the output binary
    #[clap(short = 's', long = "size")]
    pub size: Option<u16>,
    /// Prints final Assembly code
    #[clap(short = 'p', long = "print")]
    pub print: bool,
    /// Prints final machine code
    #[clap(short = 'x', long = "hex_dump")]
    pub hex_dump: bool,
    /// Enables debug mode
    #[clap(short = 'd', long = "debug")]
    pub debug: bool,
    /// Verbose mode
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,
}