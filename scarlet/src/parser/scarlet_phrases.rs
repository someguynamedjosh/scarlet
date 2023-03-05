mod any;
mod as_language_item;
mod builtin;
mod identifier;
mod is;
// mod member_access;
mod multiple_items;
mod new_type;
mod structure;
// mod substitution;

use super::phrase::Phrase;

#[macro_export]
macro_rules! phrase {
    (
        $name:expr,
        $priority:expr,
        $create_and_uncreate:expr,
        $prec:expr => $($component:expr),*
    ) => {
        Phrase {
            name: $name,
            components: vec![$($component.into()),*],
            precedence: $prec,
            priority: $priority,
            create_and_uncreate: $create_and_uncreate,
        }
    }
}

pub fn phrases() -> Vec<Phrase> {
    vec![
        any::phrase(),
        as_language_item::phrase(),
        builtin::phrase(),
        identifier::phrase(),
        is::phrase(),
        // member_access::phrase(),
        multiple_items::phrase(),
        new_type::phrase(),
        structure::phrase(),
        // substitution::phrase(),
    ]
}
