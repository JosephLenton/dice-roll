use crate::ast::{Expr, ExprOp};

use ::rand::rngs::OsRng;
use ::rand::rngs::StdRng;
use ::rand::RngCore;
use ::rand::SeedableRng;

mod maths;

#[derive(Clone, Debug, PartialEq)]
pub enum Output {
    Integer(i64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    DivideByZero,

    // todo, add float support and then add negative power support.
    NegativePowerNotImplemented,
}

pub struct Eval {
    rng: StdRng,
}

impl Eval {
    pub fn new() -> Self {
        Self::new_with_seed(OsRng.next_u64())
    }

    pub fn new_with_seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn eval(&mut self, ast: Expr) -> Result<Output, Error> {
        self.visit(ast).map(|total| Output::Integer(total))
    }

    fn visit(&mut self, ast: Expr) -> Result<i64, Error> {
        match ast {
            Expr::Integer(n) => Ok(n),
            Expr::Operator(op, box left_expr, box right_expr) => {
                self.visit_op(op, left_expr, right_expr)
            }
        }
    }

    fn visit_op(&mut self, op: ExprOp, left_expr: Expr, right_expr: Expr) -> Result<i64, Error> {
        let left = self.visit(left_expr)?;
        let right = self.visit(right_expr)?;

        Ok(match op {
            ExprOp::Add => maths::add(left, right)?,
            ExprOp::Sub => maths::sub(left, right)?,
            ExprOp::Mult => maths::mult(left, right)?,
            ExprOp::Div => maths::divide(left, right)?,
            ExprOp::Pow => maths::power(left, right)?,
            ExprOp::Roll => maths::roll(&mut self.rng, left, right)?,
        })
    }
}
