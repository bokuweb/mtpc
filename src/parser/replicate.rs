use crate::errors::ParserError;
use crate::parser::Parser;

#[derive(Copy, Clone)]
pub struct Replicate<P: Parser<Output = String>> {
    n: usize,
    parser: P,
}

impl<P: Parser<Output = String>> Parser for Replicate<P> {
    type Output = String;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        let mut input = input;
        let mut res = String::new();
        for _ in 0..self.n {
            let (s, rest) = self.parser.parse(input)?;
            input = rest;
            res = res + &s;
        }
        Ok((res, input))
    }
}

pub fn replicate<'a>(
    n: usize,
    parser: impl Parser<Output = String>,
) -> impl Parser<Output = String> {
    Replicate { parser, n }
}
