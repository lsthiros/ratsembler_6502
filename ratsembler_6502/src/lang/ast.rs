use super::instruction::InstructionCode;
use std::vec::Vec;

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