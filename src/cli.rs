use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "Assembler", version, author, about, long_about = None)]
pub struct CLI {
    /// Input files to assemble
    #[clap(short = 'i', long = "input", required = true)]
    pub input_files: Vec<String>,
    /// Output file to contain the machine code
    #[clap(short = 'o', long = "output")]
    pub output: Option<String>,
    /// Size of the output binary
    #[clap(short = 's', long = "size")]
    pub size: Option<u16>,
    /// Prints final Assembly code
    #[clap(short = 'v', long = "print_assembly")]
    pub print_assembly: bool,
    /// Prints final machine code
    #[clap(short = 'x', long = "print_binary")]
    pub print_binary: bool,
    /// Enables debug mode
    #[clap(short = 'd', long = "debug")]
    pub debug: bool,
}