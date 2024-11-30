use super::instruction::InstructionCode;
use super::instruction::AddressModeIndexer;
use super::instruction::INSTRUCTION_STR_MAP;

use super::parser::Rule;

use pest::iterators::Pairs;
use pest::iterators::Pair;

use std::vec::Vec;


pub enum ShortOperand<'a> {
    Numeric(u8),
    Label(&'a String),
}

pub enum LongOperand<'a> {
    Numeric(u16),
    Label(&'a String),
}

pub enum AddressValue<'a> {
    Accumulator,
    Implied,
    Immediate(ShortOperand<'a>),
    Absolute(LongOperand<'a>),
    ZeroPage(ShortOperand<'a>),
    Relative(ShortOperand<'a>),
    /* Absulte Indirect: refers to a little
     * endian two byte value stored at the
     * specified address. Only used by JMP */
    AbsoluteIndirect(LongOperand<'a>),
    AbsoluteX(LongOperand<'a>),
    AbsoluteY(LongOperand<'a>),
    ZeroPageX(ShortOperand<'a>),
    ZeroPageY(ShortOperand<'a>),
    IndexedIndirect(ShortOperand<'a>), // (ZP, X)
    IndirectIndexed(ShortOperand<'a>), // (ZP), Y
}
pub struct Expression<'a> {
    operator: InstructionCode,
    operand: AddressValue<'a>,
}

pub struct Program<'program> {
    expressions: Vec<Expression<'program>>,
    labels: Vec<String>,
}

impl AddressValue<'_> {
    pub fn to_indexer(self) -> AddressModeIndexer {
        match self {
            AddressValue::Accumulator => AddressModeIndexer::ACCUMULATOR,
            AddressValue::Implied => AddressModeIndexer::IMPLIED,
            AddressValue::Immediate(ShortOperand::Numeric(_)) => AddressModeIndexer::IMMEDIATE,
            AddressValue::Immediate(ShortOperand::Label(_)) => AddressModeIndexer::IMMEDIATE,
            AddressValue::Absolute(_) => AddressModeIndexer::ABSOLUTE,
            AddressValue::ZeroPage(_) => AddressModeIndexer::ZERO_PAGE,
            AddressValue::Relative(_) => AddressModeIndexer::RELATIVE,
            AddressValue::AbsoluteIndirect(_) => AddressModeIndexer::ABSOLUTE_INDIRECT,
            AddressValue::AbsoluteX(_) => AddressModeIndexer::ABS_X,
            AddressValue::AbsoluteY(_) => AddressModeIndexer::ABS_Y,
            AddressValue::ZeroPageX(_) => AddressModeIndexer::ZP_X,
            AddressValue::ZeroPageY(_) => AddressModeIndexer::ZP_Y,
            AddressValue::IndexedIndirect(_) => AddressModeIndexer::INDEX_IND,
            AddressValue::IndirectIndexed(_) => AddressModeIndexer::IND_INDEX,
        }
    }
}

fn parse_expression(expression: Pair<super::parser::Rule>) -> (Vec<String>, Expression){
    assert_eq!(expression.as_rule(), super::parser::Rule::expression);

    let mut labels: Vec<String> = Vec::new();
    // We're going to assume that we actually have an Expression here.
    // Going by the PEG we made, there should be zero or more labels,
    // one operator, and, at most, one operand.

    let mut inner_pairs = expression.into_inner();
    while inner_pairs.peek().unwrap().as_rule() == super::parser::Rule::label_dec {
        labels.push(inner_pairs.next().unwrap().as_str().into());
    }

    let operation: InstructionCode = {
        let operation_str = inner_pairs.next().unwrap().as_str().to_uppercase();
        *INSTRUCTION_STR_MAP.get(operation_str.as_str()).unwrap()
    };

    let operand: AddressValue = {
        if let Some(address_value) = inner_pairs.next() {
            match address_value.as_rule() {
                Rule::indirect_addresser => {
                    unimplemented!();
                }
                _ =>
            {
                unreachable!()
            }}
        }
        else {
            AddressValue::Implied
        }
    };

    (labels,
    Expression {
        operator: operation,
        operand: operand,
    })

}

impl<'program> Program<'program> {
    fn from_pairs(pairs: Pairs<'program, super::parser::Rule>) -> Program<'program> {
        unimplemented!();
    }
}