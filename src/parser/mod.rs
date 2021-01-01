mod map;
mod replicate;

use std::mem::swap;

use crate::errors::ParserError;

pub use map::*;
pub use replicate::*;

pub trait Parser {
    type Output;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError>;

    //     pub fn map<P, F, B>(p: P, f: F) -> Map<P, F>
    // where
    //     P: Parser,
    //     F: FnMut(P::Output) -> B,
    // {
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> B,
    {
        map(self, f)
    }
}

pub fn char1(ch: char) -> impl Parser<Output = String> {
    satisfy(move |c| c == ch)
}

pub fn digit() -> impl Parser<Output = String> + Copy {
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
pub struct Many1<P: Parser<Output = String>> {
    parser: P,
}

impl<P: Parser<Output = String> + Copy> Parser for Many1<P> {
    type Output = String;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        sequence2(self.parser, many(self.parser)).parse(input)
    }
}

fn many1<'a>(parser: impl Parser<Output = String> + Copy) -> impl Parser<Output = String> + Copy {
    Many1 { parser }
}

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

fn sequence2<'a, P1: Parser<Output = String>, P2: Parser<Output = String>>(
    p1: P1,
    p2: P2,
) -> Sequence2<P1, P2> {
    Sequence2 { p1, p2 }
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

fn integer<'a>() -> impl Parser<Output = i64> + Copy {
    many1(digit()).map(|s: String| {
        let mut n = 0;
        for c in s.chars() {
            n = n * 10 + (c as i64 - '0' as i64);
        }
        n
    })
}

macro_rules! for_tuple {
    ($h: ident $(, $id: ident)*) => {
        #[allow(non_snake_case)]
        impl <$h:, $($id:),*> Parser for ($h, $($id),*) where $h: Parser, $($id: Parser),* {
            type Output = ($h::Output, $($id::Output),*);

            fn parse<'b>(&self, input: &'b str) -> Result<(Self::Output, &'b str), ParserError> {
                let rest = input;
                let ($h, $($id),*) = self;
                let ($h, rest) = $h.parse(rest)?;
                $(
                    let ($id, rest) = $id.parse(rest)?;
                )*
                Ok((($h, $($id),*), rest))
            }
        }
    };
}

for_tuple!(P1, P2);
for_tuple!(P1, P2, P3);

// impl<'a, P1: Parser, P2: Parser> Parser for (P1, P2) {
//     type Output = (P1::Output, P2::Output);
//
//     fn parse<'b>(&self, input: &'b str) -> Result<(Self::Output, &'b str), ParserError> {
//         let (out1, rest) = self.0.parse(input)?;
//         let (out2, rest) = self.1.parse(rest)?;
//         Ok(((out1, out2), rest))
//     }
// }

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

#[test]
pub fn test_or_many() {
    assert_eq!(
        many(or(letter(), digit())).parse("abc123").unwrap(),
        ("abc123".to_string(), "")
    );
}

#[test]
pub fn test_sequence2() {
    assert_eq!(
        sequence2(letter(), digit()).parse("a1").unwrap(),
        ("a1".to_string(), "")
    );
    assert_eq!(
        sequence2(letter(), digit()).parse("aa").unwrap_err(),
        ParserError::NotSatisfy
    );
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

#[test]
pub fn test_integer() {
    assert_eq!(integer().parse("123"), Ok((123, "")));
    assert_eq!(integer().parse("abc"), Err(ParserError::NotSatisfy));

    (integer(), integer()).parse("123");
}

#[test]
pub fn test_tuple2() {
    let (res, _rest) = (integer(), string("abc")).parse("123abc").unwrap();
    assert_eq!(res.0, 123);
    assert_eq!(res.1, "abc".to_owned());

    let (res, _rest) = (integer(), string("abc"))
        .map(|res: (i64, String)| res.0)
        .parse("123abc")
        .unwrap();
    assert_eq!(res, 123);
}

#[test]
pub fn test_tuple3() {
    let (res, _rest) = (integer(), string("abc"), integer())
        .parse("123abc111")
        .unwrap();
    assert_eq!(res.0, 123);
    assert_eq!(res.1, "abc".to_owned());
    assert_eq!(res.2, 111);
}
