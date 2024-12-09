use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CLI {
    /// The .asm files to be compiled
    pub input_files: Vec<String>,
    /// Output Directory for the completed binaries
    #[arg(short, long, default_value="./output")]
    pub output_dir: String,
}