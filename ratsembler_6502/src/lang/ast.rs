
use super::parser::Rule;
use super::expression::Expression;
use super::expression::AddressValue;
use super::expression::ShortOperand;
use super::expression::LongOperand;

use crate::elf::relocatable::Relocatable;
use crate::elf::relocatable::Relocation;
use crate::elf::relocatable::Symbol;

use pest::iterators::Pairs;

use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;


#[derive(Debug)]
pub struct Program {
    pub expressions: Vec<Rc<Expression>>,
    pub labels: HashMap<String, (usize, Rc<Expression>)>,
}


impl Program {
    pub fn from_pairs(pairs: Pairs<Rule>) -> Program {
        let mut expressions: Vec<Rc<Expression>> = Vec::new();
        let mut labels: HashMap<String, (usize, Rc<Expression>)> = HashMap::new();
        let mut cursor: usize = 0;

        for pair in pairs {
            match pair.as_rule() {
                Rule::expression => {
                    let (new_labels, new_expression) = Expression::from_expression_pair(pair);
                    let reference_counted_expression = Rc::new(new_expression);
                    expressions.push(reference_counted_expression.clone());
                    for label in new_labels {
                        labels.insert(label, (cursor, reference_counted_expression.clone()));
                    }
                    cursor += reference_counted_expression.get_size();
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
