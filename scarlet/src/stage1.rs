use std::fmt::{self, Debug, Formatter};

use super::nom_prelude::*;
use crate::entry::FileNode;

#[derive(Clone, PartialEq, Eq)]
pub enum AtomicRule<'i> {
    Fundamental(Vec<AtomicRule<'i>>),
    Symbol(&'i str),
}

impl<'i> Debug for AtomicRule<'i> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Fundamental(rules) => {
                write!(f, "f")?;
                f.debug_list().entries(rules).finish()
            }
            Self::Symbol(text) => write!(f, "s {}", text),
        }
    }
}

impl<'i> AtomicRule<'i> {
    fn fundamental_tag<'a>() -> impl Parser<'a, ()> {
        map(alt((tag("fundamental"), tag("f"))), |_| ())
    }

    fn parse_fundamental<'a>() -> impl Parser<'a, AtomicRule<'a>> {
        |input| {
            let (input, _) = Self::fundamental_tag()(input)?;
            let (input, _) = after_ws(tag("{"))(input)?;
            let mut brace_count = 1;
            let mut rules = Vec::new();
            let mut input = input;
            while brace_count > 0 {
                let (input_now, next) = after_ws(Self::parse())(input)?;
                input = input_now;
                if next == AtomicRule::Symbol("{") {
                    brace_count += 1
                } else if next == AtomicRule::Symbol("}") {
                    brace_count -= 1;
                }
                rules.push(next);
            }
            rules.pop().unwrap();
            Ok((input, AtomicRule::Fundamental(rules)))
        }
    }

    fn parse_symbol<'a>() -> impl Parser<'a, AtomicRule<'a>> {
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

    fn parse<'a>() -> impl Parser<'a, AtomicRule<'a>> {
        alt((Self::parse_fundamental(), Self::parse_symbol()))
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
