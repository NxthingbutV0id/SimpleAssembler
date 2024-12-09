use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Instruction {
    pub operation: String,
    pub arguments: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}