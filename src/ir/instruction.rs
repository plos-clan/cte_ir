use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use super::{IdManager, Value};

/// This type is actually an id of the instruction.
pub type Instruction = usize;

/// This type stores the actual instruction.
pub struct InstructionList {
    pub(crate) instructions: BTreeMap<Instruction, InstructionData>,
    pub(crate) id_manger: IdManager<Instruction>,
}

impl InstructionList {
    pub fn new() -> Self {
        Self {
            instructions: BTreeMap::new(),
            id_manger: IdManager::new(),
        }
    }

    fn get_id(&mut self) -> Instruction {
        self.id_manger.next()
    }

    /// Add an instruction at the end of the list.
    /// If the name is `None`, the instruction will be named with the id.
    pub fn push_back(&mut self, name: Option<String>, value: Value) -> Instruction {
        let id = self.get_id();
        self.instructions
            .insert(id, InstructionData { name, value });
        id
    }

    /// Create an iterator of the instructions.
    pub fn iter(&self) -> impl Iterator<Item = (&Instruction, &InstructionData)> {
        self.instructions.iter()
    }
}

impl Deref for InstructionList {
    type Target = BTreeMap<Instruction, InstructionData>;

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
    /// Get the instruction by the id of the instruction.
    pub fn get_instruction(&self, id: Instruction) -> &InstructionData {
        &self.instructions[&id]
    }
}

pub struct InstructionData {
    pub name: Option<String>,
    pub value: Value,
}
