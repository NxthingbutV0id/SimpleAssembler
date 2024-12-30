use crate::symbols::{opcodes::Opcode, operands::*, instruction::Instruction};

pub struct AssemblyPrinter {
    program: Vec<Instruction>,
    output: Option<String>
}

impl AssemblyPrinter {
    pub fn new(program: &Vec<Instruction>) -> AssemblyPrinter {
        AssemblyPrinter {
            program: program.clone(),
            output: None
        }
    }

    pub fn print(&mut self) -> String {
        self.output = Some("".to_string());
        let program = self.program.clone();
        self.emit("\naddr |     encoding     |       instructions\n");
        self.emit("---- | ---------------- | -----------------------\n");
        for instruction in program {
            self.print_instruction(&instruction);
        }
        let s = self.output.clone().unwrap();
        self.output = None;
        s
    }

    fn print_instruction(&mut self, instruction: &Instruction) {
        let opcode = &instruction.opcode;
        let operands = &instruction.operands;
        let location = instruction.address;
        let encoding = instruction.encoding;

        if let Some(location) = location {
            self.emit(&format!("{:04X} |", location));
        } else {
            self.emit("---- |");
        }

        if let Some(encoding) = encoding {
            self.emit(&format!(" {:016b} |", encoding));
        } else {
            self.emit(" ---------------- |");
        }
        self.print_opcode(opcode);
        self.print_operands(operands);
        self.emit("\n");
    }

    fn print_operands(&mut self, operands: &Vec<Operand>) {
        for i in 0..operands.len() {
            if i > 0 {
                self.emit(", ");
            }
            match &operands[i] {
                Operand::Register(reg) => {
                    self.emit(&format!("r{}", *reg as u8));
                },
                Operand::Immediate(i) => self.emit(&format!("{}", *i)),
                Operand::Condition(cond) => {
                    match cond {
                        Condition::CC => self.emit("lt"),
                        Condition::CS => self.emit("ge"),
                        Condition::ZC => self.emit("ne"),
                        Condition::ZS => self.emit("eq")
                    }
                }
                Operand::Address(addr) => self.emit(&format!("0x{:04X}", *addr)),
                Operand::Offset(offset) => self.emit(&format!(".{}", *offset)),
                Operand::Label(label) => self.emit(&format!("{}", label)),
                Operand::Definition(def) => self.emit(&format!("{}", def)),
                Operand::Port(port) => self.emit(&format!("{}", port)),
                Operand::Character(c) => self.emit(&format!("{}", c))
            }
        }
    }

    fn print_opcode(&mut self, opcode: &Opcode) {
        if *opcode == Opcode::_Label {
            self.emit(".");
            return;
        }
        if *opcode == Opcode::_Definition {
            self.emit("define ");
            return;
        }
        self.emit(&format!("    {opcode}  "));
    }

    fn emit(&mut self, s: &str) {
        self.output.as_mut().unwrap().push_str(s);
    }
}