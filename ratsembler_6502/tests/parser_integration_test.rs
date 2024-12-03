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

#[test]
fn test_parser_og() {
    let input = r#"
  JSR init
  JSR loop
  JSR end

init:
  LDX #$00
  RTS

loop:
  INX
  CPX #$05
  BNE loop
  RTS

end:
  BRK
  
LDX #$08
decrement:
DEX
STX $0200
CPX #$03
BNE decrement
STX $0201
BRK
lda #$11
sta $10
lda #$10
sta $12
lda #$0f
sta $14
lda #$04
sta $11
sta $13
sta $15

catch:
  JMP catch

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

#[test]
fn test_small() {
    let input = r#"
JSR init
BRK

init:
LDX #$00
RTS


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

#[test]
fn test_test_asm() {
    let input = r#"
NOP
NOP (bepis,X)
NOP (bepis,Y)
NOP
NOP
NOP (hello)
NOP"#;

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

#[test]
fn test_two() {
    let input = r#"
INX
DEC"#;

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
