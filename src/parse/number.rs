use ::pom::parser::*;
use ::std::str::{self, FromStr};

use crate::expr::Expr;

pub fn number<'a>() -> Parser<'a, u8, Expr> {
  integer()
}

fn integer<'a>() -> Parser<'a, u8, Expr> {
  let unsigned_parser = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
  let signed_parser = sym(b'-').opt() * unsigned_parser;
  signed_parser.collect().convert(str::from_utf8).convert(|s| i64::from_str(&s)).map(|n| Expr::Integer(n))
}
