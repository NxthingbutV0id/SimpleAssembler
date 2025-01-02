use crate::symbols::{opcodes::Opcode, instruction::Instruction};
use crate::symbols::operands::Operand;

pub struct AssemblyPrinter<'a> {
    program: &'a [Instruction],
    output: Option<String>
}

impl<'a> AssemblyPrinter<'a> {
    pub fn new(program: &[Instruction]) -> AssemblyPrinter {
        AssemblyPrinter {
            program,
            output: None
        }
    }

    pub fn print(&mut self) -> String {
        self.output = Some("".to_string());
        self.emit("\naddr |     encoding     |       instructions\n");
        self.emit("---- | ---------------- | -----------------------\n");
        for instruction in self.program {
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
            self.emit(&format!("{:04X} |", location.value() << 1));
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
            self.emit(&format!("{}", operands[i]));
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