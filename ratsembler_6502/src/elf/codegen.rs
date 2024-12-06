use crate::lang::ast::Program;
use crate::lang::instruction::INSTRUCTION_MAP;

pub fn serialize_raw_section(program: &Program) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    for instruction in program.expressions.iter() {
        let opcode: u8 = match INSTRUCTION_MAP.get(instruction.operator) {
            Some(op) => *op,
            None => panic!("Unknown instruction: {}", instruction.name),
        };
    }

    unimplemented!();
}