mod ast;
mod rule;
mod state;
mod state_set;
mod token;

use self::{rule::Rule, token::Token};
use crate::{rule, rules};

mod top_level {
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
}

fn any_name(token: &Token) -> bool {
    token.role == "name"
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
    let tokens = token::tokenize(input);
    let ast = top_level::parse_to_ast(&tokens[..], &rules[..], "Root");
    println!("{:#?}", ast.unwrap());
}
