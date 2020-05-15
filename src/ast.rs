#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Integer(i64),
    Operator(ExprOp, Box<Expr>, Box<Expr>),
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
