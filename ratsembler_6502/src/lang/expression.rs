use super::instruction::AddressModeIndexer;
use super::instruction::InstructionCode;
use super::instruction::INSTRUCTION_MAP;
use super::instruction::INSTRUCTION_STR_MAP;

use pest::iterators::Pair;
use pest::iterators::Pairs;

use super::parser::Rule;
use pest::Parser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShortOperand {
    Numeric(u8),
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LongOperand {
    Numeric(u16),
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
pub struct Expression {
    operator: InstructionCode,
    pub operand: AddressValue,
}

impl AddressValue {
    pub fn to_indexer(&self) -> AddressModeIndexer {
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

    pub fn get_size(&self) -> usize {
        match self {
            AddressValue::Accumulator => 1,
            AddressValue::Implied => 1,
            AddressValue::Immediate(_) => 2,
            AddressValue::Absolute(_) => 3,
            AddressValue::ZeroPage(_) => 2,
            AddressValue::Relative(_) => 2,
            AddressValue::AbsoluteIndirect(_) => 3,
            AddressValue::AbsoluteX(_) => 3,
            AddressValue::AbsoluteY(_) => 3,
            AddressValue::ZeroPageX(_) => 2,
            AddressValue::ZeroPageY(_) => 2,
            AddressValue::IndexedIndirect(_) => 2,
            AddressValue::IndirectIndexed(_) => 2,
        }
    }

    pub fn from_indexed_addresser(addresser: Pair<Rule>) -> AddressValue {
        let mut inner_pairs = addresser.into_inner();
        let address: Pair<Rule> = inner_pairs.next().unwrap();
        match address.as_rule() {
            Rule::short_literal => match inner_pairs.next().unwrap().as_str() {
                ",X" => AddressValue::ZeroPageX(ShortOperand::Numeric(
                    u8::from_str_radix(&address.as_str()[1..], 16).unwrap(),
                )),
                ",Y" => AddressValue::ZeroPageY(ShortOperand::Numeric(
                    u8::from_str_radix(&address.as_str()[1..], 16).unwrap(),
                )),
                _ => {
                    unreachable!()
                }
            },
            Rule::label => match inner_pairs.next().unwrap().as_str() {
                ",X" => AddressValue::ZeroPageX(ShortOperand::Label(address.as_str().into())),
                ",Y" => AddressValue::ZeroPageY(ShortOperand::Label(address.as_str().into())),
                _ => {
                    unreachable!()
                }
            },
            Rule::long_literal => match inner_pairs.next().unwrap().as_str() {
                ",X" => AddressValue::AbsoluteX(LongOperand::Numeric(
                    u16::from_str_radix(&address.as_str()[1..], 16).unwrap(),
                )),
                ",Y" => AddressValue::AbsoluteY(LongOperand::Numeric(
                    u16::from_str_radix(&address.as_str()[1..], 16).unwrap(),
                )),
                _ => {
                    unreachable!()
                }
            },
            _ => {
                unreachable!()
            }
        }
    }
}

impl LongOperand {
    pub fn from_indirect_addresser(addresser: Pair<super::parser::Rule>) -> LongOperand {
        assert_eq!(addresser.as_rule(), super::parser::Rule::indirect_addresser);
        let mut inner_pairs = addresser.into_inner();
        let address: Pair<Rule> = inner_pairs.next().unwrap();
        match address.as_rule() {
            Rule::label => LongOperand::Label(address.as_str().into()),
            Rule::long_literal => {
                LongOperand::Numeric(u16::from_str_radix(&address.as_str()[1..], 16).unwrap())
            }
            _ => {
                unreachable!()
            }
        }
    }
}

impl Expression {
    pub fn get_code(&self) -> &u8 {
        INSTRUCTION_MAP
            .get(&(self.operator, self.operand.to_indexer()))
            .unwrap()
    }

    pub fn get_size(&self) -> usize {
        self.operand.get_size()
    }

    pub fn from_expression_pair(
        expression: Pair<super::parser::Rule>,
    ) -> (Vec<String>, Expression) {
        assert_eq!(expression.as_rule(), super::parser::Rule::expression);
        println!("parsing expression {:?}", expression);

        let mut labels: Vec<String> = Vec::new();
        // We're going to assume that we actually have an Expression here.
        // Going by the PEG we made, there should be zero or more labels,
        // one operator, and, at most, one operand.

        let mut inner_pairs = expression.into_inner();
        while inner_pairs.peek().unwrap().as_rule() == super::parser::Rule::label_dec {
            let mut label = inner_pairs.next().unwrap().as_str().to_string();
            label.pop();
            labels.push(label);
        }

        let operation: InstructionCode = {
            let operation_str = inner_pairs.next().unwrap().as_str().to_uppercase();
            *INSTRUCTION_STR_MAP.get(operation_str.as_str()).unwrap()
        };

        let operand: AddressValue = {
            if let Some(address_value) = inner_pairs.next() {
                match address_value.as_rule() {
                    Rule::indirect_addresser => AddressValue::AbsoluteIndirect(
                        LongOperand::from_indirect_addresser(address_value),
                    ),
                    Rule::immediate_addresser => {
                        let mut inner_pairs = address_value.into_inner();
                        // print the tokens that come through here
                        println!("{:?}", inner_pairs);
                        AddressValue::Immediate(ShortOperand::Numeric(
                            u8::from_str_radix(&inner_pairs.next().unwrap().as_str()[1..], 16)
                                .unwrap(),
                        ))
                    }
                    Rule::indexed_addresser => AddressValue::from_indexed_addresser(address_value),
                    Rule::indexed_indirect_addresser => {
                        let mut inner_pairs = address_value.into_inner();
                        let address: Pair<Rule> = inner_pairs.next().unwrap();
                        assert_eq!(address.as_rule(), Rule::short_literal);
                        AddressValue::IndexedIndirect(ShortOperand::Numeric(
                            u8::from_str_radix(&address.as_str()[1..], 16).unwrap(),
                        ))
                    }
                    Rule::indirect_indexed_addresser => {
                        let mut inner_pairs: Pairs<Rule> = address_value.into_inner();
                        let address: Pair<Rule> = inner_pairs.next().unwrap();
                        assert_eq!(address.as_rule(), Rule::short_literal);
                        AddressValue::IndirectIndexed(ShortOperand::Numeric(
                            u8::from_str_radix(&address.as_str()[1..], 16).unwrap(),
                        ))
                    }
                    Rule::short_literal => match operation {
                        InstructionCode::BCC
                        | InstructionCode::BCS
                        | InstructionCode::BEQ
                        | InstructionCode::BMI
                        | InstructionCode::BNE
                        | InstructionCode::BPL
                        | InstructionCode::BVC
                        | InstructionCode::BVS => AddressValue::Relative(ShortOperand::Numeric(
                            u8::from_str_radix(&address_value.as_str()[1..], 16).unwrap(),
                        )),
                        _ => AddressValue::ZeroPage(ShortOperand::Numeric(
                            u8::from_str_radix(&address_value.as_str()[1..], 16).unwrap(),
                        )),
                    },
                    Rule::long_literal => AddressValue::Absolute(LongOperand::Numeric(
                        u16::from_str_radix(&address_value.as_str()[1..], 16).unwrap(),
                    )),
                    Rule::label => match operation {
                        InstructionCode::BCC
                        | InstructionCode::BCS
                        | InstructionCode::BEQ
                        | InstructionCode::BMI
                        | InstructionCode::BNE
                        | InstructionCode::BPL
                        | InstructionCode::BVC
                        | InstructionCode::BVS => AddressValue::Relative(ShortOperand::Label(
                            address_value.as_str().into(),
                        )),
                        _ => AddressValue::Absolute(LongOperand::Label(
                            address_value.as_str().into(),
                        )),
                    },
                    _ => {
                        unreachable!()
                    }
                }
            } else {
                match operation {
                    InstructionCode::ASL
                    | InstructionCode::ROL
                    | InstructionCode::LSR
                    | InstructionCode::ROR => AddressValue::Accumulator,
                    _ => AddressValue::Implied,
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
}

#[cfg(test)]
mod tests {
    use super::super::parser::Assembler6502Parser;
    use super::*;

    #[test]
    fn test_to_indexer() {
        assert_eq!(
            AddressValue::Accumulator.to_indexer(),
            AddressModeIndexer::ACCUMULATOR
        );
        assert_eq!(
            AddressValue::Implied.to_indexer(),
            AddressModeIndexer::IMPLIED
        );
        assert_eq!(
            AddressValue::Immediate(ShortOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::IMMEDIATE
        );
        assert_eq!(
            AddressValue::Absolute(LongOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::ABSOLUTE
        );
        assert_eq!(
            AddressValue::ZeroPage(ShortOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::ZERO_PAGE
        );
        assert_eq!(
            AddressValue::Relative(ShortOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::RELATIVE
        );
        assert_eq!(
            AddressValue::AbsoluteIndirect(LongOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::ABSOLUTE_INDIRECT
        );
        assert_eq!(
            AddressValue::AbsoluteX(LongOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::ABS_X
        );
        assert_eq!(
            AddressValue::AbsoluteY(LongOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::ABS_Y
        );
        assert_eq!(
            AddressValue::ZeroPageX(ShortOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::ZP_X
        );
        assert_eq!(
            AddressValue::ZeroPageY(ShortOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::ZP_Y
        );
        assert_eq!(
            AddressValue::IndexedIndirect(ShortOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::INDEX_IND
        );
        assert_eq!(
            AddressValue::IndirectIndexed(ShortOperand::Numeric(0)).to_indexer(),
            AddressModeIndexer::IND_INDEX
        );
    }

    #[test]
    fn test_get_size() {
        assert_eq!(AddressValue::Accumulator.get_size(), 1);
        assert_eq!(AddressValue::Implied.get_size(), 1);
        assert_eq!(
            AddressValue::Immediate(ShortOperand::Numeric(0)).get_size(),
            2
        );
        assert_eq!(
            AddressValue::Absolute(LongOperand::Numeric(0)).get_size(),
            3
        );
        assert_eq!(
            AddressValue::ZeroPage(ShortOperand::Numeric(0)).get_size(),
            2
        );
        assert_eq!(
            AddressValue::Relative(ShortOperand::Numeric(0)).get_size(),
            2
        );
        assert_eq!(
            AddressValue::AbsoluteIndirect(LongOperand::Numeric(0)).get_size(),
            3
        );
        assert_eq!(
            AddressValue::AbsoluteX(LongOperand::Numeric(0)).get_size(),
            3
        );
        assert_eq!(
            AddressValue::AbsoluteY(LongOperand::Numeric(0)).get_size(),
            3
        );
        assert_eq!(
            AddressValue::ZeroPageX(ShortOperand::Numeric(0)).get_size(),
            2
        );
        assert_eq!(
            AddressValue::ZeroPageY(ShortOperand::Numeric(0)).get_size(),
            2
        );
        assert_eq!(
            AddressValue::IndexedIndirect(ShortOperand::Numeric(0)).get_size(),
            2
        );
        assert_eq!(
            AddressValue::IndirectIndexed(ShortOperand::Numeric(0)).get_size(),
            2
        );
    }

    #[test]
    fn test_from_indexed_addresser() {
        let pairs = Assembler6502Parser::parse(Rule::indexed_addresser, "$10,X").unwrap();
        let pair = pairs.into_iter().next().unwrap();
        assert_eq!(
            AddressValue::from_indexed_addresser(pair),
            AddressValue::ZeroPageX(ShortOperand::Numeric(0x10))
        );

        let pairs = Assembler6502Parser::parse(Rule::indexed_addresser, "$10,Y").unwrap();
        let pair = pairs.into_iter().next().unwrap();
        assert_eq!(
            AddressValue::from_indexed_addresser(pair),
            AddressValue::ZeroPageY(ShortOperand::Numeric(0x10))
        );
    }

    #[test]
    fn test_from_indirect_addresser() {
        let pairs = Assembler6502Parser::parse(Rule::indirect_addresser, "($1234)").unwrap();
        let pair = pairs.into_iter().next().unwrap();
        assert_eq!(
            LongOperand::from_indirect_addresser(pair),
            LongOperand::Numeric(0x1234)
        );

        let pairs = Assembler6502Parser::parse(Rule::indirect_addresser, "(label)").unwrap();
        let pair = pairs.into_iter().next().unwrap();
        assert_eq!(
            LongOperand::from_indirect_addresser(pair),
            LongOperand::Label("label".into())
        );
    }

    #[test]
    fn test_expression_from_expression_pair() {
        let pairs = Assembler6502Parser::parse(Rule::expression, "LDA #$10").unwrap();
        let pair = pairs.into_iter().next().unwrap();
        let (labels, expression) = Expression::from_expression_pair(pair);
        assert!(labels.is_empty());
        assert_eq!(expression.operator, InstructionCode::LDA);
        assert_eq!(
            expression.operand,
            AddressValue::Immediate(ShortOperand::Numeric(0x10))
        );

        let pairs = Assembler6502Parser::parse(Rule::expression, "label: LDA $10,X").unwrap();
        let pair = pairs.into_iter().next().unwrap();
        let (labels, expression) = Expression::from_expression_pair(pair);
        assert_eq!(labels, vec!["label".to_string()]);
        assert_eq!(expression.operator, InstructionCode::LDA);
        assert_eq!(
            expression.operand,
            AddressValue::ZeroPageX(ShortOperand::Numeric(0x10))
        );
    }
}
