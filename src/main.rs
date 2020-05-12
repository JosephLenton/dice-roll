#![feature(box_syntax, box_patterns)]

use ::std::env;
use ::pom;

mod expr;
mod parse;
mod eval;

fn main() {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let result = run(&input);
    println!("{:?}", result);
}

fn run(input:&str) -> Result<Vec<Result<eval::Output, eval::Error>>, pom::Error> {
    let mut eval = eval::Eval::new();
    let asts = parse::parse(&input)?;
    let eval_results = asts.into_iter().map(|ast| eval.eval(ast)).collect();

    Ok(eval_results)
}
