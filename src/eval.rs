
use crate::expr::Expr;

use ::rand::{RngCore};
use ::rand::rngs::{OsRng};
use ::rand::Rng;
use ::rand::SeedableRng;
use ::rand::rngs::StdRng;

#[derive(Clone, Debug, PartialEq)]
pub struct Output {
  total: i32
}

impl Output {
  fn new(total:i32) -> Self {
    Self {
      total,
    }
  }
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

  pub fn eval(&mut self, ast:Expr) -> Output {
    let total = self.walk(ast);
    Output::new(total)
  }

  fn walk(&mut self, ast:Expr) -> i32 {
    match ast {
      Expr::Integer(n) => n,
      Expr::Add(box left, box right) => self.walk(left) + self.walk(right),
      Expr::Sub(box left, box right) => self.walk(left) - self.walk(right),
      Expr::Mult(box left, box right) => self.walk(left) * self.walk(right),
      Expr::Div(box left, box right) => self.walk(left) / self.walk(right),
      Expr::Pow(box left, box right) => self.walk(left).pow(self.walk(right) as u32),
      Expr::Roll(box left, box right) => {
        let num_die = self.walk(left);
        let num_sides = self.walk(right);
        self.roll(num_die, num_sides)
      },
    }
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
