use super::structure::{construct::Construct, expression::Expression, statement::Statement};
use crate::{stage1::structure::construct::ConstructBody, util::indented};

pub fn vomit(expr: &Expression) -> String {
    let mut result = vomit_root_construct(&expr.root);
    for post in &expr.posts {
        result.push_str(&vomit_postfix_construct(post));
    }
    result
}

fn vomit_root_construct(construct: &Construct) -> String {
    if construct.label == "identifier" || construct.label == "u8" {
        let text = construct.body.expect_text().unwrap();
        text.to_owned()
    } else {
        vomit_explicit_construct(construct)
    }
}

fn vomit_postfix_construct(construct: &Construct) -> String {
    match &construct.label[..] {
        "member" => vomit_member(construct),
        _ => vomit_explicit_postfix_construct(construct),
    }
}

fn vomit_member(construct: &Construct) -> String {
    let name = vomit(construct.expect_single_expression("member").unwrap());
    format!("::{}", name)
}

fn vomit_explicit_postfix_construct(construct: &Construct) -> String {
    let mut result = match &construct.body {
        ConstructBody::PlainText(..) => format!(" "),
        ConstructBody::Statements(..) => format!("\n"),
    };
    result.push_str(&vomit_explicit_construct(construct));
    result
}

fn vomit_explicit_construct(construct: &Construct) -> String {
    match &construct.body {
        ConstructBody::PlainText(txt) => format!("{}{{{}}}", construct.label, txt),
        ConstructBody::Statements(statements) => vomit_statements_body(construct, statements),
    }
}

fn vomit_statements_body(construct: &Construct, statements: &[Statement]) -> String {
    let mut result = format!("{}{{", construct.label);
    for statement in statements {
        result.push_str("\n    ");
        result.push_str(&indented(&vomit_statement(statement)))
    }
    result.push_str("\n}");
    result
}

fn vomit_statement(statement: &Statement) -> String {
    match statement {
        Statement::Else(s) => format!("else {}", vomit(&s.value)),
        Statement::Expression(s) => format!("{}", vomit(s)),
        Statement::Is(s) => format!("{} is {}", vomit(&s.name), vomit(&s.value)),
        _ => todo!(),
    }
}
