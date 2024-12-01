use lang::parser::pairs::Pairs;
use lang::parser::Rule;

pub struct Label {
    name: String,
    address: u16,
}

pub struct Operation {
    instruction: Instruction,
    address_mode: AddressMode,
    operand: Option<u16>,
}

pub struct Program {
    labels: HashMap<String, u16>,
    operations: Vec<Operation>,
}

/*
 the main algorithm:
 1. using Pest, create the AST as a list of Expression's as defined in the grammar
 2. Set an address counter to 0
 3. For each expression
   3a. If the expression contains one or more labels, add them to the labels map with the current address counter
   3b. For the instruction, determine the address mode from the optional operand
   3c. Fill out the address mode and operand in the Operation struct
*/

impl Program {
    pub fn from_ast(ast: Pairs<'_, Rule>) -> Program {}
}
