use super::{
    nom_prelude::*,
    structure::{Module, TokenTree},
};
use crate::entry::FileNode;

fn parse_token<'a>() -> impl Parser<'a, TokenTree<'a>> {
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
        Ok((input, TokenTree::Token(token)))
    }
}

fn parse_group<'a>() -> impl Parser<'a, TokenTree<'a>> {
    let data = tuple((recognize(one_of("{[(")), parse(), recognize(one_of("}])"))));
    map(data, |(start, body, end)| TokenTree::Group {
        start,
        end,
        body,
    })
}

fn parse_tree<'a>() -> impl Parser<'a, TokenTree<'a>> {
    alt((parse_group(), parse_token()))
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
