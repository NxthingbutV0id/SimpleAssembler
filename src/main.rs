use std::path::PathBuf;
use simple_assembler::Assembler;

fn main() {
    let input_path = PathBuf::from("./examples/tetris.asm");
    let output_path = PathBuf::from("./examples/tetris.bin");
    let assembler = Assembler::new();

    match assembler.assemble(&[input_path], output_path) {
        Ok((prog, bin)) => {
            println!("Program assembled successfully");

            assembler.print_program(&prog);
            assembler.hex_dump(&bin);
        },
        Err(e) => {
            eprintln!("Failed to assemble program: {}", e);
        }
    }
}

/*
use std::io::{self, Write};
use std::fmt::Display;
fn input(message: &'_ impl Display) -> Result<String, io::Error> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
 */