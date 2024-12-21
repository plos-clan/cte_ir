use cte_ir::{builder::ValueBuilder, ir::*};

pub fn main() {
    let mut program = Program::new();

    let mut value_builder = program.new_value();

    let one = value_builder.integer(1);
    let two = value_builder.integer(2);

    let add = value_builder.add(one, two);

    program.instructions_mut().push_back(None, add);

    println!("{}", program);
}
