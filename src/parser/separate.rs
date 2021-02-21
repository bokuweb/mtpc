use crate::errors::ParserError;
use crate::parser::*;

#[derive(Clone)]
pub struct SepBy<P, S> {
    parser: P,
    separator: S,
}

impl<'a, O, Q, P: Parser<Output = O>, S: Parser<Output = Q>> Parser for SepBy<P, S> {
    type Output = Vec<O>;

    fn parse<'b>(&self, input: &'b str) -> Result<(Self::Output, &'b str), ParserError> {
        let mut items: Self::Output = vec![];
        if let Ok(_) = self.separator.parse(input) {
            if let Ok(parsed) = self.parser.parse("") {
                items.push(parsed.0);
            } else {
                return Ok((items, input));
            }
        }

        let mut rest = input;
        loop {
            if let Ok(parsed) = self.parser.parse(rest) {
                items.push(parsed.0);
                if parsed.1.is_empty() {
                    return Ok((items, parsed.1));
                } else {
                    if let Ok(separated) = self.separator.parse(parsed.1) {
                        rest = separated.1;
                    } else {
                        return Ok((items, parsed.1));
                    }
                }
            } else {
                return Ok((items, rest));
            }
        }
    }
}

pub fn sep_by<'a, O, P, Q, S>(separator: S, parser: P) -> SepBy<P, S>
where
    P: Parser<Output = O>,
    S: Parser<Output = Q>,
{
    SepBy { parser, separator }
}

#[test]
fn test_sep_by() {
    let word = many1(letter());
    let parser = sep_by(whitespace(), word);
    let result = parser.parse("Pick up that word!");
    assert_eq!(
        result,
        Ok((
            vec![
                "Pick".to_owned(),
                "up".to_owned(),
                "that".to_owned(),
                "word".to_owned()
            ],
            "!"
        ))
    );
}

#[test]
fn test_sep_by_with_map() {
    let word = many1(letter());
    let parser = sep_by(whitespace(), word).map(|mut words: Vec<String>| words.pop());
    let result = parser.parse("Pick up that word!");
    assert_eq!(result, Ok((Some("word".to_string()), "!")));
}

#[test]
fn test_sep_by_with_space() {
    let word = many1(letter());
    let parser = sep_by(whitespace(), word);
    let result = parser.parse(" Pick up that word!");
    assert_eq!(result, Ok((vec![], " Pick up that word!")));
}
