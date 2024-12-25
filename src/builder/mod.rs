use crate::ir::*;

mod program;

/// This traits is used to build values.
pub trait ValueBuilder {
    fn insert_value(&mut self, value: ValueData) -> Value;

    /// Create a new integer value.
    fn integer(&mut self, int: i64) -> Value {
        self.insert_value(ValueData::Int(int))
    }
}

pub trait BaseInstructionBuilder: ValueBuilder {
    fn insert_instruction(&mut self, value: Value) -> Value;
    
    /// Create a new add value.
    fn add(&mut self, lhs: Value, rhs: Value) -> Value {
        let value = self.insert_value(ValueData::BinaryOp(BinaryOpValue {
            lhs,
            op: BinaryOp::Add,
            rhs,
        }));
        self.insert_instruction(value)
    }

    /// Create a new sub value.
    fn sub(&mut self, lhs: Value, rhs: Value) -> Value {
        let value = self.insert_value(ValueData::BinaryOp(BinaryOpValue {
            lhs,
            op: BinaryOp::Sub,
            rhs,
        }));
        self.insert_instruction(value)
    }

    /// Create a new mul value.
    fn mul(&mut self, lhs: Value, rhs: Value) -> Value {
        let value = self.insert_value(ValueData::BinaryOp(BinaryOpValue {
            lhs,
            op: BinaryOp::Mul,
            rhs,
        }));
        self.insert_instruction(value)
    }

    /// Create a new div value.
    fn div(&mut self, lhs: Value, rhs: Value) -> Value {
        let value = self.insert_value(ValueData::BinaryOp(BinaryOpValue {
            lhs,
            op: BinaryOp::Div,
            rhs,
        }));
        self.insert_instruction(value)
    }
}
