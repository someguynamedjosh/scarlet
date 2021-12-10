use super::basics::{Extras, Precedence, SomeTransformer};
use crate::{
    shared::OwnedOrBorrowed,
    tfers,
    transform::{
        basics::Transformer,
        transformers::{
            operators::*, roots::*, special_members::*, statements, substitution::Substitution,
            unique::Unique,
        },
    },
};

pub fn all_transformers<'e>(extras: &'e Extras<'e>) -> Vec<SomeTransformer<'e>> {
    let mut result = tfers![];
    for prec in 0..=Precedence::MAX {
        result.append(&mut build_transformers(prec, extras));
    }
    result.append(
        &mut tfers![statements::OnPattern, statements::Else, Is]
            .into_iter()
            .map(OwnedOrBorrowed::Owned)
            .collect(),
    );
    result
}

pub fn build_transformers<'e>(
    precedence: Precedence,
    extras: &'e Extras<'e>,
) -> Vec<SomeTransformer<'e>> {
    let basics: Vec<Box<dyn Transformer>> = match precedence {
        10 => tfers![Unique, SubExpression, Struct],
        20 => tfers![AsLanguageItem, Shown, Variable, Substitution],
        61 => tfers![Caret],
        70 => tfers![Asterisk, Slash],
        80 => tfers![Plus, Minus],
        90 => tfers![Modulo],
        // 100 => tfers![GreaterThanOrEqual, GreaterThan, LessThanOrEqual, LessThan],
        // 130 => tfers![Matches],
        // 150 => tfers![Using],
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
