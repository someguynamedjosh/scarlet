use std::fmt::{self, Debug, Formatter};

use super::nom_prelude::*;
use crate::entry::FileNode;

#[derive(Clone, PartialEq, Eq)]
pub enum AtomicRule<'i> {
    Composite(Vec<AtomicRule<'i>>),
    Symbol(&'i str),
}

impl<'i> Debug for AtomicRule<'i> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Composite(rules) => {
                write!(f, "c")?;
                f.debug_list().entries(rules).finish()
            }
            Self::Symbol(text) => write!(f, "s {}", text),
        }
    }
}

impl<'i> AtomicRule<'i> {
    fn parse<'a>() -> impl Parser<'a, AtomicRule<'a>> {
        |input| {
            let split_on = "{}[]().:!@$%^&*-=+\\|;'\",<>/?";
            let whitespace_indicators = " \t\r\n#";
            let (input, text) = alt((
                recognize(one_of(split_on)),
                take_while1(|c| !split_on.contains(c) && !whitespace_indicators.contains(c)),
            ))(input)?;
            Ok((input, AtomicRule::Symbol(text)))
        }
    }
}

fn parse<'a>() -> impl Parser<'a, Vec<AtomicRule<'a>>> {
    terminated(many0(after_ws(AtomicRule::parse())), ws())
}

#[derive(Clone, Debug)]
pub struct Module<'a> {
    pub self_content: Vec<AtomicRule<'a>>,
    pub children: Vec<(String, Module<'a>)>,
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
