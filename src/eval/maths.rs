use ::rand::Rng;

use crate::eval::Error;

pub fn add(left: i64, right: i64) -> Result<i64, Error> {
  Ok(left + right)
}

pub fn sub(left: i64, right: i64) -> Result<i64, Error> {
  Ok(left - right)
}

pub fn mult(left: i64, right: i64) -> Result<i64, Error> {
  Ok(left * right)
}

pub fn divide(left: i64, right: i64) -> Result<i64, Error> {
  if right == 0 {
    return Err(Error::DivideByZero)
  }

  Ok(left / right)
}

pub fn power(left : i64, right: i64) -> Result<i64, Error> {
  if right < 0 {
    return Err(Error::NegativePowerNotImplemented)
  }

  Ok(left.pow(right as u32))
}

pub fn roll(rng: &mut impl Rng, num_die : i64, num_sides: i64) -> Result<i64, Error> {
  let mut total = 0;

  for _ in 0..num_die {
    total += roll_one(rng, num_sides);
  }

  Ok(total)
}

fn roll_one(rng: &mut impl Rng, num_sides: i64) -> i64 {
  if num_sides == 0 {
    0
  } else if num_sides < 0 {
    (-rng.gen_range(0, -num_sides)) - 1
  } else {
    rng.gen_range(0, num_sides) + 1
  }
}
