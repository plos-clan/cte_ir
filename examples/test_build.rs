use cte_ir::{builder::{ValueBuilder, BaseInstructionBuilder}, ir::*};

pub fn main() {
    let mut program = Program::new();

    let mut value_builder = program.new_value();

    let one = value_builder.integer(1);
    let two = value_builder.integer(2);

    let add1 = value_builder.add(one, two);
    let _add2 = value_builder.add(add1, one);
    
    let mut evaluter = program.new_evaluter();
    evaluter.evalute();

    println!("{}", program);
}
