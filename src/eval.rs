
use crate::expr::{Expr, ExprOp};

use ::rand::{RngCore};
use ::rand::rngs::{OsRng};
use ::rand::Rng;
use ::rand::SeedableRng;
use ::rand::rngs::StdRng;

#[derive(Clone, Debug, PartialEq)]
pub enum Output {
  Integer(i32)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
  DivideByZero,
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

  pub fn eval(&mut self, ast:Expr) -> Result<Output, Error> {
    self.visit(ast).map(|total| Output::Integer(total))
  }

  fn visit(&mut self, ast:Expr) -> Result<i32, Error> {
    match ast {
      Expr::Integer(n) => Ok(n),
      Expr::Operator(op, box left_expr, box right_expr) => self.visit_op(op, left_expr, right_expr),
    }
  }

  fn visit_op(&mut self, op:ExprOp, left_expr: Expr, right_expr: Expr) -> Result<i32, Error> {
    let left = self.visit(left_expr)?;
    let right = self.visit(right_expr)?;

    Ok(match op {
      ExprOp::Add => left + right,
      ExprOp::Sub => left - right,
      ExprOp::Mult => left * right,
      ExprOp::Div => self.divide(left, right)?,
      ExprOp::Pow => left.pow(right as u32),
      ExprOp::Roll => self.roll(left, right),
    })
  }

  fn divide(&mut self, left: i32, right: i32) -> Result<i32, Error> {
    if right == 0 {
      return Err(Error::DivideByZero)
    }

    Ok(left / right)
  }

  fn roll(&mut self, num_die : i32, num_sides: i32) -> i32 {
    let mut total = 0;

    for _ in 0..num_die {
      let roll = self.roll_one(num_sides);
      total += roll;
    }

    total
  }

  fn roll_one(&mut self, num_sides: i32) -> i32 {
    if num_sides == 0 {
      0
    } else if num_sides < 0 {
      (-self.rng.gen_range(0, -num_sides)) - 1
    } else {
      self.rng.gen_range(0, num_sides) + 1
    }
  }
}
