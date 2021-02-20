use crate::parser::*;
use crate::errors::ParserError;

#[derive(Copy, Clone)]
pub struct Satisfy<F: Fn(char) -> bool> {
    predicate: F,
}

pub fn satisfy<F>(predicate: F) -> Satisfy<F>
where
    F: Fn(char) -> bool,
{
    Satisfy { predicate }
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
