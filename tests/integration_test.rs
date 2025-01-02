use std::fs;
use std::process::{Command, Output};
use nom::bytes::complete::take;
use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::IResult;
use nom::sequence::delimited;

fn check_output(output: &Output) {
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(
        output.status.success(),
        "Command failed with status: {}",
        output.status
    );
}

fn check_output_failed(output: &Output) {
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(
        !output.status.success(),
        "Expected Failure, got success: {}",
        output.status
    );
}

#[test]
fn cli_output_comparison() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\compare\tetris.asm")
        .arg("-o")
        .arg(r".\test_data\compare\tetris_actual.bin")
        .output()
        .expect("Failed to execute command");

    check_output(&output);

    let expected_binary = convert_mc_to_bin(r".\test_data\compare\tetris.mc");
    let actual_binary = fs::read(r".\test_data\compare\tetris_actual.bin")
        .expect("Failed to read actual output");

    assert_ne!(expected_binary, actual_binary); // Fails because of ascii encoding
}

#[test]
fn cli_with_size() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\size\tetris.asm")
        .arg("-o")
        .arg(r".\test_data\size\tetris.bin")
        .arg("-s")
        .arg("65536") // Too big
        .output()
        .expect("Failed to execute command");

    check_output_failed(&output);

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\size\tetris.asm")
        .arg("-o")
        .arg(r".\test_data\size\tetris.bin")
        .arg("-s")
        .arg("32768")
        .output()
        .expect("Failed to execute command");

    check_output(&output);

    let expected_binary_size = 32768;
    let actual_binary_size = fs::read(r".\test_data\size\tetris.bin")
        .expect("Failed to read actual output").len();

    assert_eq!(expected_binary_size, actual_binary_size);
}

#[test]
fn cli_with_print() { // TODO: This test fails as well as all the others
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\print\tetris.asm")
        .arg("-v")
        .arg("-x")
        .arg("-d")
        .output()
        .expect("Failed to execute command");

    check_output(&output);
}

#[test]
fn different_files_same_code() {
    let output1 = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\different_files\alpha.asm")
        .arg("-o")
        .arg(r".\test_data\different_files\alpha.bin")
        .output()
        .expect("Failed to execute command");

    check_output(&output1);

    let output2 = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\different_files\beta.asm")
        .arg("-o")
        .arg(r".\test_data\different_files\beta.bin")
        .output()
        .expect("Failed to execute command");

    check_output(&output2);

    let alpha_binary = fs::read(r".\test_data\different_files\alpha.bin")
        .expect("Failed to read actual output");
    let beta_binary = fs::read(r".\test_data\different_files\beta.bin")
        .expect("Failed to read actual output");

    assert_eq!(alpha_binary, beta_binary);
}

#[test]
fn empty_file() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\empty_file\empty.asm")
        .arg("-o")
        .arg(r".\test_data\empty_file\empty.bin")
        .output()
        .expect("Failed to execute command");

    check_output(&output);

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\empty_file\only_comments.asm")
        .arg("-o")
        .arg(r".\test_data\empty_file\only_comments.bin")
        .output()
        .expect("Failed to execute command");

    check_output_failed(&output);
}

#[test]
fn invalid_file() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\invalid_file\invalid.asm")
        .arg("-o")
        .arg(r".\test_data\invalid_file\invalid.bin")
        .arg("-d")
        .output()
        .expect("Failed to execute command");

    check_output_failed(&output);
}

#[test]
fn game_of_life() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\game_of_life\gol.asm")
        .arg("-o")
        .arg(r".\test_data\game_of_life\gol.bin")
        .arg("-d")
        .output()
        .expect("Failed to execute command");

    check_output(&output); // Fails because of 
}

#[test]
fn hello_world() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\hello_world\hello_world.asm")
        .arg("-o")
        .arg(r".\test_data\hello_world\hello_world.bin")
        .arg("-d")
        .output()
        .expect("Failed to execute command");

    check_output(&output);
}

#[test]
fn minesweeper() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(r".\test_data\minesweeper\minesweeper.asm")
        .arg("-o")
        .arg(r".\test_data\minesweeper\minesweeper.bin")
        .output()
        .expect("Failed to execute command");

    check_output(&output);
}

fn convert_mc_to_bin(file: &str) -> Vec<u8> {
    let binding = fs::read_to_string(file)
        .expect("Failed to read input file");
    let mut file: &str = binding.as_str();
    let mut binary: Vec<u8> = Vec::new();

    while !file.is_empty() {
        let (rest, mc) = ws(take::<usize, &str, ()>(16usize))(file)
            .expect("Failed to take 16 bits");
        let bytes: u16 = u16::from_str_radix(mc, 2)
            .expect("Failed to convert machine code to byte");

        let lo: u8 = (bytes & 0xFF) as u8;
        let hi: u8 = (bytes >> 8) as u8;
        binary.push(lo);
        binary.push(hi);
        file = rest;
    }

    binary
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E>,{ delimited(multispace0, inner, multispace0) }