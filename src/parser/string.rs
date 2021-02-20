use crate::errors::ParserError;
use crate::parser::*;

pub fn char1(ch: char) -> impl Parser<Output = String> {
    satisfy(move |c| c == ch)
}

pub fn upper() -> impl Parser<Output = String> {
    satisfy(|c| c.is_uppercase())
}

pub fn lower() -> impl Parser<Output = String> {
    satisfy(|c| c.is_lowercase())
}

pub fn alphanumeric() -> impl Parser<Output = String> {
    satisfy(|c| c.is_alphanumeric())
}

pub fn letter() -> impl Parser<Output = String> {
    satisfy(|c| c.is_alphabetic())
}

#[derive(Clone)]
pub struct Str {
    s: String,
}

impl<'a> Parser for Str {
    type Output = String;

    fn parse<'b>(&self, input: &'b str) -> Result<(Self::Output, &'b str), ParserError> {
        let mut rest: &str = input;
        let mut output = "".to_owned();

        for c in self.s.chars() {
            let res = char1(c).parse(rest)?;
            let (a, b) = &res;
            output.push_str(a);
            rest = b;
        }
        Ok((output, rest))
    }
}

pub fn string(s: impl Into<String>) -> impl Parser<Output = String> {
    Str { s: s.into() }
}

#[test]
pub fn test_letter() {
    assert_eq!(letter().parse("abc").unwrap(), ("a".to_string(), "bc"));
}

#[test]
pub fn test_string() {
    assert_eq!(
        or(string("ab"), string("ac")).parse("ab"),
        Ok(("ab".to_string(), ""))
    );
    assert_eq!(
        or(string("ab"), string("ac")).parse("ac"),
        Ok(("ac".to_string(), ""))
    );
}
