mod as_auto_theorem;
mod as_language_item;
mod axiom;
mod decision;
mod equal;
mod from;
mod identifier;
mod is;
mod is_populated_struct;
mod label_access;
mod member_access;
mod multiple_items;
mod parentheses;
mod populated_struct;
mod recursion;
mod rest_access;
mod shown;
mod structt;
mod substitution;
mod unique;
mod value_access;
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
        variable::phrase(),
        populated_struct::phrase(),
        equal::phrase(),
        from::phrase(),
        decision::phrase(),
        label_access::phrase(),
        value_access::phrase(),
        rest_access::phrase(),
        is_populated_struct::phrase(),
        shown::phrase(),
        as_language_item::phrase(),
        as_auto_theorem::phrase(),
        member_access::phrase(),
        substitution::phrase(),
        structt::phrase(),
        is::phrase(),
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
        multiple_items::phrase(),
        parentheses::phrase(),
        recursion::phrase(),
        identifier::phrase(),
    ]
}
