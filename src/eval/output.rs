
use ::std::fmt::{Display, Result, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Output {
  Integer(i64)
}

impl Display for Output {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Integer(n) => write!(f, "{}", n),
    }
  }
}
