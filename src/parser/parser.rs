use super::*;
use crate::errors::ParserError;

pub trait Parser {
    type Output;

    fn parse<'a>(&self, input: &'a str) -> Result<(Self::Output, &'a str), ParserError>;

    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> B,
    {
        map(self, f)
    }
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
for_tuple!(P1, P2, P3, P4);
for_tuple!(P1, P2, P3, P4, P5);
for_tuple!(P1, P2, P3, P4, P5, P6);
for_tuple!(P1, P2, P3, P4, P5, P6, P7);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18);
for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24, P25
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24, P25, P26
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24, P25, P26, P27
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24, P25, P26, P27, P28
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24, P25, P26, P27, P28, P29
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24, P25, P26, P27, P28, P29, P30
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24, P25, P26, P27, P28, P29, P30, P31
);
for_tuple!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24, P25, P26, P27, P28, P29, P30, P31, P32
);

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
