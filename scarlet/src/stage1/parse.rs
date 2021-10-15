use super::{
    nom_prelude::*,
    structure::{Token, Module},
};
use crate::entry::FileNode;

impl<'i> Token<'i> {
    fn parse<'a>() -> impl Parser<'a, Token<'a>> {
        |input| {
            let split_on = "{}[]().:!@$%^&*-=+\\|;'\",<>/?";
            let whitespace_indicators = " \t\r\n#";
            let (input, text) = alt((
                recognize(one_of(split_on)),
                take_while1(|c| !split_on.contains(c) && !whitespace_indicators.contains(c)),
            ))(input)?;
            Ok((input, Token::Symbol(text)))
        }
    }
}

fn parse<'a>() -> impl Parser<'a, Vec<Token<'a>>> {
    terminated(many0(after_ws(Token::parse())), ws())
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
