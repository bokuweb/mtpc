use crate::errors::ParserError;
use crate::parser::*;

#[derive(Copy, Clone)]
pub struct Or<O, P1: Parser<Output = O>, P2: Parser<Output = O>> {
    p1: P1,
    p2: P2,
}

impl<O, P1: Parser<Output = O>, P2: Parser<Output = O>> Parser for Or<O, P1, P2> {
    type Output = O;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        if let Ok(res) = self.p1.parse(input) {
            return Ok(res);
        }
        self.p2.parse(input)
    }
}

pub fn or<'a, O, P1: Parser<Output = O>, P2: Parser<Output = O>>(p1: P1, p2: P2) -> Or<O, P1, P2> {
    Or { p1, p2 }
}

#[test]
pub fn test_or() {
    assert_eq!(
        or(letter(), digit()).parse("a").unwrap(),
        ("a".to_string(), "")
    );
    assert_eq!(
        or(letter(), digit()).parse("1").unwrap(),
        ("1".to_string(), "")
    );
    assert_eq!(
        or(letter(), digit()).parse("!"),
        Err(ParserError::NotSatisfy)
    );
    assert_eq!(
        or(
            sequence2(char1('a'), char1('b')),
            sequence2(char1('c'), char1('b'))
        )
        .parse("ab")
        .unwrap(),
        ("ab".to_owned(), "")
    );
    assert_eq!(
        or(
            sequence2(char1('a'), char1('b')),
            sequence2(char1('c'), char1('b'))
        )
        .parse("cb")
        .unwrap(),
        ("cb".to_owned(), "")
    );
    assert_eq!(
        or(
            sequence2(char1('a'), char1('b')),
            sequence2(char1('c'), char1('b'))
        )
        .parse("acb")
        .unwrap_err(),
        ParserError::NotSatisfy,
    );
}
