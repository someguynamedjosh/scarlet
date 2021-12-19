use super::{ast::AstNode, rule::Rule, token::Token};
use crate::parser::{ast, state_set::StateSet};

pub fn parse_to_ast<'x>(
    input: &'x [Token<'x>],
    rules: &'x [Rule],
    root_nonterminal: &str,
) -> Result<AstNode<'x>, String> {
    let mut state_sets = vec![StateSet::new(rules, root_nonterminal)];
    for token in input {
        let next_state = StateSet::advance(rules, &state_sets[..], token);
        state_sets.push(next_state);
    }
    let end_index = input.len();
    let last_set = &state_sets[end_index];
    let root_state = last_set
        .states
        .iter()
        .position(|state| {
            state.is_complete() && state.rule.produced_nonterminal == root_nonterminal
        })
        .unwrap();
    let ast = ast::build_ast(&state_sets[..], &input[..], root_state, end_index);
    ast.map(|x| x.0)
}
