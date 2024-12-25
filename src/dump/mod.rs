use std::fmt::Display;

use crate::ir::*;

mod value;

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.instructions.iter() {
            write!(f, "{}", value)?;
            write!(f, " = ")?;
            self.dump_value_depth_one(f, *value)?;
            writeln!(f, ";")?;
        }

        Ok(())
    }
}
