#[derive(Clone, Debug, PartialEq)]
pub enum ParserError {
    ParsePositionError,
    NotSatisfy,
}

pub trait Parser {
    type Output;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError>;
}

pub fn digit() -> impl Parser<Output = String> {
    satisfy(|c| c.is_digit(10))
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

pub fn satisfy<F>(predicate: F) -> Satisfy<F>
where
    F: Fn(char) -> bool,
{
    Satisfy { predicate }
}

#[derive(Copy, Clone)]
pub struct Satisfy<F: Fn(char) -> bool> {
    predicate: F,
}

impl<F: Fn(char) -> bool> Parser for Satisfy<F> {
    type Output = String;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        let mut iter = input.char_indices();
        let (_, cur_char) = iter.next().ok_or(ParserError::ParsePositionError)?;

        if !(self.predicate)(cur_char) {
            return Err(ParserError::NotSatisfy);
        }
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        // self.pos += next_pos;
        Ok((cur_char.to_string(), &input[next_pos..]))
    }
}

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

fn many<'a>(parser: impl Parser<Output = String>) -> impl Parser<Output = String> {
    Many { parser }
}

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

fn or<'a, O, P1: Parser<Output = O>, P2: Parser<Output = O>>(p1: P1, p2: P2) -> Or<O, P1, P2> {
    Or { p1, p2 }
}

#[test]
pub fn test_digit() {
    assert_eq!(digit().parse("123").unwrap(), ("1".to_string(), "23"));
}

#[test]
pub fn test_letter() {
    assert_eq!(letter().parse("abc").unwrap(), ("a".to_string(), "bc"));
}

#[test]
pub fn test_digit_many() {
    assert_eq!(many(digit()).parse("123").unwrap(), ("123".to_owned(), ""));
}

#[test]
pub fn test_or() {
    assert_eq!(or(letter(), digit()).parse("a").unwrap(), ("a".to_string(), ""));
    assert_eq!(or(letter(), digit()).parse("1").unwrap(), ("1".to_string(), ""));
    assert_eq!(
        or(letter(), digit()).parse("!"),
        Err(ParserError::NotSatisfy)
    );
}

#[test]
pub fn test_or_many() {
    assert_eq!(
        many(or(letter(), digit())).parse("abc123").unwrap(),
        ("abc123".to_string(), "")
    );
}
