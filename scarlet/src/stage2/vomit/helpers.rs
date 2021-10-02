use crate::stage1::structure::{
    construct::{Construct, ConstructBody},
    expression::Expression,
    statement::Statement,
};

pub fn single_expr_construct(label: &str, expr: Expression) -> Construct {
    Construct {
        body: ConstructBody::Statements(vec![Statement::Expression(expr)]),
        label: label.to_owned(),
    }
}

pub fn expressions_construct(label: &str, expressions: Vec<Expression>) -> Construct {
    statements_construct(
        label,
        expressions.into_iter().map(Statement::Expression).collect(),
    )
}

pub fn statements_construct(label: &str, statements: Vec<Statement>) -> Construct {
    Construct {
        body: ConstructBody::Statements(statements),
        label: label.to_owned(),
    }
}

pub fn text_construct(label: &str, text: String) -> Construct {
    Construct {
        body: ConstructBody::PlainText(text),
        label: label.to_owned(),
    }
}

pub fn identifier(name: &str) -> Construct {
    text_construct("identifier", name.to_owned())
}

pub fn simple_builtin_item(name: &str) -> Construct {
    single_expr_construct("builtin_item", just_root_expression(identifier(name)))
}

pub fn just_root_expression(root: Construct) -> Expression {
    Expression {
        root,
        others: vec![],
    }
}
