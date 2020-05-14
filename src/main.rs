#![feature(box_syntax, box_patterns)]

use ::std::fmt;
use ::std::env;
use ::pom;

mod expr;
mod parse;
mod eval;

fn main() -> Result<(), fmt::Error> {
    let name = "You";
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let result = run(&input);

    println!("{} rolled ... {}\nThey got ... {}", name, input, ResultOutput(result));

    Ok(())
}

fn run(input:&str) -> Result<Vec<Result<eval::Output, eval::Error>>, pom::Error> {
    let mut eval = eval::Eval::new();
    let asts = parse::parse(&input)?;
    let eval_results = asts.into_iter().map(|ast| eval.eval(ast)).collect();

    Ok(eval_results)
}

struct ResultOutput(Result<Vec<Result<eval::Output, eval::Error>>, pom::Error>);

impl fmt::Display for ResultOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultOutput(Ok(outputs)) => format_parse_ok(f, &outputs),
            ResultOutput(Err(parse_err)) => format_parse_err(f, &parse_err),
        }
    }
}

fn format_parse_ok(f:&mut fmt::Formatter<'_>, outputs: &Vec<Result<eval::Output, eval::Error>>) -> fmt::Result {
    for (i, output) in outputs.iter().enumerate() {
        if i > 0 {
            write!(f, " ")?;
        }

        format_output(f, output)?;
    }

    Ok(())
}

fn format_parse_err(f: &mut fmt::Formatter<'_>, err: &pom::Error) -> fmt::Result {
    write!(f, "error ... {:?}", err)
}

fn format_output(f:&mut fmt::Formatter<'_>, output_result: &Result<eval::Output, eval::Error>) -> fmt::Result {
    match output_result {
        Ok(output) => format_output_ok(f, output),
        Err(err) => format_output_err(f, err),
    }
}

fn format_output_ok(f: &mut fmt::Formatter<'_>, output:&eval::Output) -> fmt::Result {
    write!(f, "{}", output)
}

fn format_output_err(f: &mut fmt::Formatter<'_>, err:&eval::Error) -> fmt::Result {
    write!(f, "{}", err)
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
