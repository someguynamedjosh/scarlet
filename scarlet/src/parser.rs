mod ast;
mod rule;
mod state;
mod state_set;
mod token;

use self::{
    rule::{Component::*, Rule},
    token::Token,
};
use crate::{parser::state_set::StateSet, rule, rules, shared::indented};

// https://en.wikipedia.org/wiki/Earley_parser

fn parse_internal(input: &str, rules: &[Rule], root_nonterminal: &str) {
    let mut state_sets = vec![StateSet::new(rules, root_nonterminal)];
    let tokens = token::tokenize(input);
    println!("{:?}", tokens);
    for token in &tokens {
        let next_state = StateSet::advance(rules, &state_sets[..], token);
        state_sets.push(next_state);
    }
    println!("{:#?}", state_sets);
    let end_index = tokens.len();
    let last_set = &state_sets[end_index];
    let root_state = last_set
        .states
        .iter()
        .position(|state| {
            state.is_complete() && state.rule.produced_nonterminal == root_nonterminal
        })
        .unwrap();
    let ast = ast::build_ast(&state_sets[..], &tokens[..], root_state, end_index);
    let ast = ast.unwrap().0;
    println!("{:#?}", ast);
}

fn any_name(token: &Token) -> bool {
    token.role == "name" && !["[", "]", "(", ")", "{", "}"].contains(&token.content)
}

fn any_whitespace(token: &Token) -> bool {
    token.role == "whitespace"
}

fn quote(text: &'static str) -> impl Fn(&Token) -> bool {
    move |token: &Token| token.content == text
}

pub fn parse(input: &str) {
    let mut rules = rules![
        (Root -> W Expr W)

        (ExprList -> )
        (ExprList -> ExprList W Expr)

        (Expr -> Expr4)
        (Expr4 -> Expr3)
        (Expr3 -> Expr2)
        (Expr2 -> Expr1)
        (Expr1 -> Expr0)

        (Expr3 -> Expr2 W := W Expr2)

        (
            Expr0 ->
            :POPULATED_STRUCT
            W (quote("["))
            W (any_name)
            W Expr
            W Expr
            W (quote("]"))
        )
        (Expr0 -> Expr W :. W :LABEL)
        (Expr0 -> Expr W :. W :VALUE)
        (Expr0 -> Expr W :. W :REST)
        (Expr0 -> Expr W :. W :IS_POPULATED_STRUCT)

        (
            Expr0 ->
            :IF_THEN_ELSE
            W (quote("["))
            W Expr
            W Expr
            W Expr
            W (quote("]"))
        )

        (
            Expr0 ->
            :VARIABLE
            W (quote("["))
            W ExprList
            W (quote("]"))
        )

        (Expr0 -> :AE)
        (Expr0 -> :UNIQUE)

        (W -> (any_whitespace))
        (W -> )
    ];

    let mut identifier = rule!(Expr0 -> (any_name));
    identifier.preferred = false;
    rules.push(identifier);

    let mut substitution = rule!(
        Expr0 ->
        Expr0
        W (quote("["))
        W ExprList
        W (quote("]"))
    );
    substitution.preferred = false;
    rules.push(substitution);

    println!("{:#?}", rules);
    parse_internal(input, &rules[..], "Root");
}
