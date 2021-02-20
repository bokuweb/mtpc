use crate::errors::ParserError;
use crate::parser::*;

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

#[test]
pub fn test_replicate() {
    assert_eq!(
        sequence2(letter(), replicate(2, digit()))
            .parse("a23")
            .unwrap(),
        ("a23".to_string(), "")
    );
    assert_eq!(
        sequence2(letter(), replicate(2, digit()))
            .parse("abc")
            .unwrap_err(),
        ParserError::NotSatisfy,
    );
}
