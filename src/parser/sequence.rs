use crate::errors::ParserError;
use crate::parser::*;

#[derive(Copy, Clone)]
pub struct Sequence2<P1: Parser<Output = String>, P2: Parser<Output = String>> {
    p1: P1,
    p2: P2,
}

impl<P1: Parser<Output = String>, P2: Parser<Output = String>> Parser for Sequence2<P1, P2> {
    type Output = String;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        let input = input;
        let (res1, rest) = self.p1.parse(input)?;
        let (res2, rest) = self.p2.parse(rest)?;
        Ok((res1.to_owned() + &res2, rest))
    }
}

pub fn sequence2<'a, P1: Parser<Output = String>, P2: Parser<Output = String>>(
    p1: P1,
    p2: P2,
) -> Sequence2<P1, P2> {
    Sequence2 { p1, p2 }
}

#[test]
fn test_sequence2() {
    assert_eq!(
        sequence2(letter(), digit()).parse("a1").unwrap(),
        ("a1".to_string(), "")
    );
    assert_eq!(
        sequence2(letter(), digit()).parse("aa").unwrap_err(),
        ParserError::NotSatisfy
    );
}
