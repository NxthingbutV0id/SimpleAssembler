use thiserror::Error;
use crate::symbols::operands::label::Label;
use crate::symbols::operands::Operand;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Invalid Extension: {0}")]
    InvalidExtension(String),
    #[error("No instructions found in file: {0}")]
    NoInstructions(String),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Failed to parse {file}\n{reason}")]
    FailedToParse{
        file: String,
        reason: String
    },
}

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("Value Out of Bound\n{0} does not fit within the encoding")]
    ValueOutOfBounds(u16),
    #[error("Value Overflow\nThe value of {value} is too large to fit in {length} bits")]
    ValueOverflow {
        value: u16,
        length: u16
    },
    #[error("invalid operand: Expected {expected:?}, found {found:?}")]
    InvalidOperand {
        expected: String,
        found: Operand
    },
    #[error("unbound definition: Definition {0} does not have a value")]
    UnboundDefinition(String),
    #[error("unbound offset: Offset {0} does not have a binding")]
    UnboundOffset(Label),
    #[error("invalid offset: Offset {0} does not have an address")]
    InvalidOffset(Label)
}

#[derive(Error, Debug)]
pub enum EvaluatorError {
    #[error("unknown definition: Definition {0} is not defined")]
    UnknownDefinition(String),
    #[error("missing definition value: Definition {0} does not have a value")]
    MissingDefinitionValue(String),
    #[error("unknown offset: Offset {0} was not found")]
    UnknownOffset(String),
    #[error("invalid offset: Offset {0} does not have an address")]
    InvalidOffset(String),
}

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("unknown label: Label {0} was not found")]
    UnknownLabel(String),
    #[error("invalid label: Label {0} is already defined")]
    InvalidLabel(String),
    #[error("missing address: Label {0} does not have an address")]
    MissingAddress(String)
}

