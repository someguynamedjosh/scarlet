use crate::parse::{expression::Expression, nom_prelude::*};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, PartialEq)]
pub enum Statement {
    Is(Is),
    Replace(Replace),
    // MatchCase(MatchCase),
    Expression(Expression),
}

impl Statement {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        alt((
            map(Is::parser(), |s| Statement::Is(s)),
            map(Replace::parser(), |s| Statement::Replace(s)),
            // map(MatchCase::parser(), |s| Statement::MatchCase(s)),
            map(Expression::parser(), |s| Statement::Expression(s)),
        ))
    }
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Is(s) => s.fmt(f),
            Self::Replace(s) => s.fmt(f),
            Self::Expression(s) => s.fmt(f),
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
