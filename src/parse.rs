use crate::expr::{Expr, ExprOp};
use ::pom::parser::*;
use ::pom;
use ::std::str::{self, FromStr};

enum AddSubOp {
  Add,
  Sub,
}

enum MultDivOp {
  Mult,
  Div,
}

pub fn parse<'a>(expression : &'a str) -> Result<Vec<Expr>, pom::Error> {
  let bytes = expression.as_bytes();
  let parser = exprs();
  parser.parse(&bytes)
}

fn exprs<'a>() -> Parser<'a, u8, Vec<Expr>> {
  let exprs_list = list(call(expr_0), space_or_comma());
  space() * exprs_list - end()
}

fn expr_0<'a>() -> Parser<'a, u8, Expr> {
  add_sub() | expr_1()
}

fn add_sub<'a>() -> Parser<'a, u8, Expr> {
  let parser = (expr_1() - space() + add_sub_op() - space()).repeat(1..) + expr_1();

  parser.map(|(add_subs, right)| {
    add_subs.into_iter().rev().fold(right, |right, (left, op)| {
      match op {
        AddSubOp::Add => Expr::Operator(ExprOp::Add, box left, box right),
        AddSubOp::Sub => Expr::Operator(ExprOp::Sub, box left, box right),
      }
    })
  })
}

fn expr_1<'a>() -> Parser<'a, u8, Expr> {
  mult_div() | expr_2()
}

fn add_sub_op<'a>() -> Parser<'a, u8, AddSubOp> {
  sym(b'+').discard().map(|_| AddSubOp::Add) | sym(b'-').discard().map(|_| AddSubOp::Sub)
}

fn mult_div<'a>() -> Parser<'a, u8, Expr> {
  let parser = (expr_2() - space() + mult_div_op() - space()).repeat(1..) + expr_2();

  parser.map(|(mult_divs, right)| {
    mult_divs.into_iter().rev().fold(right, |right, (left, op)| {
      match op {
        MultDivOp::Mult => Expr::Operator(ExprOp::Mult, box left, box right),
        MultDivOp::Div => Expr::Operator(ExprOp::Div, box left, box right),
      }
    })
  })
}

fn mult_div_op<'a>() -> Parser<'a, u8, MultDivOp> {
  sym(b'*').discard().map(|_| MultDivOp::Mult) | sym(b'/').discard().map(|_| MultDivOp::Div)
}

fn expr_2<'a>() -> Parser<'a, u8, Expr> {
  pow() | expr_3()
}

fn pow<'a>() -> Parser<'a, u8, Expr> {
  let parser = (expr_3() - space() - sym(b'^') - space()).repeat(1..) + expr_3();

  parser.map(|(pows, right)| {
    pows.into_iter().rev().fold(right, |right, left| {
      Expr::Operator(ExprOp::Pow, box left, box right)
    })
  })
}

fn expr_3<'a>() -> Parser<'a, u8, Expr> {
  roll() | expr_4()
}

fn roll<'a>() -> Parser<'a, u8, Expr> {
  let num_dice_parser = expr_4().opt().map(|maybe_num| maybe_num.unwrap_or(Expr::Integer(1)));
  let parser = (num_dice_parser - sym(b'd')).repeat(1..) + expr_4();

  parser.map(|(rolls, right)| {
    rolls.into_iter().rev().fold(right, |right, left| {
      Expr::Operator(ExprOp::Roll, box left, box right)
    })
  })
}

fn expr_4<'a>() -> Parser<'a, u8, Expr> {
  expr_with_brackets() | number()
}

fn expr_with_brackets<'a>() -> Parser<'a, u8, Expr> {
  sym(b'(') * space() * call(expr_0) - space() - sym(b')')
}

fn number<'a>() -> Parser<'a, u8, Expr> {
  integer() // | float()
}

fn integer<'a>() -> Parser<'a, u8, Expr> {
  let parser = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
  parser.collect().convert(str::from_utf8).convert(|s| i32::from_str(&s)).map(|n| Expr::Integer(n))
}

/*
fn float() -> Parser<u8, Expr> {
  let parser = sym(b"0") | one_of(b"123456789") + one_of(b"0123456789").repeat(0..);
  parser.collect().convert(str::from_utf8).convert(|s| i32::from_str(&s)).map(|n| Expr::Integer(n))
}
*/

fn space<'a>() -> Parser<'a, u8, ()> {
	one_of(b" \t\r\n").repeat(0..).discard()
}

fn required_space<'a>() -> Parser<'a, u8, ()> {
	one_of(b" \t\r\n").repeat(1..).discard()
}

fn space_or_comma<'a>() -> Parser<'a, u8, ()> {
  ((space() - sym(b',') - space()) | required_space()).discard()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_should_parse_number_zero() {
    test_single(&"0", Expr::Integer(0));
  }

  #[test]
  fn it_should_parse_number() {
    test_single(&"1", Expr::Integer(1));
    test_single(&"123", Expr::Integer(123));
    test_single(&"900", Expr::Integer(900));
    test_single(&"6594", Expr::Integer(6594));
  }

  #[test]
  fn it_should_add_two_numbers_with_no_spaces() {
    test_single(&"1+2", Expr::Operator( ExprOp::Add,
      Box::new(Expr::Integer(1)),
      Box::new(Expr::Integer(2)),
    ))
  }

  #[test]
  fn it_should_add_two_numbers_with_spaces() {
    test_single(&"1 + 2", Expr::Operator( ExprOp::Add,
      Box::new(Expr::Integer(1)),
      Box::new(Expr::Integer(2)),
    ))
  }

  #[test]
  fn it_should_add_three_numbers_with_spaces() {
    test_single(&"1 + 2 + 3",
      Expr::Operator( ExprOp::Add,
        Box::new(Expr::Integer(1)),
        Box::new(Expr::Operator( ExprOp::Add,
          Box::new(Expr::Integer(2)),
          Box::new(Expr::Integer(3)),
        ))
      )
    )
  }

  #[test]
  fn it_should_add_and_subtract_lots_of_numbers_with_spaces() {
    test_single(&"1 + 2 - 3 + 4 - 5 - 6 + 7 + 8 - 9",
      Expr::Operator( ExprOp::Add,
        Box::new(Expr::Integer(1)),
        Box::new(Expr::Operator( ExprOp::Sub,
          Box::new(Expr::Integer(2)),
          Box::new(Expr::Operator( ExprOp::Add,
            Box::new(Expr::Integer(3)),
            Box::new(Expr::Operator( ExprOp::Sub,
              Box::new(Expr::Integer(4)),
              Box::new(Expr::Operator( ExprOp::Sub,
                Box::new(Expr::Integer(5)),
                Box::new(Expr::Operator( ExprOp::Add,
                  Box::new(Expr::Integer(6)),
                  Box::new(Expr::Operator( ExprOp::Add,
                    Box::new(Expr::Integer(7)),
                    Box::new(Expr::Operator( ExprOp::Sub,
                      Box::new(Expr::Integer(8)),
                      Box::new(Expr::Integer(9)),
                    ))
                  ))
                ))
              ))
            ))
          ))
        ))
      )
    )
  }

  #[test]
  fn it_should_add_handle_operator_precedence_with_mult_and_add() {
    test_single(&"1 + 2 * 3",
      Expr::Operator( ExprOp::Add,
        Box::new(Expr::Integer(1)),
        Box::new(Expr::Operator( ExprOp::Mult,
          Box::new(Expr::Integer(2)),
          Box::new(Expr::Integer(3)),
        )),
      )
    );

    test_single(&"1 * 2 + 3",
      Expr::Operator( ExprOp::Add,
        Box::new(Expr::Operator( ExprOp::Mult,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Integer(2)),
        )),
        Box::new(Expr::Integer(3)),
      )
    );
  }

  #[test]
  fn it_should_add_handle_operator_precedence_with_lots_of_numbers() {
    test_single(&"1 + 2 * 3 * 4 + 5 * 6 + 7",
      Expr::Operator( ExprOp::Add,
        Box::new(Expr::Integer(1)),
        Box::new(Expr::Operator( ExprOp::Add,
          Box::new(Expr::Operator( ExprOp::Mult,
            Box::new(Expr::Integer(2)),
            Box::new(Expr::Operator( ExprOp::Mult,
              Box::new(Expr::Integer(3)),
              Box::new(Expr::Integer(4)),
            )),
          )),
          Box::new(Expr::Operator( ExprOp::Add,
            Box::new(Expr::Operator( ExprOp::Mult,
              Box::new(Expr::Integer(5)),
              Box::new(Expr::Integer(6)),
            )),
            Box::new(Expr::Integer(7)),
          )),
        )),
      )
    )
  }

  #[test]
  fn it_should_handle_brackets() {
    test_single(&"(1 + 2) * 3 * (4 + 5)",
      Expr::Operator( ExprOp::Mult,
        Box::new(Expr::Operator( ExprOp::Add,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Integer(2)),
        )),
        Box::new(Expr::Operator( ExprOp::Mult,
          Box::new(Expr::Integer(3)),
          Box::new(Expr::Operator( ExprOp::Add,
            Box::new(Expr::Integer(4)),
            Box::new(Expr::Integer(5)),
          )),
        )),
      ),
    )
  }

  #[test]
  fn it_should_handle_multiple_expressions() {
    test_multiple(&"1 + 2 * 3 1 * 2 + 3",
      vec![
        Expr::Operator( ExprOp::Add,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Operator( ExprOp::Mult,
            Box::new(Expr::Integer(2)),
            Box::new(Expr::Integer(3)),
          )),
        ),
        Expr::Operator( ExprOp::Add,
          Box::new(Expr::Operator( ExprOp::Mult,
            Box::new(Expr::Integer(1)),
            Box::new(Expr::Integer(2)),
          )),
          Box::new(Expr::Integer(3)),
        ),
      ]
    );
  }

  #[test]
  fn it_should_handle_multiple_die_rolls() {
    test_multiple(&"1d6 1d6",
      vec![
        Expr::Operator( ExprOp::Roll,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Integer(6)),
        ),
        Expr::Operator( ExprOp::Roll,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Integer(6)),
        ),
      ]
    );
  }

  #[test]
  fn it_should_handle_multiple_die_rolls_with_commas_no_spaces() {
    test_multiple(&"1d6,1d6",
      vec![
        Expr::Operator( ExprOp::Roll,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Integer(6)),
        ),
        Expr::Operator( ExprOp::Roll,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Integer(6)),
        ),
      ]
    );
  }

  #[test]
  fn it_should_handle_multiple_die_rolls_with_commas_and_spaces() {
    test_multiple(&"1d6 1d6",
      vec![
        Expr::Operator( ExprOp::Roll,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Integer(6)),
        ),
        Expr::Operator( ExprOp::Roll,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Integer(6)),
        ),
      ]
    );
  }

  fn test_single(
    expr : &str,
    expected : Expr,
  ) {
    test_multiple(expr, vec![expected])
  }

  fn test_multiple(
    expr : &str,
    expected : Vec<Expr>,
  ) {
    assert_eq!(parse(expr), Ok(expected))
  }
}
