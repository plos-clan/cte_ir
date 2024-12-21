# CTE IR

CTE ( compile-time evalution ) IR is a high-level IR. In this IR, all the types are values. It is designed to support compile-time evaluation.

## Example

### IR

``` cte_ir
%0 = add 1,2;
```

### Builder

This code is included in [this file](examples/test_build.rs).
You can also see other examples in the examples directory.

``` rust
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
```
