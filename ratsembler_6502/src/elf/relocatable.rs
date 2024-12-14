use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
pub enum Symbol {
    Location(usize),
    ShortValue(u8),
    LongValue(u16),
}

#[derive(Debug)]
pub enum Relocation {
    Absolute(String, u16),
    Relative(String, u16),
    Short(String, u16),
    Long(String, u16),
}

pub trait Relocatable {
    fn get_raw_section(&self) -> Vec<u8>;
    fn get_relocations(&self) -> Vec<Relocation>;
    fn get_symbols(&self) -> HashMap<String, Symbol>;
}
