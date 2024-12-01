mod elf;
mod lang;

use std::fs::read_to_string;
use std::io::{self};

#[macro_use]
extern crate lazy_static;

use lang::parser::Assembler6502Parser;
use lang::parser::Rule;
use lang::ast::Program;

use pest::Parser;

fn main() -> io::Result<()> {
    // read_to_string a file thats argument 1 on the command line
    println!("Parsiing file: {}", std::env::args().nth(1).unwrap());
    let unparsed = read_to_string(std::env::args().nth(1).unwrap())?;
    let parsed = Assembler6502Parser::parse(Rule::program, &unparsed);
    match parsed {
        Ok(mut pairs) => {
            let program_pairs = pairs.next().unwrap();
            let program = Program::from_pairs(program_pairs.into_inner());
            println!("{:?}", program);
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
        }
    }

    return Ok(());
}
