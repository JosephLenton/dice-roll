#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  Integer(i32),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Mult(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),
  Pow(Box<Expr>, Box<Expr>),
  Roll(Box<Expr>, Box<Expr>),
}
