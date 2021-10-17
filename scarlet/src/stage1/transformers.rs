use std::ops::{Range, RangeInclusive};

use super::structure::TokenTree;

pub struct TransformerResult<'t> {
    replace_range: RangeInclusive<usize>,
    with: TokenTree<'t>,
}

pub trait Transformer {
    /// Returns true if the transformer should be applied at the given location.
    fn should_be_applied_at(&self, tt: &TokenTree) -> bool;
    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t>;
}

struct Parentheses;
impl Transformer for Parentheses {
    fn should_be_applied_at(&self, tt: &TokenTree) -> bool {
        match tt {
            TokenTree::Group { start, end, .. } => *start == "(" && *end == ")",
            _ => false,
        }
    }

    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
        if let TokenTree::Group { body, .. } = &to[at] {
            let mut body = body.clone();
            apply_transformers(&mut body);
            let name = "paren";
            TransformerResult {
                replace_range: at..=at,
                with: TokenTree::PrimitiveRule { name, body },
            }
        } else {
            unreachable!("Checked in should_be_applied_at")
        }
    }
}

macro_rules! binary_operator {
    ($StructName:ident, $internal_name:expr, $operator:expr) => {
        struct $StructName;
        impl Transformer for $StructName {
            fn should_be_applied_at(&self, tt: &TokenTree) -> bool {
                tt == &TokenTree::Token($operator)
            }

            fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
                let left = to[at - 1].clone();
                let right = to[at + 1].clone();
                TransformerResult {
                    replace_range: at - 1..=at + 1,
                    with: TokenTree::PrimitiveRule {
                        name: $internal_name,
                        body: vec![left, right],
                    },
                }
            }
        }
    };
}

binary_operator!(Caret, "pow", "^");
binary_operator!(Asterisk, "mul", "*");
binary_operator!(Plus, "add", "+");

fn build_transformers(precedence: u8) -> Vec<Box<dyn Transformer>> {
    match precedence {
        10 => vec![Box::new(Parentheses)],
        61 => vec![Box::new(Caret)],
        70 => vec![Box::new(Asterisk)],
        80 => vec![Box::new(Plus)],
        _ => vec![],
    }
}

fn apply_transformer_ltr<'t>(
    to: &mut Vec<TokenTree<'t>>,
    transformer: &(impl Transformer + ?Sized),
) {
    let mut index = 0;
    while index < to.len() {
        if transformer.should_be_applied_at(&to[index]) {
            let result = transformer.apply(to, index);
            if !result.replace_range.contains(&index) {
                panic!(
                    "Transformer wants to replace {:?}, \
                    which does not contain the original index {}.",
                    result.replace_range, index
                );
            }
            index = *result.replace_range.start();
            to.splice(result.replace_range, std::iter::once(result.with));
        }
        index += 1;
    }
}

fn apply_transformer_rtl<'t>(
    to: &mut Vec<TokenTree<'t>>,
    transformer: &(impl Transformer + ?Sized),
) {
    let mut index = to.len();
    while index > 0 {
        index -= 1;
        if transformer.should_be_applied_at(&to[index]) {
            let result = transformer.apply(to, index);
            if !result.replace_range.contains(&index) {
                panic!(
                    "Transformer wants to replace {:?}, \
                    which does not contain the original index {}.",
                    result.replace_range, index
                );
            }
            index = *result.replace_range.start();
            to.splice(result.replace_range, std::iter::once(result.with));
        }
    }
}

pub fn apply_transformers<'t>(to: &mut Vec<TokenTree<'t>>) {
    for precedence in 0..=u8::MAX {
        for transformer in build_transformers(precedence) {
            if precedence % 2 == 0 {
                apply_transformer_ltr(to, &*transformer);
            } else {
                apply_transformer_rtl(to, &*transformer);
            }
        }
    }
}
