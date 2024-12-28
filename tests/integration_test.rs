use std::fs;
use std::process::Command;
use nom::bytes::complete::take;
use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::IResult;
use nom::sequence::delimited;

#[test]
fn test_cli() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg("test_data/tetris.asm")
        .arg("-o")
        .arg("test_data/tetris_actual.bin")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let expected_binary = convert_mc_to_bin("test_data/tetris.mc");
    let actual_binary = fs::read("test_data/tetris_actual.bin").expect("Failed to read actual output");

    assert_eq!(expected_binary, actual_binary);
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