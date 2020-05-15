#![feature(box_syntax, box_patterns)]

use ::std::io;

mod ast;
mod eval;
mod format;
mod parse;

pub fn main(username: &str, input: &str, output: &mut impl io::Write) -> io::Result<()> {
    let mut eval = eval::Eval::new();
    let result = parse::parse(&input)
        .map(|asts| return asts.into_iter().map(|ast| eval.eval(ast)).collect());

    format::fmt(output, &username, &input, &result)
}
