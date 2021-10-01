use super::structure::{construct::Construct, expression::Expression, statement::Statement};
use crate::{stage1::structure::construct::ConstructBody, util::indented};

pub fn vomit(expr: &Expression) -> String {
    let mut result = vomit_root_construct(&expr.root);
    for post in &expr.others {
        result.push_str(&vomit_postfix_construct(post));
    }
    result
}

fn vomit_root_construct(construct: &Construct) -> String {
    if construct.label == "identifier" || construct.label == "u8" {
        let text = construct.body.expect_text().unwrap();
        text.to_owned()
    } else {
        todo!("{}", construct.label)
    }
}

fn vomit_postfix_construct(construct: &Construct) -> String {
    match &construct.label[..] {
        "member" => {
            format!(
                "::{}",
                vomit(construct.expect_single_expression("member").unwrap())
            )
        }
        other => match &construct.body {
            ConstructBody::PlainText(txt) => format!(" {}{{{}}}", other, txt),
            ConstructBody::Statements(statements) => {
                let mut result = format!("\n{}{{", other);
                for statement in statements {
                    result.push_str("\n    ");
                    result.push_str(&indented(&vomit_statement(statement)))
                }
                result.push_str("\n}");
                result
            }
        },
    }
}

fn vomit_statement(statement: &Statement) -> String {
    match statement {
        Statement::Else(s) => format!("else {}", vomit(&s.value)),
        Statement::Expression(s) => format!("{}", vomit(s)),
        Statement::Is(s) => format!("{} is {}", vomit(&s.name), vomit(&s.value)),
        _ => todo!(),
    }
}
