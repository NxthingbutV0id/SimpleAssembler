use std::{
    fs::File,
    io::Write
};
use crate::assembly_parser::encode_instruction;
use crate::program::Program;

pub fn write_to_bin(program: Program, output_path: &str) -> anyhow::Result<()> {
    let mut file = File::create(output_path)?;
    for instr in &program.instructions {
        let encoded = encode_instruction(instr);
        file.write_all(&encoded.to_le_bytes())?;
    }
    Ok(())
}