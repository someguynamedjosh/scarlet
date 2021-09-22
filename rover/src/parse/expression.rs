use crate::parse::{indented, nom_prelude::*, statements::Statement};
use nom::{bytes::complete::take_while1, combinator::fail};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, PartialEq)]
pub struct Expression {
    pub root: Construct,
    pub others: Vec<Construct>,
}

impl Expression {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, root) = Construct::parser(true)(input)?;
            let (input, others) = many0(after_ws(Construct::parser(false)))(input)?;
            let expr = Self { root, others };
            Ok((input, expr))
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.root.fmt(f)?;
        for con in &self.others {
            if f.alternate() {
                write!(f, "\n")?;
            } else {
                write!(f, " ")?;
            }
            con.fmt(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub enum ConstructBody {
    Statements(Vec<Statement>),
    PlainText(String),
}

impl ConstructBody {
    fn is_plain_text(&self) -> bool {
        match self {
            Self::PlainText(..) => true,
            _ => false,
        }
    }
}

impl Debug for ConstructBody {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Statements(statements) => {
                for s in statements {
                    if f.alternate() {
                        let st = format!("{:#?}", s);
                        write!(f, "{}\n", indented(&st))?;
                    } else {
                        write!(f, " ")?;
                        s.fmt(f)?;
                    }
                }
            }
            Self::PlainText(text) => {
                write!(f, "{}", text)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct Construct {
    pub label: String,
    pub body: ConstructBody,
}

const ROOT_CONSTRUCTS: &[&str] = &["identifier", "Type", "any", "the", "i32", "variant", "pick"];
const TEXT_CONSTRUCTS: &[&str] = &["identifier", "i32"];
const ALIASES: &[(&str, &str)] = &[
    ("iv", "is_variant"),
    ("is_same_variant_as", "is_variant"),
    ("T", "Type"),
    ("F", "From"),
    ("FromVariables", "From"),
    ("d", "defining"),
    ("r", "replacing"),
    ("p", "pick"),
    ("pick_by_conditions", "pick"),
];

fn is_root_label(label: &str) -> bool {
    ROOT_CONSTRUCTS.iter().any(|i| *i == label)
}

fn is_text_label(label: &str) -> bool {
    TEXT_CONSTRUCTS.iter().any(|i| *i == label)
}

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

    pub fn expect_single_expression(&self, label: &str) -> Result<&Expression, String> {
        let body = self.expect_statements(label)?;
        if body.len() != 1 {
            Err(format!(
                "Expected a single expression, got {} items instead.",
                body.len()
            ))
        } else {
            match &body[0] {
                Statement::Expression(expr) => Ok(expr),
                _ => Err(format!(
                    "Expected an expression, got a different statement instead."
                )),
            }
        }
    }

    pub fn parser<'i>(root: bool) -> impl Parser<'i, Self> {
        move |input| {
            if root {
                alt((
                    Self::explicit_construct_parser(true),
                    Self::integer_shorthand_parser(),
                    Self::ident_shorthand_parser(),
                ))(input)
            } else {
                alt((
                    Self::explicit_construct_parser(false),
                    Self::member_shorthand_parser(),
                    Self::replacing_shorthand_parser(),
                    Self::type_is_shorthand_parser(),
                ))(input)
            }
        }
    }

    fn explicit_construct_parser<'i>(root: bool) -> impl Parser<'i, Self> {
        move |input| {
            let (input, label) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
            let label = {
                let mut label = label;
                for (alias, real_label) in ALIASES {
                    if *alias == label {
                        label = *real_label;
                        break;
                    }
                }
                label
            };
            if is_root_label(label) != root {
                return fail(input);
            }
            let (input, _) = ws()(input)?;
            let (input, _) = tag("{")(input)?;
            let (input, body) = if is_text_label(label) {
                let (input, body) = take_until("}")(input)?;
                (input, ConstructBody::PlainText(String::from(body)))
            } else {
                let (input, body) = many0(after_ws(Statement::parser()))(input)?;
                (input, ConstructBody::Statements(body))
            };
            let (input, _) = ws()(input)?;
            let (input, _) = tag("}")(input)?;
            let label = String::from(label);
            Ok((input, Self { label, body }))
        }
    }

    fn ident_shorthand_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, name) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
            let body = ConstructBody::PlainText(String::from(name));
            let (input, _) = not(tuple((ws(), tag("{"))))(input)?;
            let label = String::from("identifier");
            Ok((input, Self { label, body }))
        }
    }

    fn member_shorthand_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = tag("::")(input)?;
            let (input, _) = ws()(input)?;
            let (input, root) = alt((
                Self::explicit_construct_parser(true),
                Self::ident_shorthand_parser(),
            ))(input)?;
            let others = Vec::new();
            let expr = Expression { root, others };
            let body = ConstructBody::Statements(vec![Statement::Expression(expr)]);
            let label = String::from("member");
            Ok((input, Self { label, body }))
        }
    }

    fn type_is_shorthand_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = tag(":")(input)?;
            let (input, _) = ws()(input)?;
            let (input, typee) = Expression::parser()(input)?;
            let label = String::from("type_is");
            let statement = Statement::Expression(typee);
            let body = ConstructBody::Statements(vec![statement]);
            Ok((input, Self { label, body }))
        }
    }

    fn replacing_shorthand_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = tag("[")(input)?;
            let (input, _) = ws()(input)?;
            let (input, body) = many0(after_ws(Statement::parser()))(input)?;
            let (input, _) = ws()(input)?;
            let (input, _) = tag("]")(input)?;
            let label = String::from("replacing");
            let body = ConstructBody::Statements(body);
            Ok((input, Self { label, body }))
        }
    }

    fn integer_shorthand_parser<'i>() -> impl Parser<'i, Self> {
        |input: &'i str| {
            let (input, data) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
            let without_underscores = data.replace("_", "");
            let is_int = without_underscores.parse::<i32>().is_ok();
            if !is_int {
                fail::<_, (), _>(input)?;
            }
            let label = format!("i32");
            let body = ConstructBody::PlainText(without_underscores);
            Ok((input, Self { label, body }))
        }
    }
}

impl Debug for Construct {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{{", self.label)?;
        if f.alternate() && !self.body.is_plain_text() {
            write!(f, "\n")?;
        }
        self.body.fmt(f)?;
        write!(f, "}}")
    }
}
