pub mod operand;
pub mod opcode;
pub mod instruction;

pub const KEYWORDS: [&str; 76] = [
    "define",
    // Opcodes (1-23)
    "nop", "hlt", "add", "sub",
    "nor", "and", "xor", "rsh",
    "ldi", "adi", "jmp", "brh",
    "cal", "ret", "lod", "str",
    "cmp", "mov", "lsh", "inc",
    "dec", "not", "neg",
    // Registers (24-39)
    "r0", "r1", "r2", "r3",
    "r4", "r5", "r6", "r7",
    "r8", "r9", "r10", "r11",
    "r12", "r13", "r14", "r15",
    // Conditions (40-55)
    "zs", "zc", "cs", "cc",
    "lt", "ge", "eq", "ne",
    "=", "!=", ">=", "<",
    "nc", "c", "z", "nz",
    "notcarry", "carry", "zero", "notzero",
    // Ports (56-75)
    "pixel_x", "pixel_y", "draw_pixel", "clear_pixel",
    "load_pixel", "buffer_screen", "clear_screen_buffer", "write_char", "buffer_chars",
    "clear_chars_buffer", "show_number", "clear_number", "signed_mode", "unsigned_mode",
    "rng", "controller_input"
];