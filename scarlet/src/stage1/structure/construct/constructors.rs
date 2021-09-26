use super::{Construct, ConstructBody};
use crate::stage1::structure::{expression::Expression, statement::Statement};

impl Construct {
    pub fn from_text(label: &str, body: &str) -> Self {
        Self::from_body(label, ConstructBody::PlainText(body.to_owned()))
    }

    pub fn from_expression(label: &str, expression: Expression) -> Self {
        let statement = Statement::Expression(expression);
        Self::from_statements(label, vec![statement])
    }

    pub fn from_statements(label: &str, statements: Vec<Statement>) -> Self {
        Self::from_body(label, ConstructBody::Statements(statements))
    }

    pub fn from_body(label: &str, body: ConstructBody) -> Self {
        Self {
            label: label.to_owned(),
            body,
        }
    }
}
