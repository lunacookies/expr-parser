use super::{Expr, Number, Operation, Root};
use crate::Op;

impl Number {
    fn eval(&self) -> u32 {
        self.text().parse().unwrap()
    }
}

impl Operation {
    fn eval(&self) -> Option<u32> {
        let lhs = self.lhs()?.eval()?;
        let rhs = self.rhs()?.eval()?;

        let op = match self.op()?.into() {
            Op::Add => std::ops::Add::add,
            Op::Sub => std::ops::Sub::sub,
            Op::Mul => std::ops::Mul::mul,
            Op::Div => std::ops::Div::div,
        };

        Some(op(lhs, rhs))
    }
}

impl Expr {
    fn eval(&self) -> Option<u32> {
        match self {
            Self::Number(n) => Some(n.eval()),
            Self::Operation(o) => o.eval(),
        }
    }
}

impl Root {
    pub(crate) fn eval(&self) -> Option<u32> {
        // Roots are expected to include only one child, with that child being an Expr.
        let expr = Expr::cast(self.0.children_with_tokens().next()?)?;
        expr.eval()
    }
}
