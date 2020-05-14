
use ::std::fmt::{Display, Result, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
  DivideByZero,

  // todo, add float support and then add negative power support.
  NegativePowerNotImplemented,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::DivideByZero => write!(f, "divide by zero"),
      Self::NegativePowerNotImplemented => write!(f, "negative powers are not implemented"),
    }
  }
}
