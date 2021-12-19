use crate::{
    parser::{
        rule::{Component, Rule},
        scarlet_rules::recognizers::*,
    },
    rule, rules,
};

pub fn rules() -> Vec<Rule> {
    let mut substitution = rule!(
        Expr0 ->
        Expr0
        W (quote("["))
        W ExprList
        W (quote("]"))
    );
    substitution.preferred = false;
    vec![substitution]
}
