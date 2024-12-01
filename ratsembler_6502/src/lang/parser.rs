use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lang/assembly_6502.pest"]
pub struct Assembler6502Parser;

#[cfg(test)]
mod tests {
    use crate::lang::ast::Program;

    use super::*;

    #[test]
    fn test_parse_valid_instruction() {
        let parse_result = Assembler6502Parser::parse(Rule::instruction, "LDA");
        println!("{:?}", parse_result);
        assert!(parse_result.is_ok());
    }

    #[test]
    fn test_parse_invalid_instruction() {
        let parse_result = Assembler6502Parser::parse(Rule::instruction, "INVALID");
        assert!(parse_result.is_err());
    }

    #[test]
    fn test_parse_valid_label_declaration() {
        let parse_result = Assembler6502Parser::parse(Rule::label_dec, "start:");
        assert!(parse_result.is_ok());
    }

    #[test]
    fn test_parse_invalid_label() {
        let parse_result = Assembler6502Parser::parse(Rule::label, "123start:");
        assert!(parse_result.is_err());
    }

    #[test]
    fn test_parse_label_with_instruction_name() {
        let parse_result = Assembler6502Parser::parse(Rule::label, "decrement:");
        assert!(parse_result.is_ok());
    }

    #[test]
    fn test_parse_two_operand_free_instructions() {
        let parser_result = Assembler6502Parser::parse(Rule::program, "INX\nDEX\n");
        assert!(parser_result.is_ok(), "Expected successful parse, got {:?}", parser_result);
        let mut program_pairs= parser_result.unwrap().next().unwrap().into_inner();
        let first_instruction = program_pairs.next().unwrap();
        assert_eq!(first_instruction.as_rule(), Rule::expression);
        let second_instruction = program_pairs.next().unwrap();
        assert_eq!(second_instruction.as_rule(), Rule::expression);
        let eoi = program_pairs.next().unwrap();
        assert_eq!(eoi.as_rule(), Rule::EOI);
        // validate that the program pair has two "expression" children
    }
}
