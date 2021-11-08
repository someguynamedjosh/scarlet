use super::{nom_prelude::*, structure::Module};
use crate::{
    entry::FileNode,
    stage2::structure::{Token, TokenStream},
};

fn parse_plain_token<'a>() -> impl Parser<'a, &'a str> {
    |input: &'a str| {
        let parens = "{[()]}";
        let split_on = ".:!@$%^&*-=+\\|;'\",<>/?";
        let whitespace_indicators = " \t\r\n#";
        let (input, token) = alt((
            recognize(one_of(split_on)),
            take_while1(|c| {
                !split_on.contains(c) && !parens.contains(c) && !whitespace_indicators.contains(c)
            }),
        ))(input)?;
        for c in "{}()[]".chars() {
            if token.contains(c) {
                return fail(input);
            }
        }
        Ok((input, token))
    }
}

fn parse_group<'a>() -> impl Parser<'a, Token<'a>> {
    let data = tuple((recognize(one_of("{[(")), parse(), recognize(one_of("}])"))));
    map(data, |(start, contents, end)| {
        // I'm dedicated.
        let label = match (start, end) {
            ("{", "}") => "group{}",
            ("{", "]") => "group{]",
            ("{", ")") => "group{)",
            ("[", "}") => "group[}",
            ("[", "]") => "group[]",
            ("[", ")") => "group[)",
            ("(", "}") => "group(}",
            ("(", "]") => "group(]",
            ("(", ")") => "group()",
            _ => unreachable!(),
        };
        Token::Stream { label, contents }
    })
}

fn parse_builtin_rule<'a>() -> impl Parser<'a, Token<'a>> {
    let begin = tuple((tag("Builtin"), ws(), tag("("), ws()));
    let name = preceded(begin, parse_plain_token());
    let body = delimited(ws(), parse(), tag(")"));
    let data = tuple((name, body));
    map(data, |(label, contents)| Token::Stream { label, contents })
}

fn parse_tree<'a>() -> impl Parser<'a, Token<'a>> {
    let token = map(parse_plain_token(), Token::Plain);
    alt((parse_builtin_rule(), parse_group(), token))
}

fn parse<'a>() -> impl Parser<'a, TokenStream<'a>> {
    |input| terminated(many0(after_ws(parse_tree())), ws())(input)
}

pub fn ingest(file_tree: &FileNode) -> Module {
    let (remainder, self_content) = parse()(&file_tree.self_content).unwrap();
    if remainder.len() > 0 {
        eprintln!("Syntax error at: {}", remainder);
        todo!("nice error");
    }
    let mut children = Vec::new();
    for (name, tree) in &file_tree.children {
        children.push((name.clone(), ingest(tree)));
    }
    Module {
        self_content,
        children,
    }
}
