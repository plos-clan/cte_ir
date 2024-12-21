use crate::ir::*;

impl Program {
    pub fn dump_binary_op(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        binary_op: BinaryOpValue,
    ) -> Result<(), std::fmt::Error> {
        let BinaryOpValue { lhs, op, rhs } = binary_op;

        match op {
            BinaryOp::Add => write!(f, "add"),
            BinaryOp::Sub => write!(f, "sub"),
            BinaryOp::Mul => write!(f, "mul"),
            BinaryOp::Div => write!(f, "div"),
        }?;
        
        write!(f, " ")?;
        
        self.dump_value(f, lhs)?;
        write!(f, ", ")?;
        self.dump_value(f, rhs)
    }
}
