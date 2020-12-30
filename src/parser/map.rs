use crate::errors::ParserError;
use crate::parser::Parser;

#[derive(Copy, Clone)]
pub struct Map<P, F>(P, F);

impl<A, B, P, F> Parser for Map<P, F>
where
    P: Parser<Output = A>,
    F: Fn(A) -> B,
{
    type Output = B;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        let (s, rest) = self.0.parse(input)?;
        let res = (self.1)(s);
        Ok((res, rest))
    }
}

pub fn map<P, F, B>(p: P, f: F) -> Map<P, F>
where
    P: Parser,
    F: Fn(P::Output) -> B,
{
    Map(p, f)
}
