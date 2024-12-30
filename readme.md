# Simple Assembler
This is a simple command line application that takes in an 
.asm file with your source code and outputs a .bin file with the
machine code. The assembler is based off of the 
[BatPU-2](https://github.com/mattbatwings/BatPU-2)
with some minor syntax changes. 

The assembler is written in Rust and uses the following libraries:
- [Nom](https://crates.io/crates/nom) for parsing
- [Clap](https://crates.io/crates/clap) for command line arguments
- [pretty-env-logging](https://crates.io/crates/pretty_env_logger) for logging
- [Custom Error](https://crates.io/crates/custom_error) for simple errors
- [Anyhow](https://crates.io/crates/anyhow) because I'm lazy

## This is a work in progress

This project is not complete, there is a lot of testing that needs to be done.
If you grab this project and try to use it, you will likely run into several issues. 
I know about these issues, and I am working on them.
Also, I am not an expert in Rust, so if you see something that could be done simpler, let me know.