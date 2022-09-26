
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
    ]
}
