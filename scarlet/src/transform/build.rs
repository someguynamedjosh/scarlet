use crate::{
    shared::OwnedOrBorrowed,
    tfers,
    transform::{
        basics::{Extras, Precedence, SomeTransformer, Transformer},
        transformers::{
            empty_struct::EmptyStruct, if_then_else::IfThenElse, operators::*,
            populated_struct::PopulatedStruct, special_members::*, statements,
            struct_sugar::StructSugar, sub_expression::SubExpression, substitution::Substitution,
            unique::Unique, variable::Variable,
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
        10 => tfers![
            Unique,
            SubExpression,
            Variable,
            PopulatedStruct,
            EmptyStruct,
            IfThenElse,
            StructSugar
        ],
        20 => tfers![
            IsPopulatedStruct,
            AsLanguageItem,
            Shown,
            StructLabel,
            StructValue,
            StructRest,
            Substitution
        ],
        // 61 => tfers![Caret],
        // 70 => tfers![Asterisk, Slash],
        // 80 => tfers![Plus, Minus],
        // 90 => tfers![Modulo],
        // 100 => tfers![GreaterThanOrEqual, GreaterThan, LessThanOrEqual, LessThan],
        100 => tfers![Equal],
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
