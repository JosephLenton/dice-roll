
use ::std::str::{self, FromStr};
use ::std::marker::PhantomData;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Error {
  position: LineColumn,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct LineColumn {
  line: u32,
  column: u32,
}

pub trait TParser<'a> : Copy + Sized {
  type Return;

  fn parse(&self, input: &'a str) -> Result<Self::Return, Error>;
  fn map<R2:Copy, MF>(&self, map_function:MF) -> MapParser<'a, Self, Self::Return, R2, MF>
    where MF: (Fn(Self::Return) -> R2) + Copy + Sized;
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct SymParser {
  symbol: &'static str
}

impl<'a> TParser<'a> for SymParser {
  type Return = &'a str;

  fn parse(&self, input: &'a str) -> Result<Self::Return, Error> {
    if input.starts_with(self.symbol) {
      return Ok(input)
    }

    Err(Error {
      position: LineColumn {
        line: 1,
        column: 1,
      }
    })
  }

  fn map<R2, MF>(&self, map_function:MF) -> MapParser<'a, Self, Self::Return, R2, MF>
    where MF: (Fn(&'a str) -> R2) + Copy + Sized
  {
    MapParser {
      inner: *self,
      map_function,
      phantom_t: PhantomData,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct MapParser<'a, T:TParser<'a, Return=I>, I, R, MF:(Fn(I) -> R) + Copy + Sized> {
  inner: T,
  map_function : MF,
  phantom_t: PhantomData<&'a T>,
}

impl <'a, T:TParser<'a, Return=I>, I:Copy, R:Copy, IF:(Fn(I) -> R) + Copy> TParser<'a> for MapParser<'a, T, I, R, IF> {
  type Return = R;

  fn parse(&self, input: &'a str) -> Result<Self::Return, Error> {
    let r = self.inner.parse(input)?;
    Ok((self.map_function)(r))
  }

  fn map<R2:Copy, MF>(&self, map_function:MF) -> MapParser<'a, Self, Self::Return, R2, MF>
    where MF: (Fn(Self::Return) -> R2) + Copy + Sized
  {
    MapParser {
      inner: *self,
      map_function,
      phantom_t: PhantomData,
    }
  }
}

fn sym(symbol: &'static str) -> SymParser {
  SymParser {
    symbol,
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_yolo() {
    let parser = sym("yolo");
    assert_eq!(parser.parse("yolo blah"), Ok("yolo blah"));
  }

  #[test]
  fn it_should_map_nums() {
    let parser = sym("1234").map(|input| i64::from_str(&input).unwrap());
    assert_eq!(parser.parse("1234"), Ok(1234));
  }
}

/*
type FnParse<'a, R> = impl Fn(&'a str) -> Result<R, Error> + 'a;

pub struct Parser<'a, R = &'a str> {
  test_function : Box<FnParse<'a, R>>
}

impl<'a, R: 'a> Parser<'a, R> {
  fn parse(&self, input : &'a str) -> Result<R, Error> {
    (self.test_function)(input)
  }

  //fn map(&self, f:impl Fn(U) -> B) -> MapReturn;
}
*/

/*
pub fn sym<'a>(symbol : &'static str) -> Parser<'a, &'a str> {
  let test_function : FnParse<'a, &'a str> = move |input| {
    if input.starts_with(symbol) {
      return Ok(input)
    }

    Err(Error {
      position: LineColumn {
        line: 1,
        column: 1,
      }
    })
  };

  Parser {
    test_function: box test_function,
  }
}
*/

/*
struct SymParser<M + Sized, U> {
  symbol: &'static str,
  map_function: M,
}

impl<M, U> SymParser<U> {
  fn new(symbol: &'static str) -> Self {
    Self {
      symbol,
      map_function: |input| -> input,
    }
  }

  fn map<B>(&self, f:impl Fn(U) -> B) -> MapReturn {

  }
}
*/

/*
impl Parser<'a, U = &'a str> for SymParser {
  fn parse(&self, input : &str) -> Result<U, Error> {
    Ok(self.symbol)
  }

  fn map<B>(&self, f:impl Fn(U) -> B) -> MapReturn;
}
*/

/*
pub fn one_of<'a>(part : &str) -> impl Parser<'a, &'a str> {
}

pub fn regex<'a>(regex: &str) -> impl Parser<'a, &'a str> {
}
*/