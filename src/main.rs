mod parser;
mod errors;

fn main() {
    let mut t = Test {
        pos: 0,
        input: "abc".to_owned(),
    };
    dbg!(t.any_char());
    dbg!(t.any_char());
}

struct Test {
    pos: usize,
    input: String,
}

impl Test {
    // fn any_char(&self) -> char {
    //     self.input[self.pos..].chars().next().unwrap()
    // }

    fn test1(&mut self) -> String {
        let x1 = self.any_char();
        let x2 = self.any_char();
        format!("{}{}", x1, x2)
    }

    fn any_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }
}

struct Parser {
    pos: usize,
    input: String,
}

#[derive(Clone, Debug)]
enum ParserError {
    ParsePositionError,
    NotSatisfy,
}

impl Parser {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            pos: 0,
            input: s.into(),
        }
    }

    // Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Do the next characters start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peak(&self) -> Result<char, ParserError> {
        self.input[self.pos..]
            .chars()
            .next()
            .ok_or(ParserError::ParsePositionError)
    }

    fn consume_char(&mut self) -> Result<char, ParserError> {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().ok_or(ParserError::ParsePositionError)?;
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        Ok(cur_char)
    }

    fn consume_while<F>(&mut self, test: F) -> Result<String, ParserError>
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char()?);
        }
        Ok(result)
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // Parse a tag or attribute name.
    fn parse_tag_name(&mut self) -> Result<String, ParserError> {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false,
        })
    }

    fn any_char(&mut self) -> Result<char, ParserError> {
        self.consume_char()
    }

    fn test1(&mut self) -> Result<String, ParserError> {
        let x1 = self.any_char()?;
        let x2 = self.any_char()?;
        Ok(format!("{}{}", x1, x2))
    }

    fn test2(&mut self) -> Result<String, ParserError> {
        let x1 = self.test1()?;
        let x2 = self.any_char()?;
        Ok(format!("{}{}", x1, x2))
    }

    fn test3(&mut self) -> Result<String, ParserError> {
        let x1 = self.letter()?;
        let x2 = self.digit()?;
        let x3 = self.digit()?;
        Ok(format!("{}{}{}", x1, x2, x3))
    }

    fn satisfy<F>(&mut self, test: F) -> Result<char, ParserError>
    where
        F: Fn(char) -> bool,
    {
        if !test(self.peak()?) {
            return Err(ParserError::NotSatisfy);
        }
        self.consume_char()
    }

    fn digit(&mut self) -> Result<char, ParserError> {
        self.satisfy(|c| c.is_digit(10))
    }

    fn upper(&mut self) -> Result<char, ParserError> {
        self.satisfy(|c| c.is_uppercase())
    }

    fn lower(&mut self) -> Result<char, ParserError> {
        self.satisfy(|c| c.is_lowercase())
    }

    fn alpha(&mut self) -> Result<char, ParserError> {
        self.satisfy(|c| c.is_alphabetic())
    }

    fn alphanumeric(&mut self) -> Result<char, ParserError> {
        self.satisfy(|c| c.is_alphanumeric())
    }

    fn letter(&mut self) -> Result<char, ParserError> {
        self.satisfy(|c| c.is_alphabetic())
    }
}

#[test]
pub fn test() {
    let mut t = Parser::new("12");
    let res = t.test2();
    dbg!(res);

    let mut t = Parser::new("123");
    let res = t.test2();
    dbg!(res);

    let mut t = Parser::new("abc");
    let res = t.satisfy(|c| c.is_digit(10));
    dbg!(res);

    let mut t = Parser::new("1");
    let res = t.satisfy(|c| c.is_digit(10));
    dbg!(res);

    let mut t = Parser::new("abc");
    let res = t.digit();
    dbg!(res);

    let mut t = Parser::new("123");
    let res = t.digit();
    dbg!(res);

    let mut t = Parser::new("abc");
    let res = t.letter();
    dbg!(res);

    let mut t = Parser::new("123");
    let res = t.letter();
    dbg!(res);

    let mut t = Parser::new("abc");
    let res = t.test3();
    dbg!(res);

    let mut t = Parser::new("123");
    let res = t.test3();
    dbg!(res);

    let mut t = Parser::new("a23");
    let res = t.test3();
    dbg!(res);

    let mut t = Parser::new("a234");
    let res = t.test3();
    dbg!(res);
}
