use super::{rule::Rule, token::Token};
use crate::{parser::scarlet_rules::recognizers::*, rule, rules};

mod expression;
mod if_then_else;
mod operators;
mod plain_keywords;
mod populated_struct;
mod recognizers;
mod substitution;
mod variable;

pub fn scarlet_rules() -> Vec<Rule> {
    let mut rules = rules![
        (Root -> ExprList W)

        (ExprList -> )
        (ExprList -> ExprList W Expr)

        (W -> (any_whitespace))
        (W -> )
    ];

    rules.append(&mut expression::rules());
    rules.append(&mut if_then_else::rules());
    rules.append(&mut operators::rules());
    rules.append(&mut plain_keywords::rules());
    rules.append(&mut populated_struct::rules());
    rules.append(&mut substitution::rules());
    rules.append(&mut variable::rules());

    let mut identifier = rule!(Expr0 -> (any_name));
    identifier.preferred = false;
    rules.push(identifier);

    rules
}
