mod elf;
mod lang;

use std::fs::File;
use std::fs::read_to_string;
use std::io::{self};

use lang::parser::Assembler6502Parser;
use lang::parser::Rule;

use pest::Parser;

use elf::writer::write_elf_header;
fn main() -> io::Result<()> {
    // read_to_string a file thats argument 1 on the command line
    let unparsed = read_to_string(std::env::args().nth(1).unwrap())?;
    let parsed = Assembler6502Parser::parse(Rule::program, &unparsed);
    // let parsed = Assembler6502Parser::parse(Rule::program, &unparsed);
    match parsed {
        Ok(pairs) => {
            for pair in pairs {
                println!("{:?}", pair);
            }
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
        }
    }

    return Ok(());
}
