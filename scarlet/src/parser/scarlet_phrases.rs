mod any_proof_of;
mod as_auto_theorem;
mod as_language_item;
mod axiom;
mod builtin_function;
mod equal;
mod from;
mod identifier;
mod is;
mod member_access;
mod multiple_items;
mod shown;
mod structt;
mod substitution;
mod unique;
mod variable;

use super::phrase::Phrase;

#[macro_export]
macro_rules! phrase {
    (
        $name:expr,
        $priority:expr,
        $vomit_priority: expr,
        $create_and_uncreate:expr,
        $vomit:expr,
        $prec:expr => $($component:expr),*
    ) => {
        Phrase {
            name: $name,
            components: vec![$($component.into()),*],
            create_and_uncreate: $create_and_uncreate,
            vomit: $vomit,
            precedence: $prec,
            priority: $priority,
            vomit_priority: $vomit_priority
        }
    }
}

pub fn phrases() -> Vec<Phrase> {
    vec![
        unique::phrase(),
        axiom::phrase(),
        builtin_function::phrase(),
        variable::phrase(),
        any_proof_of::phrase(),
        equal::phrase(),
        from::phrase(),
        member_access::phrase(),
        shown::phrase(),
        as_language_item::phrase(),
        as_auto_theorem::phrase(),
        substitution::phrase(),
        structt::phrase(),
        is::phrase(),
        multiple_items::phrase(),
        identifier::phrase(),
    ]
}
