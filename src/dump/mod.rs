use std::fmt::Display;

use crate::ir::*;

mod value;

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, instruction) in self.instructions.iter() {
            let InstructionData { name, value } = instruction;
            if let Some(name) = name {
                write!(f, "%{}", name)?;
            } else {
                write!(f, "%{}", id)?;
            }
            write!(f, " = ")?;
            self.dump_value(f, *value)?;
            write!(f, ";")?;
        }

        Ok(())
    }
}
