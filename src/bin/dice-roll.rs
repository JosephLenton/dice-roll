#![feature(box_syntax, box_patterns)]

use ::std::env;
use ::std::fmt;

use ::dice_roll;

fn main() -> Result<(), fmt::Error> {
    let username = "You";
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let mut output = String::new();

    dice_roll::main(&username, &input, &mut output)?;

    println!("{}", output);
    Ok(())
}
