use crate::stage2::{
    structure::{BuiltinOperation, Definition, Token, VarType},
    transform::{
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureAny, PatPlain, Pattern, PatternMatchSuccess},
    },
};

macro_rules! binary_operator {
    ($StructName:ident, $internal_name:expr, $operator:expr) => {
        pub struct $StructName;
        impl Transformer for $StructName {
            fn pattern(&self) -> Box<dyn Pattern> {
                Box::new((
                    PatCaptureAny { key: "left" },
                    $operator,
                    PatCaptureAny { key: "right" },
                ))
            }

            fn apply<'t>(
                &self,
                c: &mut ApplyContext<'_, 't>,
                success: PatternMatchSuccess<'_, 't>,
            ) -> TransformerResult<'t> {
                let left = c.push_token(success.get_capture("left").clone());
                let right = c.push_token(success.get_capture("right").clone());
                let result = Definition::BuiltinOperation($internal_name, vec![left, right]);
                let result = c.env.push_def(result);
                TransformerResult(Token::Item(result))
            }
        }
    };
}

binary_operator!(Caret, BuiltinOperation::Power32U, PatPlain("^"));
binary_operator!(Asterisk, BuiltinOperation::Product32U, PatPlain("*"));
binary_operator!(Slash, BuiltinOperation::Quotient32U, PatPlain("/"));
binary_operator!(Plus, BuiltinOperation::Sum32U, PatPlain("+"));
binary_operator!(Minus, BuiltinOperation::Difference32U, PatPlain("-"));
binary_operator!(Modulo, BuiltinOperation::Modulo32U, PatPlain("mod"));

binary_operator!(GreaterThan, BuiltinOperation::GreaterThan32U, PatPlain(">"));
binary_operator!(
    GreaterThanOrEqual,
    BuiltinOperation::GreaterThanOrEqual32U,
    (PatPlain(">"), PatPlain("="))
);
binary_operator!(LessThan, BuiltinOperation::LessThan32U, PatPlain("<"));
binary_operator!(
    LessThanOrEqual,
    BuiltinOperation::LessThanOrEqual32U,
    (PatPlain("<"), PatPlain("="))
);

// binary_operator!(Matches, BuiltinOperation::Matches, "matches");

pub struct PatternAnd;
impl Transformer for PatternAnd {
    fn pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatCaptureAny { key: "left" },
            PatPlain("AND"),
            PatCaptureAny { key: "right" },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let left = c.push_token(success.get_capture("left").clone());
        let right = c.push_token(success.get_capture("right").clone());
        let item = c.push_var(VarType::And(left, right));
        TransformerResult(Token::Item(item))
    }
}

pub struct PatternOr;
impl Transformer for PatternOr {
    fn pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatCaptureAny { key: "left" },
            PatPlain("OR"),
            PatCaptureAny { key: "right" },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let left = c.push_token(success.get_capture("left").clone());
        let right = c.push_token(success.get_capture("right").clone());
        let item = c.push_var(VarType::Or(left, right));
        TransformerResult(Token::Item(item))
    }
}

// binary_operator!(Member, BuiltinOperation::member, ".");
// binary_operator!(Using, BuiltinOperation::using, "using");
// binary_operator!(Is, BuiltinOperation::target, "is");

pub struct Is;
impl Transformer for Is {
    fn pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatCaptureAny { key: "left" },
            PatPlain("is"),
            PatCaptureAny { key: "right" },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let left = success.get_capture("left").clone();
        let right = c.push_token(success.get_capture("right").clone());
        let right = Token::Item(right);
        TransformerResult(Token::Stream {
            label: "target",
            contents: vec![left, right],
        })
    }
}
