use crate::{
    parser::{
        rule::{Component, Rule},
        scarlet_rules::recognizers::*,
    },
    rules,
};

pub fn rules() -> Vec<Rule> {
    rules![
        (
            Expr0 ->
            :IF_THEN_ELSE
            W (quote("["))
            W Expr
            W Expr
            W Expr
            W (quote("]"))
        )
    ]
}
