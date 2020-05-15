use crate::ast::ExprOp;
use ::pom::parser::*;

pub fn mult_div<'a>() -> Parser<'a, u8, ExprOp> {
    mult() | div()
}

pub fn add_sub<'a>() -> Parser<'a, u8, ExprOp> {
    add() | sub()
}

pub fn add<'a>() -> Parser<'a, u8, ExprOp> {
    sym(b'+').discard().map(|_| ExprOp::Add)
}

pub fn sub<'a>() -> Parser<'a, u8, ExprOp> {
    sym(b'-').discard().map(|_| ExprOp::Sub)
}

pub fn mult<'a>() -> Parser<'a, u8, ExprOp> {
    sym(b'*').discard().map(|_| ExprOp::Mult)
}

pub fn div<'a>() -> Parser<'a, u8, ExprOp> {
    sym(b'/').discard().map(|_| ExprOp::Div)
}

pub fn power<'a>() -> Parser<'a, u8, ExprOp> {
    sym(b'^').discard().map(|_| ExprOp::Pow)
}

pub fn roll<'a>() -> Parser<'a, u8, ExprOp> {
    sym(b'd').discard().map(|_| ExprOp::Roll)
}
