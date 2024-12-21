mod binary_op;

pub use binary_op::*;

/// This type is actually an id of the value.
/// The real data of the values is stored in the `ValueData` type.
pub type Value = usize;

/// This type stores the actual value.
/// You can't create a value directly. Please use `ValueBuilder` to create a value.
pub enum ValueData {
    Int(i64),
    BinaryOp(BinaryOpValue),
}
