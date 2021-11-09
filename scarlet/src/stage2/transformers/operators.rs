use crate::stage2::{
    structure::{Environment, Token},
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
                env: &mut Environment,
                to: &Vec<Token<'t>>,
                at: usize,
            ) -> TransformerResult<'t> {
                let left = to[at - 1].clone();
                let right = to[at + $operator.len()].clone();
                TransformerResult {
                    replace_range: at - 1..=at + $operator.len(),
                    with: Token::Stream {
                        label: $internal_name,
                        contents: vec![left, right],
                    },
                }
            }
        }
    };
}

binary_operator!(Caret, "power_32u", "^");
binary_operator!(Asterisk, "product_32u", "*");
binary_operator!(Slash, "quotient_32u", "/");
binary_operator!(Plus, "sum_32u", "+");
binary_operator!(Minus, "difference_32u", "-");
binary_operator!(Modulo, "modulo_32u", "mod");

binary_operator!(GreaterThan, "greater_than_32u", ">");
compound_binary_operator!(GreaterThanOrEqual, "greater_than_or_equal_32u", &[">", "="]);
binary_operator!(LessThan, "less_than_32u", "<");
compound_binary_operator!(LessThanOrEqual, "less_than_or_equal_32u", &["<", "="]);

binary_operator!(Matches, "matches", "matches");
binary_operator!(PatternAnd, "AND", "AND");
binary_operator!(PatternOr, "OR", "OR");

binary_operator!(Member, "member", ".");
binary_operator!(Using, "using", "using");
binary_operator!(Is, "target", "is");
