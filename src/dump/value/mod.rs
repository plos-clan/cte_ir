use crate::ir::*;

mod binary_op;

impl Program {
    pub fn dump_value(&self, f: &mut std::fmt::Formatter<'_>, value: Value) -> std::fmt::Result {
        let value = self.get_value(value);

        match value {
            ValueData::Int(int) => write!(f, "{}", int),
            ValueData::BinaryOp(binary_op) => self.dump_binary_op(f, *binary_op),
        }
    }
}
