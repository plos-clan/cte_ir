use std::collections::BTreeMap;

use super::*;

impl Program {
    pub fn new() -> Self {
        Self {
            instructions: InstructionList::new(),
            values: BTreeMap::new(),
            value_id_manager: IdManager::new(),
        }
    }
    
    /// Get the immutable reference of the instructions.
    pub fn instructions(&self) -> &InstructionList {
        &self.instructions
    }

    /// Get the mutable reference of the instructions.
    pub fn instructions_mut(&mut self) -> &mut InstructionList {
        &mut self.instructions
    }

    /// Create a new global builder.
    pub fn new_value(&mut self) -> GlobalBuilder {
        GlobalBuilder { program: self }
    }
}

pub struct GlobalBuilder<'a> {
    program: &'a mut Program,
}

impl ValueBuilder for GlobalBuilder<'_> {
    fn insert_value(&mut self, value: ValueData) -> Value {
        let id = self.program.value_id_manager.next();
        self.program.values.insert(id, value);
        id
    }
}

impl BaseInstructionBuilder for GlobalBuilder<'_> {
    fn insert_instruction(&mut self, value: Value) -> Value {
        self.program.instructions_mut().push(value);
        value
    }
}
