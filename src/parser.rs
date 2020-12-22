#[derive(Clone, Debug)]
pub enum ParserError {
    ParsePositionError,
    NotSatisfy,
}

pub trait Parser {
    type Output;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError>;
}

pub fn digit() -> impl Parser<Output = char> {
    satisfy(|c| c.is_digit(10))
}

// pub fn letter<Input>() -> impl Parser<Input, Output = char, PartialState = ()>
// where
//     Input: Stream<Token = char>,
//     Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
//     satisfy(|ch: char| ch.is_alphabetic()).expected("letter")
// }
//
pub fn satisfy<F>(predicate: F) -> Satisfy<F>
where
    F: Fn(char) -> bool,
{
    Satisfy { predicate }
}

// fn satisfy<F>(test: F) -> Result<char, ParserError>
// where
//     F: Fn(char) -> bool,
// {
//     if !test(self.peak()?) {
//         return Err(ParserError::NotSatisfy);
//     }
//     self.consume_char()
// }

#[derive(Copy, Clone)]
pub struct Satisfy<F: Fn(char) -> bool> {
    predicate: F,
}

impl<F: Fn(char) -> bool> Parser for Satisfy<F> {
    type Output = char;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        let mut iter = input.char_indices();
        let (_, cur_char) = iter.next().ok_or(ParserError::ParsePositionError)?;

        if !(self.predicate)(cur_char) {
            return Err(ParserError::NotSatisfy);
        }
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        // self.pos += next_pos;
        Ok((cur_char, &input[next_pos..]))
    }
}

#[derive(Copy, Clone)]
pub struct Many<P: Parser<Output = char>> {
    parser: P,
}

impl<P: Parser<Output = char>> Parser for Many<P> {
    type Output = String;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError> {
        let mut input = input;
        let mut res = String::new();
        loop {
            match self.parser.parse(input) {
                Ok(c) => {
                    input = c.1;
                    res.push(c.0);
                }
                Err(_) => return Ok((res, input)),
            }
        }
    }
}

fn many<'a>(parser: impl Parser<Output = char>) -> impl Parser<Output = String> {
    Many { parser }
}

// pub fn many1<F, Input, P>(p: P) -> Many1<F, P>
// where
//     Input: Stream,
//     F: Extend<P::Output> + Default,
//     P: Parser<Input>,
// {
//     Many1(p, PhantomData)
// }

/*
#[derive(Copy, Clone)]
pub struct Many1<F, P>(P, PhantomData<fn() -> F>);
impl<F, Input, P> Parser<Input> for Many1<F, P>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
{
    type Output = F;
    type PartialState = (bool, bool, F, P::PartialState);

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mut mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<F, Input::Error>
    where
        M: ParseMode,
    {
        let (ref mut parsed_one, ref mut committed_state, ref mut elements, ref mut child_state) =
            *state;

        if mode.is_first() || !*parsed_one {
            debug_assert!(!*parsed_one);

            let (first, committed) = ctry!(self.0.parse_mode(mode, input, child_state));
            elements.extend(Some(first));
            // TODO Should PeekOk be an error?
            *committed_state = !committed.is_peek();
            *parsed_one = true;
            mode.set_first();
        }

        let mut iter = Iter {
            parser: &mut self.0,
            committed: *committed_state,
            input,
            state: State::Ok,
            partial_state: child_state,
            mode,
        };
        elements.extend(iter.by_ref());

        iter.into_result_fast(elements).map(|x| {
            *parsed_one = false;
            x
        })
    }

    fn add_committed_expected_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.add_error(errors);
    }

    forward_parser!(Input, add_error parser_count, 0);
}
*/

#[test]
pub fn test() {
    assert_eq!(digit().parse("123").unwrap(), ('1', "23"));
    assert_eq!(many(digit()).parse("123").unwrap(), ("123".to_owned(), ""));
}
