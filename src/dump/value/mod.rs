use crate::ir::*;

mod binary_op;

impl Program {
    pub fn dump_value(&self, f: &mut std::fmt::Formatter<'_>, value: Value) -> std::fmt::Result {
        if self.instructions().is_instruction(value) {
            write!(f, "{}", value)
        } else {
            self.dump_value_depth_one(f, value)
        }
    }
    
    pub fn dump_value_depth_one(&self, f: &mut std::fmt::Formatter<'_>, value: Value) -> std::fmt::Result {
        let value = self.get_value(value);

        match value {
            ValueData::Int(int) => write!(f, "{}", int),
            ValueData::BinaryOp(binary_op) => self.dump_binary_op(f, *binary_op),
        }
    }
}
