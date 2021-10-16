use super::{
    nom_prelude::*,
    structure::{Module, Token},
};
use crate::entry::FileNode;

fn parse_token<'a>() -> impl Parser<'a, Token<'a>> {
    |input| {
        let split_on = "{}[]().:!@$%^&*-=+\\|;'\",<>/?";
        let whitespace_indicators = " \t\r\n#";
        let (input, token) = alt((
            recognize(one_of(split_on)),
            take_while1(|c| !split_on.contains(c) && !whitespace_indicators.contains(c)),
        ))(input)?;
        Ok((input, token))
    }
}

fn parse<'a>() -> impl Parser<'a, Vec<Token<'a>>> {
    terminated(many0(after_ws(parse_token())), ws())
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
