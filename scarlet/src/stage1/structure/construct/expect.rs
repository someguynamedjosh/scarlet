use super::{Construct, ConstructBody};
use crate::stage1::structure::{expression::Expression, statement::Statement};

impl Construct {
    pub fn expect_label(&self, label: &str) -> Result<&ConstructBody, String> {
        if self.label == label {
            Ok(&self.body)
        } else {
            Err(format!(
                "Expected a {} construct, got {} instead.",
                label, self.label
            ))
        }
    }

    pub fn expect_text(&self, label: &str) -> Result<&str, String> {
        let body = self.expect_label(label)?;
        match body {
            ConstructBody::PlainText(t) => Ok(t),
            ConstructBody::Statements(..) => panic!("{} is not a text construct", label),
        }
    }

    pub fn expect_ident(&self) -> Result<&str, String> {
        self.expect_text("identifier")
    }

    pub fn expect_statements(&self, label: &str) -> Result<&[Statement], String> {
        let body = self.expect_label(label)?;
        match body {
            ConstructBody::PlainText(..) => panic!("{} is a text construct", label),
            ConstructBody::Statements(s) => Ok(s),
        }
    }

    pub fn expect_single_statement(&self, label: &str) -> Result<&Statement, String> {
        let body = self.expect_statements(label)?;
        if body.len() == 1 {
            Ok(&body[0])
        } else {
            Err(format!(
                "Expected a single expression, got {} statements instead.",
                body.len()
            ))
        }
    }

    pub fn expect_single_expression(&self, label: &str) -> Result<&Expression, String> {
        match &self.expect_single_statement(label)? {
            Statement::Expression(expr) => Ok(expr),
            _ => Err("Expected an expression, got a different statement instead.".to_string()),
        }
    }
}
