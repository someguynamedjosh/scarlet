mod builtin;
mod identifier;
mod multiple_items;

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
        }
    }
}

pub fn phrases() -> Vec<Phrase> {
    vec![
        builtin::phrase(),
        identifier::phrase(),
        multiple_items::phrase(),
    ]
}
