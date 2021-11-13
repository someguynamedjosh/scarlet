use crate::stage2::{
    structure::{BuiltinOperation, Definition, Token, VarType},
    transform::basics::{ApplyContext, Transformer, TransformerResult},
};

macro_rules! binary_operator {
    ($StructName:ident, $internal_name:expr, $operator:expr) => {
        compound_binary_operator!($StructName, $internal_name, &[$operator]);
    };
}

macro_rules! compound_binary_operator {
    ($StructName:ident, $internal_name:expr, $operator:expr) => {
        pub struct $StructName;
        impl Transformer for $StructName {
            fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
                for (index, token) in $operator.iter().enumerate() {
                    if &c.to[at + index] != &Token::Plain(token) {
                        return false;
                    }
                }
                true
            }

            fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
                let left = c.push_token(c.to[at - 1].clone());
                let right = c.push_token(c.to[at + $operator.len()].clone());
                let result = Definition::BuiltinOperation($internal_name, vec![left, right]);
                let result = c.env.push_def(result);
                TransformerResult {
                    replace_range: at - 1..=at + $operator.len(),
                    with: Token::Item(result),
                }
            }
        }
    };
}

binary_operator!(Caret, BuiltinOperation::Power32U, "^");
binary_operator!(Asterisk, BuiltinOperation::Product32U, "*");
binary_operator!(Slash, BuiltinOperation::Quotient32U, "/");
binary_operator!(Plus, BuiltinOperation::Sum32U, "+");
binary_operator!(Minus, BuiltinOperation::Difference32U, "-");
binary_operator!(Modulo, BuiltinOperation::Modulo32U, "mod");

binary_operator!(GreaterThan, BuiltinOperation::GreaterThan32U, ">");
compound_binary_operator!(
    GreaterThanOrEqual,
    BuiltinOperation::GreaterThanOrEqual32U,
    &[">", "="]
);
binary_operator!(LessThan, BuiltinOperation::LessThan32U, "<");
compound_binary_operator!(
    LessThanOrEqual,
    BuiltinOperation::LessThanOrEqual32U,
    &["<", "="]
);

// binary_operator!(Matches, BuiltinOperation::Matches, "matches");

pub struct PatternAnd;
impl Transformer for PatternAnd {
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        &c.to[at] == &Token::Plain("AND")
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        let left = c.push_token(c.to[at - 1].clone());
        let right = c.push_token(c.to[at + 1].clone());
        let item = c.push_var(VarType::And(left, right));
        TransformerResult {
            replace_range: at - 1..=at + 1,
            with: Token::Item(item),
        }
    }
}

pub struct PatternOr;
impl Transformer for PatternOr {
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        &c.to[at] == &Token::Plain("OR")
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        let left = c.push_token(c.to[at - 1].clone());
        let right = c.push_token(c.to[at + 1].clone());
        let item = c.push_var(VarType::Or(left, right));
        TransformerResult {
            replace_range: at - 1..=at + 1,
            with: Token::Item(item),
        }
    }
}

// binary_operator!(Member, BuiltinOperation::member, ".");
// binary_operator!(Using, BuiltinOperation::using, "using");
// binary_operator!(Is, BuiltinOperation::target, "is");

pub struct Is;
impl Transformer for Is {
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        &c.to[at] == &Token::Plain("is")
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        let left = c.to[at - 1].clone();
        let right = c.push_token(c.to[at + 1].clone());
        let right = Token::Item(right);
        TransformerResult {
            replace_range: at - 1..=at + 1,
            with: Token::Stream {
                label: "target",
                contents: vec![left, right],
            },
        }
    }
}
