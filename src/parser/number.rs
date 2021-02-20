use crate::parser::*;

pub fn digit() -> impl Parser<Output = String> + Copy {
    satisfy(|c| c.is_digit(10))
}

pub fn integer<'a>() -> impl Parser<Output = i64> + Copy {
    many1(digit()).map(|s: String| {
        let mut n = 0;
        for c in s.chars() {
            n = n * 10 + (c as i64 - '0' as i64);
        }
        n
    })
}

#[test]
fn test_digit() {
    assert_eq!(digit().parse("123").unwrap(), ("1".to_string(), "23"));
}

#[test]
fn test_integer() {
    use crate::errors::ParserError;

    assert_eq!(integer().parse("123"), Ok((123, "")));
    assert_eq!(integer().parse("abc"), Err(ParserError::NotSatisfy));
    assert_eq!(
        (integer(), alphanumeric()).parse("123a"),
        Ok(((123, "a".to_owned()), ""))
    );
}
