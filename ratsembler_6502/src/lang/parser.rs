use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lang/assembly_6502.pest"]
pub struct Assembler6502Parser;
