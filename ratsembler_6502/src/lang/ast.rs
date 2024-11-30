use super::instruction::AddressModeIndexer;
use super::instruction::InstructionCode;
use super::instruction::INSTRUCTION_STR_MAP;

use super::parser::Rule;

use pest::iterators::Pair;
use pest::iterators::Pairs;

use std::vec::Vec;

pub enum ShortOperand {
    Numeric(u8),
    Label(String),
}

pub enum LongOperand {
    Numeric(u16),
    Label(String),
}

pub enum AddressValue {
    Accumulator,
    Implied,
    Immediate(ShortOperand),
    Absolute(LongOperand),
    ZeroPage(ShortOperand),
    Relative(ShortOperand),
    /* Absulte Indirect: refers to a little
     * endian two byte value stored at the
     * specified address. Only used by JMP */
    AbsoluteIndirect(LongOperand),
    AbsoluteX(LongOperand),
    AbsoluteY(LongOperand),
    ZeroPageX(ShortOperand),
    ZeroPageY(ShortOperand),
    IndexedIndirect(ShortOperand), // (ZP, X)
    IndirectIndexed(ShortOperand), // (ZP), Y
}
pub struct Expression {
    operator: InstructionCode,
    operand: AddressValue,
}

pub struct Program {
    expressions: Vec<Expression>,
    labels: Vec<String>,
}

impl AddressValue {
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

impl LongOperand {
    pub fn from_indirect_addresser(addresser: Pair<super::parser::Rule>) -> LongOperand {
        let inner_address: String = addresser.into_inner().next().unwrap().as_str().to_string();
        if let Ok(value) = u16::from_str_radix(&inner_address.as_str()[1..], 16) {
            LongOperand::Numeric(value)
        } else {
            LongOperand::Label(inner_address)
        }
    }
}

fn parse_expression(expression: Pair<super::parser::Rule>) -> (Vec<String>, Expression) {
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
                    AddressValue::AbsoluteIndirect(LongOperand::from_indirect_addresser(address_value))
                }
                _ => {
                    unreachable!()
                }
            }
        } else {
            match operation {
                InstructionCode::ASL |
                InstructionCode::ROL |
                InstructionCode::LSR |
                InstructionCode::ROR => {
                    AddressValue::Accumulator
                }
                _ => {
                    AddressValue::Implied
                }
            }
        }
    };

    (
        labels,
        Expression {
            operator: operation,
            operand: operand,
        },
    )
}

impl Program {
    fn from_pairs(pairs: Pairs<Rule>) -> Program {
        unimplemented!();
    }
}
