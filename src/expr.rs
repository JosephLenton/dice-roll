#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  Integer(i32),
  Operator(ExprOp, Box<Expr>, Box<Expr>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprOp {
  Add,
  Sub,
  Mult,
  Div,
  Pow,
  Roll,
}
