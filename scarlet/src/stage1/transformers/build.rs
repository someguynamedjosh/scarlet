use crate::{
    stage1::transformers::{
        basics::{Extras, OwnedOrBorrowed, Precedence, SomeTransformer, Transformer},
        operators::*,
        roots::{Builtin, Struct, SubExpression},
        special_members::{Eager, Matched, Shown, Variable, MemberAtIndex},
        substitution::Substitution,
    },
    tfers,
};

pub fn build_transformers<'e>(
    precedence: Precedence,
    extras: &'e Extras<'e>,
) -> Vec<SomeTransformer<'e>> {
    let basics: Vec<Box<dyn Transformer>> = match precedence {
        10 => tfers![SubExpression, Struct, Builtin],
        20 => tfers![Matched, Variable, Shown, Eager, MemberAtIndex, Substitution, Member],
        61 => tfers![Caret],
        70 => tfers![Asterisk, Slash],
        80 => tfers![Plus, Minus],
        90 => tfers![Modulo],
        100 => tfers![GreaterThanOrEqual, GreaterThan, LessThanOrEqual, LessThan],
        120 => tfers![PatternAnd, PatternOr],
        130 => tfers![Matches],
        150 => tfers![Using],
        _ => tfers![],
    };
    let basics: Vec<_> = basics.into_iter().map(OwnedOrBorrowed::Owned).collect();
    if let Some(extras) = extras.get(&precedence) {
        let mut extras: Vec<_> = extras
            .iter()
            .map(|x| &**x)
            .map(OwnedOrBorrowed::Borrowed)
            .collect();
        let mut total = basics;
        total.append(&mut extras);
        total
    } else {
        basics
    }
}
