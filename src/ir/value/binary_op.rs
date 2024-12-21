use std::fmt::Display;

use super::Value;

#[derive(Debug, Clone, Copy)]
pub struct BinaryOpValue {
    pub(crate) lhs: Value,
    pub(crate) op: BinaryOp,
    pub(crate) rhs: Value,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}
