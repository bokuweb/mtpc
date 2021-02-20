use crate::parser::*;
use crate::errors::ParserError;

#[derive(Copy, Clone)]
pub struct Many<P: Parser<Output = String>> {
    parser: P,
}

impl<P: Parser<Output = String>> Parser for Many<P> {
    type Output = String;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        let mut input = input;
        let mut res = String::new();
        loop {
            match self.parser.parse(input) {
                Ok(c) => {
                    input = c.1;
                    res = res + &c.0;
                }
                Err(_) => return Ok((res, input)),
            }
        }
    }
}

pub fn many<'a>(parser: impl Parser<Output = String>) -> impl Parser<Output = String> {
    Many { parser }
}

#[derive(Copy, Clone)]
pub struct Many1<P: Parser<Output = String>> {
    parser: P,
}

impl<P: Parser<Output = String> + Copy> Parser for Many1<P> {
    type Output = String;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        sequence2(self.parser, many(self.parser)).parse(input)
    }
}

pub fn many1<'a>(
    parser: impl Parser<Output = String> + Copy,
) -> impl Parser<Output = String> + Copy {
    Many1 { parser }
}

#[test]
pub fn test_digit_many() {
    assert_eq!(many(digit()).parse("123").unwrap(), ("123".to_owned(), ""));
}

#[test]
pub fn test_or_many() {
    assert_eq!(
        many(or(letter(), digit())).parse("abc123").unwrap(),
        ("abc123".to_string(), "")
    );
}
