use crate::stage2::{
    structure::{BuiltinOperation, Definition, Environment, Token},
    transformers::basics::{Transformer, TransformerResult},
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
            fn should_be_applied_at(&self, to: &[Token], at: usize) -> bool {
                for (index, token) in $operator.iter().enumerate() {
                    if &to[at + index] != &Token::Plain(token) {
                        return false;
                    }
                }
                true
            }

            fn apply<'t>(
                &self,
                env: &mut Environment<'t>,
                to: &Vec<Token<'t>>,
                at: usize,
            ) -> TransformerResult<'t> {
                let left = env.push_token(to[at - 1].clone());
                let right = env.push_token(to[at + $operator.len()].clone());
                let result = Definition::BuiltinOperation($internal_name, vec![left, right]);
                let result = env.push_def(result);
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
// binary_operator!(PatternAnd, BuiltinOperation::PatternAnd, "AND");
// binary_operator!(PatternOr, BuiltinOperation::OR, "OR");

// binary_operator!(Member, BuiltinOperation::member, ".");
// binary_operator!(Using, BuiltinOperation::using, "using");
// binary_operator!(Is, BuiltinOperation::target, "is");

pub struct Is;
impl Transformer for Is {
    fn should_be_applied_at(&self, to: &[Token], at: usize) -> bool {
        &to[at] == &Token::Plain("is")
    }

    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        to: &Vec<Token<'t>>,
        at: usize,
    ) -> TransformerResult<'t> {
        let left = to[at - 1].clone();
        let right = env.push_token(to[at + 1].clone());
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
