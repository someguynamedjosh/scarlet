use super::structure::{construct::Construct, expression::Expression};
use crate::{stage1::structure::construct::ConstructBody, util::indented};

pub fn vomit(expr: &Expression) -> String {
    let mut result = String::new();
    for pre in &expr.pres {
        result.push_str(&vomit_prefix_construct(pre));
    }
    result.push_str(&vomit_root_construct(&expr.root));
    for post in &expr.posts {
        result.push_str(&vomit_postfix_construct(post));
    }
    result
}

fn vomit_prefix_construct(construct: &Construct) -> String {
    match &construct.label[..] {
        _ => vomit_explicit_prefix_construct(construct),
    }
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

fn vomit_explicit_prefix_construct(construct: &Construct) -> String {
    let mut result = vomit_explicit_construct(construct);
    result.push_str(match &construct.body {
        ConstructBody::PlainText(..) => " ",
        ConstructBody::Expressions(..) => "\n",
    });
    result
}

fn vomit_explicit_postfix_construct(construct: &Construct) -> String {
    let mut result = match &construct.body {
        ConstructBody::PlainText(..) => format!(" "),
        ConstructBody::Expressions(..) => format!("\n"),
    };
    result.push_str(&vomit_explicit_construct(construct));
    result
}

fn vomit_explicit_construct(construct: &Construct) -> String {
    match &construct.body {
        ConstructBody::PlainText(txt) => format!("{}{{{}}}", construct.label, txt),
        ConstructBody::Expressions(statements) => vomit_expressions_body(construct, statements),
    }
}

fn vomit_expressions_body(construct: &Construct, expressions: &[Expression]) -> String {
    let mut result = format!("{}{{", construct.label);
    for expression in expressions {
        result.push_str("\n    ");
        result.push_str(&indented(&vomit(expression)))
    }
    result.push_str("\n}");
    result
}
