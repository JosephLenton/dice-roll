use crate::ast::Expr;
use ::pom::parser::*;
use ::pom;

mod op;
mod space;
mod number;

pub fn parse<'a>(expression : &'a str) -> Result<Vec<Expr>, pom::Error> {
  let bytes = expression.as_bytes();
  let parser = exprs();
  parser.parse(&bytes)
}

fn exprs<'a>() -> Parser<'a, u8, Vec<Expr>> {
  let exprs_list = list(call(expr_0), space::comma());
  space::optional() * exprs_list - end()
}

fn expr_0<'a>() -> Parser<'a, u8, Expr> {
  add_sub() | expr_1()
}

fn add_sub<'a>() -> Parser<'a, u8, Expr> {
  let parser = (expr_1() - space::optional() + op::add_sub() - space::optional()) + call(expr_0);
  parser.map(|((left, op), right)| Expr::Operator(op, box left, box right))
}

fn expr_1<'a>() -> Parser<'a, u8, Expr> {
  mult_div() | expr_2()
}

fn mult_div<'a>() -> Parser<'a, u8, Expr> {
  let parser = (expr_2() - space::optional() + op::mult_div() - space::optional()) + call(expr_1);
  parser.map(|((left, op), right)| Expr::Operator(op, box left, box right))
}

fn expr_2<'a>() -> Parser<'a, u8, Expr> {
  pow() | expr_3()
}

fn pow<'a>() -> Parser<'a, u8, Expr> {
  let parser = (expr_3() - space::optional() + op::power() - space::optional()) + call(expr_2);
  parser.map(|((left, op), right)| Expr::Operator(op, box left, box right))
}

fn expr_3<'a>() -> Parser<'a, u8, Expr> {
  roll() | expr_4()
}

fn roll<'a>() -> Parser<'a, u8, Expr> {
  let parser = (expr_4().opt() + op::roll() - space::optional()) + call(expr_3);
  parser.map(|((maybe_left, op), right)| {
    let left = maybe_left.unwrap_or(Expr::Integer(1));
    Expr::Operator(op, box left, box right)
  })
}

fn expr_4<'a>() -> Parser<'a, u8, Expr> {
  expr_with_brackets() | number::number()
}

fn expr_with_brackets<'a>() -> Parser<'a, u8, Expr> {
  sym(b'(') * space::optional() * call(expr_0) - space::optional() - sym(b')')
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::ast::ExprOp;

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

  #[test]
  fn it_should_handle_one_dice_roll_with_num_dice_omitted() {
    test_multiple(&"d6",
      vec![
        Expr::Operator( ExprOp::Roll,
          Box::new(Expr::Integer(1)),
          Box::new(Expr::Integer(6)),
        ),
      ]
    );
  }

  #[test]
  fn it_should_handle_multiple_dice_rolls_with_num_dice_omitted() {
    test_multiple(&"d6 d6",
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
