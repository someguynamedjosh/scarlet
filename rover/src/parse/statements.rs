use std::fmt::{self, Debug, Formatter};

use crate::parse::{
    expression::{Construct, ConstructBody, Expression},
    nom_prelude::*,
};

#[derive(Clone, PartialEq)]
pub enum Statement {
    Else(Else),
    Expression(Expression),
    Is(Is),
    PickIf(PickIf),
    PickElif(PickElif),
    Replace(Replace),
}

impl Statement {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        alt((
            map(Else::parser(), |s| Statement::Else(s)),
            map(Is::parser(), |s| Statement::Is(s)),
            map(Is::variant_shorthand_parser(), |s| Statement::Is(s)),
            map(PickIf::parser(), |s| Statement::PickIf(s)),
            map(PickElif::parser(), |s| Statement::PickElif(s)),
            map(Replace::parser(), |s| Statement::Replace(s)),
            map(Expression::parser(), |s| Statement::Expression(s)),
            // map(MatchCase::parser(), |s| Statement::MatchCase(s)),
        ))
    }
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Else(s) => s.fmt(f),
            Self::Expression(s) => s.fmt(f),
            Self::Is(s) => s.fmt(f),
            Self::PickIf(s) => s.fmt(f),
            Self::PickElif(s) => s.fmt(f),
            Self::Replace(s) => s.fmt(f),
        }
    }
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct Check {
//     pub value: Expression
// }

#[derive(Clone, PartialEq)]
pub struct Is {
    pub public: bool,
    pub name: Expression,
    pub value: Expression,
}

impl Is {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, name) = Expression::parser()(input)?;
            let (input, _) = ws()(input)?;
            let (input, _) = tag("is")(input)?;
            let (input, _) = ws()(input)?;
            let (input, public) = opt(tag("public"))(input)?;
            let public = public.is_some();
            let (input, _) = ws()(input)?;
            let (input, value) = Expression::parser()(input)?;
            let sel = Self {
                public,
                name,
                value,
            };
            Ok((input, sel))
        }
    }

    pub fn variant_shorthand_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = tag("variant")(input)?;
            let (input, _) = ws()(input)?;
            let (input, variant_def) = Expression::parser()(input)?;
            let name = Expression {
                root: variant_def.root.clone(),
                others: vec![],
            };
            let value = Expression {
                root: Construct {
                    label: format!("variant"),
                    body: ConstructBody::Statements(vec![Statement::Expression(variant_def)]),
                },
                others: vec![],
            };
            let sel = Self {
                public: true,
                name,
                value,
            };
            Ok((input, sel))
        }
    }
}

impl Debug for Is {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.name.fmt(f)?;
        write!(f, " is ")?;
        if self.public {
            write!(f, "public ")?;
        }
        self.value.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct PickIf {
    pub condition: Expression,
    pub value: Expression,
}

impl PickIf {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = tag("if")(input)?;
            let (input, _) = ws()(input)?;
            let (input, condition) = Expression::parser()(input)?;
            let (input, _) = ws()(input)?;
            let (input, _) = tag(",")(input)?;
            let (input, _) = ws()(input)?;
            let (input, value) = Expression::parser()(input)?;
            let sel = Self { condition, value };
            Ok((input, sel))
        }
    }
}

impl Debug for PickIf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "if ")?;
        self.condition.fmt(f)?;
        write!(f, ", ")?;
        self.value.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct PickElif {
    pub condition: Expression,
    pub value: Expression,
}

impl PickElif {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = tag("elif")(input)?;
            let (input, _) = ws()(input)?;
            let (input, condition) = Expression::parser()(input)?;
            let (input, _) = ws()(input)?;
            let (input, _) = tag(",")(input)?;
            let (input, _) = ws()(input)?;
            let (input, value) = Expression::parser()(input)?;
            let sel = Self { condition, value };
            Ok((input, sel))
        }
    }
}

impl Debug for PickElif {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "if ")?;
        self.condition.fmt(f)?;
        write!(f, ", ")?;
        self.value.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct Else {
    pub value: Expression,
}

impl Else {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = tag("else")(input)?;
            let (input, _) = ws()(input)?;
            let (input, value) = Expression::parser()(input)?;
            let sel = Self { value };
            Ok((input, sel))
        }
    }
}

impl Debug for Else {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "else ")?;
        self.value.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct Replace {
    pub target: Expression,
    pub value: Expression,
}

impl Replace {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, target) = Expression::parser()(input)?;
            let (input, _) = ws()(input)?;
            let (input, _) = tag("with")(input)?;
            let (input, _) = ws()(input)?;
            let (input, value) = Expression::parser()(input)?;
            let sel = Self { target, value };
            Ok((input, sel))
        }
    }
}

impl Debug for Replace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "replace ")?;
        self.target.fmt(f)?;
        write!(f, " with ")?;
        self.value.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchCase {
    pub constructor: Expression,
    pub value: Expression,
}
