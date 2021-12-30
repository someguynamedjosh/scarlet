mod as_language_item;
mod equal_operator;
mod identifier;
mod if_then_else;
mod is;
mod is_populated_struct;
mod keyword_axiom_of_equality;
mod keyword_unique;
mod label_access;
mod member_access;
mod multiple_constructs;
mod parentheses;
mod populated_struct;
mod rest_access;
mod shown;
mod structt;
mod substitution;
mod value_access;
mod variable;

use super::phrase::Phrase;

#[macro_export]
macro_rules! phrase {
    ($name:expr, $create_and_uncreate:expr, $vomit:expr, $prec:expr => $($component:expr),*) => {
        Phrase {
            name: $name,
            components: vec![$($component.into()),*],
            create_and_uncreate: $create_and_uncreate,
            vomit: $vomit,
            precedence: $prec
        }
    }
}

pub fn phrases() -> Vec<Phrase> {
    vec![
        keyword_unique::phrase(),
        keyword_axiom_of_equality::phrase(),
        variable::phrase(),
        populated_struct::phrase(),
        if_then_else::phrase(),
        structt::phrase(),
        label_access::phrase(),
        value_access::phrase(),
        rest_access::phrase(),
        is_populated_struct::phrase(),
        shown::phrase(),
        as_language_item::phrase(),
        member_access::phrase(),
        substitution::phrase(),
        is::phrase(),
        equal_operator::phrase(),
        // phrase!(
        //     "add operator",
        //     None,
        //     20 => 20, r"\+", 20
        // ),
        // phrase!(
        //     "exponent operator",
        //     None,
        //     10 => 9, r"\^", 10
        // ),
        multiple_constructs::phrase(),
        parentheses::phrase(),
        identifier::phrase(),
    ]
}
