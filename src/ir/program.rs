use std::collections::BTreeMap;

use super::{IdManager, InstructionList, Value, ValueData};

pub struct Program {
    pub(crate) instructions: InstructionList,
    pub(crate) values: BTreeMap<Value, ValueData>,
    pub(crate) value_id_manager: IdManager<Value>,
}

impl Program {
    /// Get the value by the id of the value.
    pub fn get_value(&self, id: Value) -> &ValueData {
        &self.values[&id]
    }
}
