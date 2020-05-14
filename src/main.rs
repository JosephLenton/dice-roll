#![feature(box_syntax, box_patterns)]

use ::std::fmt;
use ::std::fmt::{Write};
use ::std::env;
use ::pom;

mod expr;
mod parse;
mod eval;

fn main() -> Result<(), fmt::Error> {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let result = run(&input);
    let output = format_result(&"you", &input, result)?;
    println!("{}", output);

    Ok(())
}

fn run(input:&str) -> Result<Vec<Result<eval::Output, eval::Error>>, pom::Error> {
    let mut eval = eval::Eval::new();
    let asts = parse::parse(&input)?;
    let eval_results = asts.into_iter().map(|ast| eval.eval(ast)).collect();

    Ok(eval_results)
}

fn format_result(
    name: &str,
    input: &str,
    result: Result<Vec<Result<eval::Output, eval::Error>>, pom::Error>,
) -> Result<String, fmt::Error> {
    let mut msg = String::new();

    writeln!(&mut msg, "{} rolled ... {}", name, input)?;
    write!(&mut msg, "They got ... ")?;
    match result {
        Ok(outputs) => format_parse_ok(&mut msg, outputs),
        Err(parse_err) => format_parse_err(&mut msg, &parse_err),
    }?;
    writeln!(&mut msg, "")?;

    Ok(msg)
}

fn format_parse_ok(msg:&mut String, outputs: Vec<Result<eval::Output, eval::Error>>) -> fmt::Result {
    for (i, output) in outputs.into_iter().enumerate() {
        if i > 0 {
            write!(msg, " ")?;
        }

        format_output(msg, output)?;
    }

    Ok(())
}

fn format_parse_err(msg: &mut String, err: &pom::Error) -> fmt::Result {
    write!(msg, "error ... {:?}", err)
}

fn format_output(msg:&mut String, output_result: Result<eval::Output, eval::Error>) -> fmt::Result {
    match output_result {
        Ok(output) => format_output_ok(msg, output),
        Err(err) => format_output_err(msg, err),
    }
}

fn format_output_ok(msg: &mut String, output:eval::Output) -> fmt::Result {
    write!(msg, "{}", output)
}

fn format_output_err(msg: &mut String, err:eval::Error) -> fmt::Result {
    write!(msg, "{}", err)
}

impl fmt::Display for eval::Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        Self::Integer(n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for eval::Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivideByZero => write!(f, "divide by zero"),
            Self::NegativePowerNotImplemented => write!(f, "negative powers are not implemented"),
        }
    }
}
