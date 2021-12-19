use std::fmt::{self, Debug, Formatter};

use super::{rule::Rule, state_set::StateSet, token::Token};
use crate::{parser::state::ComponentMatch, shared::indented};

#[derive(Clone)]
pub enum AstNode<'a> {
    Rule {
        rule: &'a Rule,
        components: Vec<AstNode<'a>>,
    },
    Terminal(&'a Token<'a>),
}

impl<'a> Debug for AstNode<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Rule { rule, components } => {
                writeln!(f, "{:?} [", rule)?;
                for component in components {
                    writeln!(f, "    {},", indented(&format!("{:?}", component)))?;
                }
                write!(f, "]")
            }
            AstNode::Terminal(token) => token.fmt(f),
        }
    }
}

pub fn build_ast<'a>(
    sets: &'a [StateSet<'a>],
    tokens: &'a [Token<'a>],
    state: usize,
    end_index: usize,
) -> Result<(AstNode<'a>, usize), String> {
    let state_set_here = &sets[end_index];
    let state = state_set_here.states.get_index(state).unwrap();
    assert!(state.is_complete());
    let rule = &state.rule;
    let mut components = Vec::new();
    let mut input_index = end_index;
    for component in state.matches.iter().rev() {
        match component {
            &ComponentMatch::ByState(state) => {
                let (node, new_input_index) = build_ast(sets, tokens, state, input_index)?;
                components.push(node);
                input_index = new_input_index;
            }
            &ComponentMatch::ByToken => {
                input_index -= 1;
                components.push(AstNode::Terminal(&tokens[input_index]));
            }
        }
    }
    components.reverse();
    let node = AstNode::Rule { rule, components };
    Ok((node, input_index))
}
