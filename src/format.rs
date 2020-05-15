use crate::eval;
use ::pom;
use ::std::fmt;
use ::std::io;

pub type ResultOutput = Result<Vec<Result<eval::Output, eval::Error>>, pom::Error>;

pub fn fmt(
    f: &mut impl io::Write,
    username: &str,
    input: &str,
    output: &ResultOutput,
) -> io::Result<()> {
    writeln!(
        f,
        "{} rolled ... {}\nThey got ... {}",
        username,
        input,
        ResultOutputFormatter(&output)
    )?;

    Ok(())
}

struct ResultOutputFormatter<'a>(&'a ResultOutput);
impl<'a> fmt::Display for ResultOutputFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultOutputFormatter(Ok(outputs)) => fmt_parse_ok(f, &outputs),
            ResultOutputFormatter(Err(parse_err)) => fmt_parse_err(f, &parse_err),
        }
    }
}

fn fmt_parse_ok(
    f: &mut fmt::Formatter<'_>,
    outputs: &Vec<Result<eval::Output, eval::Error>>,
) -> fmt::Result {
    for (i, output) in outputs.iter().enumerate() {
        if i > 0 {
            write!(f, " ")?;
        }

        fmt_output(f, output)?;
    }

    Ok(())
}

fn fmt_parse_err(f: &mut fmt::Formatter<'_>, err: &pom::Error) -> fmt::Result {
    write!(f, "error ... {:?}", err)
}

fn fmt_output(
    f: &mut fmt::Formatter<'_>,
    output_result: &Result<eval::Output, eval::Error>,
) -> fmt::Result {
    match output_result {
        Ok(output) => fmt_output_ok(f, output),
        Err(err) => fmt_output_err(f, err),
    }
}

fn fmt_output_ok(f: &mut fmt::Formatter<'_>, output: &eval::Output) -> fmt::Result {
    write!(f, "{}", output)
}

fn fmt_output_err(f: &mut fmt::Formatter<'_>, err: &eval::Error) -> fmt::Result {
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
