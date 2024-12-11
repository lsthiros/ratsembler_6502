use super::instruction::AddressModeIndexer;
use super::instruction::InstructionCode;
use super::instruction::INSTRUCTION_MAP;
use super::instruction::INSTRUCTION_STR_MAP;

use super::parser::Rule;

use crate::elf::relocatable::Relocatable;
use crate::elf::relocatable::Relocation;
use crate::elf::relocatable::Symbol;

use pest::iterators::Pair;
use pest::iterators::Pairs;

use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

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
    operand: AddressValue,
}

#[derive(Debug)]
pub struct Program {
    pub expressions: Vec<Rc<Expression>>,
    pub labels: HashMap<String, (usize, Rc<Expression>)>,
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

impl Expression {
    pub fn get_code(&self) -> &u8 {
        INSTRUCTION_MAP
            .get(&(self.operator, self.operand.to_indexer()))
            .unwrap()
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

fn parse_expression(expression: Pair<super::parser::Rule>) -> (Vec<String>, Expression) {
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
                        u8::from_str_radix(&inner_pairs.next().unwrap().as_str()[1..], 16).unwrap(),
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
                    | InstructionCode::BVS => {
                        AddressValue::Relative(ShortOperand::Label(address_value.as_str().into()))
                    }
                    _ => AddressValue::Absolute(LongOperand::Label(address_value.as_str().into())),
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

impl Program {
    pub fn from_pairs(pairs: Pairs<Rule>) -> Program {
        let mut expressions: Vec<Rc<Expression>> = Vec::new();
        let mut labels: HashMap<String, (usize, Rc<Expression>)> = HashMap::new();
        let mut cursor: usize = 0;

        for pair in pairs {
            match pair.as_rule() {
                Rule::expression => {
                    let (new_labels, new_expression) = parse_expression(pair);
                    let reference_counted_expression = Rc::new(new_expression);
                    expressions.push(reference_counted_expression.clone());
                    for label in new_labels {
                        labels.insert(label, (cursor, reference_counted_expression.clone()));
                    }
                    cursor += reference_counted_expression.operand.get_size();
                }
                Rule::EOI => {
                    break;
                }
                _ => {
                    panic!("Got unexpected rule: {:?}", pair.as_rule());
                }
            }
        }

        Program {
            expressions: expressions,
            labels: labels,
        }
    }
}

impl Relocatable for Program {
    fn get_raw_section(&self) -> Vec<u8> {
        self.expressions
            .iter()
            .fold((Vec::<u8>::new(), 0), |(mut acc, cursor), expression| {
                let code: u8 = *expression.get_code();
                acc.push(code);
                match expression.operand {
                    AddressValue::Immediate(ref op)
                    | AddressValue::ZeroPage(ref op)
                    | AddressValue::ZeroPageX(ref op)
                    | AddressValue::ZeroPageY(ref op)
                    | AddressValue::IndexedIndirect(ref op)
                    | AddressValue::IndirectIndexed(ref op) => match op {
                        ShortOperand::Numeric(value) => {
                            acc.push(*value);
                        }
                        ShortOperand::Label(_) => {
                            acc.push(0xFF);
                        }
                    },

                    AddressValue::Absolute(ref long_op)
                    | AddressValue::AbsoluteX(ref long_op)
                    | AddressValue::AbsoluteY(ref long_op)
                    | AddressValue::AbsoluteIndirect(ref long_op) => match long_op {
                        LongOperand::Numeric(value) => {
                            acc.push((value & 0xFF) as u8);
                            acc.push((value >> 8) as u8);
                        }
                        LongOperand::Label(_) => {
                            acc.push(0xFF);
                            acc.push(0xFF);
                        }
                    },

                    AddressValue::Relative(ref short_op) => match short_op {
                        ShortOperand::Numeric(value) => {
                            acc.push(*value);
                        }
                        ShortOperand::Label(label) => {
                            // If the label is in the symbol table, calculate the offset
                            // to the currect cursor and insert that value as a u8.
                            // Otherwise, insert 0xFF.
                            if let Some((label_cursor, _)) = self.labels.get(label) {
                                let offset = *label_cursor as i16 - cursor as i16;
                                acc.push(offset as u8);
                            } else {
                                acc.push(0xFF);
                            }
                        }
                    }

                    AddressValue::Accumulator | AddressValue::Implied => {}
                }
                (acc, cursor + expression.operand.get_size())
            })
            .0
    }

    fn get_relocations(&self) -> Vec<Relocation> {
        self.expressions
            .iter()
            .fold(
                (Vec::<Relocation>::new(), 0),
                |(mut acc, cursor), expression| {
                    let current_relocation = cursor + 1;
                    let new_cursor = cursor + expression.operand.get_size();
                    match expression.operand {
                        AddressValue::Immediate(ShortOperand::Label(ref label))
                        | AddressValue::ZeroPage(ShortOperand::Label(ref label))
                        | AddressValue::ZeroPageX(ShortOperand::Label(ref label))
                        | AddressValue::ZeroPageY(ShortOperand::Label(ref label))
                        | AddressValue::IndexedIndirect(ShortOperand::Label(ref label))
                        | AddressValue::IndirectIndexed(ShortOperand::Label(ref label)) => {
                            acc.push(Relocation::Short(label.clone(), current_relocation as u16));
                        }
                        AddressValue::Absolute(LongOperand::Label(ref label))
                        | AddressValue::AbsoluteX(LongOperand::Label(ref label))
                        | AddressValue::AbsoluteY(LongOperand::Label(ref label)) => {
                            acc.push(Relocation::Long(label.clone(), current_relocation as u16));
                        }
                        AddressValue::Relative(ShortOperand::Label(ref label)) => {
                            acc.push(Relocation::Relative(
                                label.clone(),
                                current_relocation as u16,
                            ));
                        }
                        AddressValue::AbsoluteIndirect(LongOperand::Label(ref label)) => {
                            acc.push(Relocation::Absolute(
                                label.clone(),
                                current_relocation as u16,
                            ));
                        }
                        _ => {}
                    }
                    (acc, new_cursor)
                },
            )
            .0
    }

    fn get_symbols(&self) -> HashMap<String, Symbol> {
        self.labels
            .iter()
            .map(|(label, (cursor, _))| (label.clone(), Symbol::Location(*cursor)))
            .collect()
    }
}
