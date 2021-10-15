use nom::{branch::alt, complete::tag, multi::many0};

pub trait Parser<'a, O>: nom::Parser<&'a str, O, nom::Err<nom::error::ErrorKind>> {}

impl<'a, O, T: nom::Parser<&'a str, O, nom::Err<nom::error::ErrorKind>>> Parser<'a, O> for T {}

#[derive(Clone, Debug)]
pub enum AtomicRule {
    Fundamental(Vec<AtomicRule>),
    Symbol(String),
}

impl AtomicRule {
    fn parse_fundamental<'a>() -> impl Parser<'a, Self> {
        |input| todo!()
    }

    fn parse_explicit_symbol<'a>() -> impl Parser<'a, Self> {
        |input| todo!()
    }

    fn parse_implicit_symbol<'a>() -> impl Parser<'a, Self> {
        |input| todo!()
    }

    fn parse<'a>() -> impl Parser<'a, Self> {
        alt((
            Self::parse_fundamental(),
            Self::parse_explicit_symbol(),
            Self::parse_implicit_symbol(),
        ))
    }
}

pub fn parse<'a>() -> impl Parser<'a, Vec<AtomicRule>> {
    |input| todo!()
}
