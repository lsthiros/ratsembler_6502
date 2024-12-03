use pest::Parser;

use ratsembler_6502::lang::ast::Program;
use ratsembler_6502::lang::parser::Assembler6502Parser;
use ratsembler_6502::lang::parser::Rule;

#[test]
fn test_parser_list() {
    let input = r#"
    one:
    NOP

    two:
    three:
    NOP ($23,X)
    NOP ($23),Y
    NOP

    four:
    NOP
    NOP (hello)
    NOP
    "#;

    let parsed = Assembler6502Parser::parse(Rule::program, input);
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
}
