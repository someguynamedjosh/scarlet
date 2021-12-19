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
    ]
}
