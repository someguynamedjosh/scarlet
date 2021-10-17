use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

use maplit::hashmap;

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
            TokenTree::PrimitiveRule { name, .. } => *name == "group()",
            _ => false,
        }
    }

    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
        if let TokenTree::PrimitiveRule { body, .. } = &to[at] {
            let mut body = body.clone();
            apply_transformers(&mut body, &Default::default());
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

fn expect_bracket_group<'a, 't>(tt: &'a TokenTree<'t>) -> &'a Vec<TokenTree<'t>> {
    if let TokenTree::PrimitiveRule {
        name: "group{}",
        body,
    } = tt
    {
        body
    } else {
        todo!("nice error, expected curly brackets")
    }
}

macro_rules! root_construct {
    ($StructName:ident, $label:expr, $extras:expr) => {
        struct $StructName;
        impl Transformer for $StructName {
            fn should_be_applied_at(&self, tt: &TokenTree) -> bool {
                tt == &TokenTree::Token($label)
            }

            fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
                let mut body = expect_bracket_group(&to[at + 1]).clone();
                let extras: Extras = $extras;
                apply_transformers(&mut body, &extras);
                TransformerResult {
                    replace_range: at..=at + 1,
                    with: TokenTree::PrimitiveRule { name: $label, body },
                }
            }
        }
    };
}

macro_rules! tfers {
    ($($transformer:expr),*) => {
        vec![$(Box::new($transformer) as BoxedTransformer),*]
    }
}

binary_operator!(Caret, "pow", "^");
binary_operator!(Asterisk, "mul", "*");
binary_operator!(Plus, "add", "+");
binary_operator!(Is, "target", "is");

root_construct!(Builtin, "builtin", hashmap![]);
root_construct!(Struct, "struct", hashmap![160 => tfers![Is]]);

pub type Precedence = u8;

pub enum Either<First, Second> {
    Fst(First),
    Snd(Second),
}

type BoxedTransformer = Box<dyn Transformer>;
type Extras<'e> = HashMap<Precedence, Vec<Box<dyn Transformer + 'e>>>;
type SomeTransformer<'e> = Either<Box<dyn Transformer>, &'e dyn Transformer>;

fn build_transformers<'e>(
    precedence: Precedence,
    extras: &'e Extras<'e>,
) -> Vec<SomeTransformer<'e>> {
    let basics: Vec<Box<dyn Transformer>> = match precedence {
        10 => tfers![Parentheses, Builtin, Struct],
        61 => tfers![Caret],
        70 => tfers![Asterisk],
        80 => tfers![Plus],
        _ => tfers![],
    };
    let basics: Vec<_> = basics.into_iter().map(Either::Fst).collect();
    if let Some(extras) = extras.get(&precedence) {
        let mut extras: Vec<_> = extras.iter().map(|x| &**x).map(Either::Snd).collect();
        let mut total = basics;
        total.append(&mut extras);
        total
    } else {
        basics
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

fn transformer_ref<'f>(from: &'f SomeTransformer) -> &'f dyn Transformer {
    match from {
        Either::Fst(boxed) => &**boxed,
        Either::Snd(plain) => *plain,
    }
}

pub fn apply_transformers<'e, 't>(
    to: &mut Vec<TokenTree<'t>>,
    extras: &'e HashMap<Precedence, Vec<Box<dyn Transformer + 'e>>>,
) {
    for precedence in 0..=u8::MAX {
        for transformer in build_transformers(precedence, extras) {
            let transformer = transformer_ref(&transformer);
            if precedence % 2 == 0 {
                apply_transformer_ltr(to, transformer);
            } else {
                apply_transformer_rtl(to, transformer);
            }
        }
    }
}
