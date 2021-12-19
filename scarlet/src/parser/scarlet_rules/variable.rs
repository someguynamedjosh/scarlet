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
            :VARIABLE
            W (quote("["))
            W ExprList
            W (quote("]"))
        )
    ]
}
