use super::{Construct, ConstructBody};
use crate::stage1::structure::expression::Expression;

impl Construct {
    pub fn from_text(label: &str, body: &str) -> Self {
        Self::from_body(label, ConstructBody::PlainText(body.to_owned()))
    }

    pub fn from_expression(label: &str, expression: Expression) -> Self {
        Self::from_expressions(label, vec![expression])
    }

    pub fn from_expressions(label: &str, expressions: Vec<Expression>) -> Self {
        Self::from_body(label, ConstructBody::Expressions(expressions))
    }

    pub fn from_body(label: &str, body: ConstructBody) -> Self {
        Self {
            label: label.to_owned(),
            body,
        }
    }
}
