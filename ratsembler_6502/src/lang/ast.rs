use super::instruction::InstructionCode;
use super::instruction::AddressModeIndexer;
use std::vec::Vec;

use pest::iterators::Pairs;

pub enum ShortOperand<'a> {
    Numeric(u8),
    Label(&'a str),
}

pub enum LongOperand<'a> {
    Numeric(u16),
    Label(&'a str),
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
    labels: Vec<&'a str>,
    operator: InstructionCode,
    operand: AddressValue<'a>,
}

pub struct Program<'program> {
    expressions: Vec<Expression<'program>>,
    labels: Vec<Box<str>>,
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

impl<'program> Program<'program> {
    fn from_pairs(pairs: Pairs<'program, super::parser::Rule>) -> Program<'program> {
        unimplemented!();
    }
}