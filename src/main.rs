#![feature(box_syntax, box_patterns)]

use ::std::env;
use ::pom;

mod expr;
mod parse;
mod eval;

fn main() {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let mut eval = eval::Eval::new();
    let result : Result<Vec<eval::Output>, pom::Error> = parse::parse(&input).map(|asts| asts.into_iter().map(|ast| eval.eval(ast)).collect());
    println!("{:?}", result);
}
