mod binary_op;

use std::fmt::{Debug, Display};

pub use binary_op::*;

/// This type is actually an id of the value.
/// The real data of the values is stored in the `ValueData` type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(usize);

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// This type stores the actual value.
/// You can't create a value directly. Please use `ValueBuilder` to create a value.
pub enum ValueData {
    Int(i64),
    BinaryOp(BinaryOpValue),
}
