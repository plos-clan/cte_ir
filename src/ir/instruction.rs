use std::ops::{Deref, DerefMut};

use super::Value;

/// This type stores the actual instruction.
pub struct InstructionList {
    pub(crate) instructions: Vec<Value>,
}

impl InstructionList {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    /// Add an instruction at the end of the list.
    /// If the name is `None`, the instruction will be named with the id.
    pub fn push_back(&mut self, value: Value) -> Value {
        self.instructions.push(value);
        value
    }

    /// Create an iterator of the instructions.
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.instructions.iter()
    }
}

impl Deref for InstructionList {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.instructions
    }
}

impl DerefMut for InstructionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instructions
    }
}

impl InstructionList {
    /// Return true if the value is an instruction
    pub fn is_instruction(&self, value: Value) -> bool {
        self.instructions.iter().find(|x| **x == value).is_some()
    }
}

pub struct InstructionData {
    pub name: Option<String>,
    pub value: Value,
}
