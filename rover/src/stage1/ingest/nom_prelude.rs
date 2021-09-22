pub use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while, take_while1},
    character::complete::one_of,
    combinator::{fail, map, not, opt, value, verify},
    multi::many0,
    sequence::tuple,
    IResult,
};

fn single_line_comment<'i>() -> impl Parser<'i, ()> {
    |input| {
        let (input, _) = tag("#")(input)?;
        let (input, _) = not(tag("="))(input)?;
        let (input, _) = take_while(|c| c != '\n')(input)?;
        let (input, _) = opt(tag("\n"))(input)?;
        Ok((input, ()))
    }
}

fn multi_line_comment<'i>() -> impl Parser<'i, ()> {
    |input| {
        let (input, _) = tag("#=")(input)?;
        let mut body_parser = alt((
            map(tag("=#"), |_| false),
            map(multi_line_comment(), |_| true),
            map(take(1usize), |_| true),
        ));
        let mut input = input;
        loop {
            let (new_input, more) = body_parser(input)?;
            input = new_input;
            if !more {
                break;
            }
        }
        Ok((input, ()))
    }
}

fn ws_element<'i>() -> impl Parser<'i, ()> {
    alt((
        multi_line_comment(),
        single_line_comment(),
        map(one_of(" \t\r\n"), |_| ()),
    ))
}

pub fn ws<'i>() -> impl Parser<'i, ()> {
    |input| {
        let (input, _) = many0(ws_element())(input)?;
        Ok((input, ()))
    }
}

pub fn after_ws<'i, T>(mut parser: impl Parser<'i, T>) -> impl Parser<'i, T> {
    move |input| {
        let (input, _) = ws()(input)?;
        let (input, val) = parser(input)?;
        Ok((input, val))
    }
}

pub trait Parser<'i, Result>: FnMut(&'i str) -> IResult<&'i str, Result> {}

impl<'i, T, Result> Parser<'i, Result> for T where T: FnMut(&'i str) -> IResult<&'i str, Result> {}
