use crate::parsing::parsers::*;
use crate::symbols::operands::Operand;

#[test]
fn test_parse_labels() {
    let input = ".main_loop_start";
    let result = parse_labels(input);
    assert!(result.is_ok(), "Failed to parse valid label");
    let (rest, label) = result.unwrap();
    assert_eq!(rest, "");
    assert_eq!(label.operands[0], Operand::Label("main_loop_start".to_string()));
}

#[test]
fn test_parse_definitions() {
    let input = "define MY_CONST 10\r\n";
    let result = parse_definitions(input);
    assert!(result.is_ok(), "Failed to parse valid definition");
}

#[test]
fn test_parse_instruction() {
    let input = "ADD R1, R2, R3";
    let result = parse_instruction(input);
    assert!(result.is_ok(), "Failed to parse valid instruction");
}

#[test]
fn test_parse_invalid_instruction() {
    let input = "INVALID R1, R2, R3";
    let result = parse_instruction(input);
    assert!(result.is_err(), "Parsed invalid instruction successfully");
}

#[test]
fn test_parse_character() {
    let input = "'a'";
    let result = character(input);
    assert!(result.is_ok(), "Failed to parse valid character");
}

#[test]
fn test_parse_port() {
    let input = "PORT1";
    let result = port(input);
    assert!(result.is_ok(), "Failed to parse valid port");
}

#[test]
fn test_parse_condition() {
    let input = "EQ,";
    let result = condition(input);
    assert!(result.is_ok(), "Failed to parse valid condition");
}

#[test]
fn test_parse_offset() {
    let input = ".offset";
    let result = offset(input);
    assert!(result.is_ok(), "Failed to parse valid offset");
}

#[test]
fn test_parse_address() {
    let input = "0x1234";
    let result = address(input);
    assert!(result.is_ok(), "Failed to parse valid address");
}

#[test]
fn test_parse_immediate() {
    let input = "42";
    let result = immediate(input);
    assert!(result.is_ok(), "Failed to parse valid immediate value");
}

#[test]
fn test_parse_register() {
    let input = "R1,";
    let result = register(input);
    assert!(result.is_ok(), "Failed to parse valid register");
}

#[test]
fn test_parse_opcode() {
    let input = "ADD";
    let result = opcode(input);
    assert!(result.is_ok(), "Failed to parse valid opcode");
}

#[test]
fn test_parse_label() {
    let input = ".my_label";
    let result = label(input);
    assert!(result.is_ok(), "Failed to parse valid label");
}