use super::{
    nom_prelude::*,
    structure::{Module, Token, TokenTree},
};
use crate::entry::FileNode;

fn parse_token<'a>() -> impl Parser<'a, Token<'a>> {
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

fn parse_group<'a>() -> impl Parser<'a, TokenTree<'a>> {
    let data = tuple((recognize(one_of("{[(")), parse(), recognize(one_of("}])"))));
    map(data, |(start, body, end)| {
        // I'm dedicated.
        let name = match (start, end) {
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
        TokenTree::BuiltinRule { name, body }
    })
}

fn parse_primitive_rule<'a>() -> impl Parser<'a, TokenTree<'a>> {
    let begin = tuple((tag("primitive"), ws(), tag("{"), ws()));
    let name = preceded(begin, parse_token());
    let body = delimited(ws(), parse(), tag("}"));
    let data = tuple((name, body));
    map(data, |(name, body)| TokenTree::BuiltinRule { name, body })
}

fn parse_tree<'a>() -> impl Parser<'a, TokenTree<'a>> {
    let token = map(parse_token(), TokenTree::Token);
    alt((parse_primitive_rule(), parse_group(), token))
}

fn parse<'a>() -> impl Parser<'a, Vec<TokenTree<'a>>> {
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
