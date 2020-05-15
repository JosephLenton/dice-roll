use ::pom::parser::*;

pub fn optional<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

pub fn required<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(1..).discard()
}

pub fn comma<'a>() -> Parser<'a, u8, ()> {
    ((optional() - sym(b',') - optional()) | required()).discard()
}
