use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, Lines};
use std::path::{Path, PathBuf};
use std::env::args;
use std::ffi::OsStr;
use regex::Regex;

enum Operations {
    NOP, ADI, ADC, SBB, AND, ORR, XOR, SLT, LSL, LSR, ASR, LDR, STR, BRC, JMP, HLT
}

enum Conditions {
    EQ, NE, GT, GE, LT, LE, CC, CS
}

impl Conditions {
    fn from_str(string: &str) -> Option<Conditions> {
        match string {
            "EQ" => Some(Conditions::EQ),
            "NE" => Some(Conditions::NE),
            "GT" => Some(Conditions::GT),
            "GE" => Some(Conditions::GE),
            "LT" => Some(Conditions::LT),
            "LE" => Some(Conditions::LE),
            "CC" => Some(Conditions::CC),
            "CS" => Some(Conditions::CS),
            _ => None
        }
    }

    pub fn value(&self) -> u8 {
        match *self {
            Conditions::EQ => 0,
            Conditions::NE => 1,
            Conditions::LT => 2,
            Conditions::LE => 3,
            Conditions::GT => 4,
            Conditions::GE => 5,
            Conditions::CS => 6,
            Conditions::CC => 7
        }
    }
}

impl Operations {
    fn from_str(string: &str) -> Option<Operations> {
        match string {
            "NOP" => Some(Operations::NOP),
            "ADI" => Some(Operations::ADI),
            "ADC" => Some(Operations::ADC),
            "SBB" => Some(Operations::SBB),
            "AND" => Some(Operations::AND),
            "ORR" => Some(Operations::ORR),
            "XOR" => Some(Operations::XOR),
            "SLT" => Some(Operations::SLT),
            "LSL" => Some(Operations::LSL),
            "LSR" => Some(Operations::LSR),
            "ASR" => Some(Operations::ASR),
            "LDR" => Some(Operations::LDR),
            "STR" => Some(Operations::STR),
            "BRC" => Some(Operations::BRC),
            "JMP" => Some(Operations::JMP),
            "HLT" => Some(Operations::HLT),
            _ => None
        }
    }

    pub fn value(&self) -> u8 {
        match *self {
            Operations::NOP => 0,
            Operations::ADC => 1,
            Operations::SBB => 2,
            Operations::AND => 3,
            Operations::ORR => 4,
            Operations::XOR => 5,
            Operations::SLT => 6,
            Operations::ADI => 7,
            Operations::LSL => 8,
            Operations::LSR => 9,
            Operations::ASR => 10,
            Operations::LDR => 11,
            Operations::STR => 12,
            Operations::BRC => 13,
            Operations::JMP => 14,
            Operations::HLT => 15
        }
    }
}

fn get_file() -> Result<PathBuf, io::Error> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Error: Missing .asm file\nUsage: {} <file_name>.asm\n", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid arguments"));
    }
    let input_path = PathBuf::from(&args[1]);
    if input_path.extension() != Some(OsStr::new("asm")) {
        eprintln!("Error: File must be a .asm file\n");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file extension"));
    }
    Ok(input_path)
}

fn parse_asm(asm_path: &PathBuf) -> Result<Vec<u8>, io::Error> {
    let asm_lines: Lines<BufReader<File>> = read_lines(asm_path)?;
    let mut machine_code: Vec<u8> = Vec::new();
    let syntax_regex: Regex = Regex::new(
        r"^\s*(\w{3})\s*(r[0-9]+|0[x|X][0-9a-fA-F]+|0b[01]+|[0-9]+|[A-Z]{2}\b)?,?\s*(r[0-9]+|0[x|X][0-9a-fA-F]+|0b[01]+|[0-9]+)?,?\s*(r[0-9]+|0[x|X][0-9a-fA-F]+|0b[01]+|[0-9]+)?"
    ).unwrap();

    for line in asm_lines {
        if let Ok(instruction) = line {
            let trimmed = instruction.trim();

            if trimmed.is_empty()  || trimmed.starts_with(";") {
                continue;
            }

            if let Some(cap) = syntax_regex.captures(trimmed) {
                let op_str = cap.get(1).unwrap().as_str();
                let op = match Operations::from_str(op_str) {
                    Some(op) => op,
                    None => {
                        eprintln!("Unknown Operation: {}\n", op_str);
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown operation"));
                    }
                };

                let (instruction_high, instruction_low): (u8, u8) = match op {
                    Operations::NOP => (0x00, 0x00),
                    Operations::HLT => (0x00, 0x0F),
                    Operations::JMP => {
                        let num_str: &str = cap.get(2).unwrap().as_str();
                        let address: u16 = get_u16_from_str(num_str);

                        let low = ((address & 0xF) << 4) as u8 | op.value();
                        let high = (address >> 4) as u8;
                        (high, low)
                    }
                    Operations::ADI => {
                        let rd_str: &str = cap.get(2).unwrap().as_str();
                        let imm_str: &str = cap.get(3).unwrap().as_str();
                        let rd: u8 = get_register_number(rd_str).unwrap();
                        let imm: u8 = get_u8_from_str(imm_str);
                        (imm, (rd << 4) | op.value())
                    }
                    Operations::BRC => {
                        let cond_str: &str = cap.get(2).unwrap().as_str();
                        let offset_str: &str = cap.get(3).unwrap().as_str();
                        let cond: u8 = Conditions::from_str(cond_str).unwrap().value();
                        let offset: u8 = get_u8_from_str(offset_str);
                        (offset, (cond << 4) | op.value())
                    }
                    Operations::LDR => {
                        let rd_str: &str = cap.get(2).unwrap().as_str();
                        let imm_str: &str = cap.get(3).unwrap().as_str();
                        let rs1_str: &str = cap.get(4).unwrap().as_str();

                        let rd = get_register_number(rd_str).unwrap();
                        let imm = get_u8_from_str(imm_str);
                        let rs1 = get_register_number(rs1_str).unwrap();
                        ((imm << 4) | rs1, (rd << 4) | op.value())
                    }
                    Operations::STR => {
                        let rs2_str: &str = cap.get(2).unwrap().as_str();
                        let imm_str: &str = cap.get(3).unwrap().as_str();
                        let rs1_str: &str = cap.get(4).unwrap().as_str();

                        let rs2 = get_register_number(rs2_str).unwrap();
                        let imm = get_u8_from_str(imm_str);
                        let rs1 = get_register_number(rs1_str).unwrap();
                        ((rs2 << 4) | rs1, (imm << 4) | op.value())
                    }
                    _ => {
                        let rd_str: &str = cap.get(2).unwrap().as_str();
                        let rs1_str: &str = cap.get(3).unwrap().as_str();
                        let rs2_str: &str = cap.get(4).unwrap().as_str();

                        let rs2 = get_register_number(rs2_str).unwrap();
                        let rd = get_register_number(rd_str).unwrap();
                        let rs1 = get_register_number(rs1_str).unwrap();
                        ((rs2 << 4) | rs1, (rd << 4) | op.value())
                    }
                };

                machine_code.push(instruction_low);
                machine_code.push(instruction_high);
            } else {
                eprintln!("Invalid instruction format: {}\n", trimmed);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid instruction format"));
            }
        }
    }

    Ok(machine_code)
}

fn get_register_number(string: &str) -> Option<u8> {
    if string.starts_with("r") {
        let index = string[1..].parse::<u8>().unwrap();
        if index > 0x0F { None } else { Some(index) }
    } else { None }
}

fn get_u8_from_str(num_str: &str) -> u8 {
    if num_str.starts_with("0x") {
        u8::from_str_radix(&num_str[2..], 16).unwrap()
    } else if num_str.starts_with("0b") {
        u8::from_str_radix(&num_str[2..], 2).unwrap()
    } else {
        num_str.parse::<u8>().unwrap()
    }
}

fn get_u16_from_str(num_str: &str) -> u16 {
    if num_str.starts_with("0x") {
        u16::from_str_radix(&num_str[2..], 16).unwrap()
    } else if num_str.starts_with("0b") {
        u16::from_str_radix(&num_str[2..], 2).unwrap()
    } else {
        num_str.parse::<u16>().unwrap()
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn write_to_output(path: &PathBuf, machine_code: &mut Vec<u8>) -> Result<(), io::Error> {
    let mut output_file = File::create(path)?;
    output_file.write_all(&machine_code)
}

fn main() -> io::Result<()> {
    let input_path: PathBuf = get_file()?;
    let mut output_path: PathBuf = input_path.clone();
    output_path.set_extension("bin");

    //TODO: Add a preprocessing step
    // 1. Removing comments and empty lines from the file
    // 2. Converting Pseudo-Instructions to real instructions
    // 3. Implement Labels... eventually
    let mut machine_code: Vec<u8> = parse_asm(&input_path)?;
    write_to_output(&output_path, &mut machine_code)?;
    println!("Assembly compiled successfully! {} -> {}",
             input_path.display(), output_path.display()
    );
    Ok(())
}
