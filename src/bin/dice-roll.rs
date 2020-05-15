#![feature(box_syntax, box_patterns)]

use ::std::env;
use ::std::io;

use ::dice_roll;

fn main() -> io::Result<()> {
    let username = "You";
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let mut stdout = io::stdout();

    dice_roll::main(&username, &input, &mut stdout)?;

    Ok(())
}
