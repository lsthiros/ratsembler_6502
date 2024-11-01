mod elf;
mod lang;

use std::fs::File;
use std::fs::read_to_string;
use std::io::{self};
use std::u32;

use lang::parser::Assembler6502Parser;
use lang::parser::Rule;

use pest::Parser;

use elf::writer::write_elf_header;
fn main() -> io::Result<()> {
    // read_to_string a file thats argument 1 on the command line
    println!("Parsiing file: {}", std::env::args().nth(1).unwrap());
    let unparsed = read_to_string(std::env::args().nth(1).unwrap())?;
    let parsed = Assembler6502Parser::parse(Rule::program, &unparsed);
    println!("Parsed");
    match parsed {
        Ok(pairs) => {
            let count: u32 = pairs
                .map(|pair| {
                    pair.into_inner().fold(0, |acc, inner_pair| {
                        println!("Rule:    {:?}", inner_pair.as_rule());
                        println!("{:?}", inner_pair);
                        acc + 1
                    })
                })
                .sum();
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
        }
    }

    return Ok(());
}
