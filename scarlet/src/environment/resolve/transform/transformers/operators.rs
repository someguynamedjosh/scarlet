use crate::{
    constructs::{
        builtin_operation::{BuiltinOperation, CBuiltinOperation},
        variable::VarType,
    },
    environment::resolve::transform::{
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureAny, PatFirstOf, PatPlain, Pattern, PatternMatchSuccess},
    },
    tokens::structure::Token,
};

macro_rules! binary_operator {
    ($StructName:ident, $internal_name:expr, $operator:expr) => {
        pub struct $StructName;
        impl Transformer for $StructName {
            fn input_pattern(&self) -> Box<dyn Pattern> {
                Box::new((
                    PatCaptureAny { key: "left" },
                    $operator,
                    PatCaptureAny { key: "right" },
                ))
            }

            fn output_pattern(&self) -> Box<dyn Pattern> {
                Box::new(PatCaptureAny { key: "" })
            }

            fn apply<'t>(
                &self,
                c: &mut ApplyContext<'_, 't>,
                success: PatternMatchSuccess<'_, 't>,
            ) -> TransformerResult<'t> {
                let left = c.push_unresolved(success.get_capture("left").clone());
                let right = c.push_unresolved(success.get_capture("right").clone());
                let result = CBuiltinOperation {
                    op: $internal_name,
                    args: vec![left, right],
                };
                let result = c.env.push_construct(Box::new(result));
                TransformerResult(Token::Construct(result))
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

pub struct VariableAnd;
impl Transformer for VariableAnd {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatCaptureAny { key: "left" },
            PatFirstOf(vec![
                Box::new(PatPlain("VA")),
                Box::new(PatPlain("VAR_AND")),
                Box::new(PatPlain("VARIABLE_AND")),
            ]),
            PatCaptureAny { key: "right" },
        ))
    }

    fn output_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatCaptureAny { key: "" })
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let left = c.push_unresolved(success.get_capture("left").clone());
        let right = c.push_unresolved(success.get_capture("right").clone());
        let con = c.push_var(VarType::And(left, right), false);
        TransformerResult(Token::Construct(con))
    }
}

pub struct VariableOr;
impl Transformer for VariableOr {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatCaptureAny { key: "left" },
            PatFirstOf(vec![
                Box::new(PatPlain("VO")),
                Box::new(PatPlain("VAR_OR")),
                Box::new(PatPlain("VARIABLE_OR")),
            ]),
            PatCaptureAny { key: "right" },
        ))
    }

    fn output_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatCaptureAny { key: "" })
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let left = c.push_unresolved(success.get_capture("left").clone());
        let right = c.push_unresolved(success.get_capture("right").clone());
        let con = c.push_var(VarType::Or(left, right), false);
        TransformerResult(Token::Construct(con))
    }
}

// binary_operator!(Member, BuiltinOperation::member, ".");
// binary_operator!(Using, BuiltinOperation::using, "using");
// binary_operator!(Is, BuiltinOperation::target, "is");

pub struct Is;
impl Transformer for Is {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatCaptureAny { key: "left" },
            PatPlain("IS"),
            PatCaptureAny { key: "right" },
        ))
    }

    fn output_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatCaptureAny { key: "" })
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let left = success.get_capture("left").clone();
        let right = c.push_unresolved(success.get_capture("right").clone());
        let right = Token::Construct(right);
        TransformerResult(Token::Stream {
            label: "target",
            contents: vec![left, right],
        })
    }
}
