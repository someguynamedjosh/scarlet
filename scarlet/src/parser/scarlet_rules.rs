use super::{
    rule::{self, Phrase},
    scarlet_creators,
    stack::CreateFn,
};
use crate::constructs::structt::AtomicStructMember;

macro_rules! phrase {
    ($name:expr, $create_fn:expr, $prec:expr => $($component:expr),*) => {
        Phrase {
            name: $name,
            components: vec![$($component.into()),*],
            create_item: $create_fn,
            precedence: $prec
        }
    }
}

pub fn comma() -> Phrase {
    phrase!(
        "multiple constructs",
        None,
        255 => 254, r",", 255
    )
}

pub fn rules() -> Vec<Phrase> {
    vec![
        phrase!(
            "keyword UNIQUE",
            Some(scarlet_creators::unique as CreateFn),
            0 => r"\bUNIQUE\b"
        ),
        phrase!(
            "keyword AXIOM_OF_EQUALITY",
            Some(scarlet_creators::unique as CreateFn),
            0 => r"\bAXIOM_OF_EQUALITY\b"
        ),
        phrase!(
            "addition",
            None,
            20 => 20, r"\+", 20
        ),
        phrase!(
            "exponentiation",
            None,
            10 => 9, r"\^", 10
        ),
        phrase!(
            "identifier",
            None,
            0 => r"[a-zA-Z0-9_]+"
        ),
    ]

    // let mut phrases = Vec::new();

    // for (name, car, re) in [
    //     (
    //         "keyword UNIQUE",
    //         Some(scarlet_creators::unique as CreateFn),
    //         r"\bUNIQUE\b",
    //     ),
    //     ("keyword AXIOM_OF_EQUALITY", None, r"\bAXIOM_OF_EQUALITY\b"),
    // ] {
    //     phrases.push(rule::phrase(name, car, None, [(re, 0, false,
    // vec![])])); }
    // for (name, car, re) in [
    //     (
    //         "variable",
    //         Some(scarlet_creators::variable as CreateFn),
    //         r"\b(VARIABLE|VAR|V)\b",
    //     ),
    //     (
    //         "populated struct",
    //         Some(scarlet_creators::populated_struct as CreateFn),
    //         r"\bPOPULATED_STRUCT\b",
    //     ),
    //     (
    //         "if/then/else",
    //         Some(scarlet_creators::if_then_else as CreateFn),
    //         r"\bIF_THEN_ELSE\b",
    //     ),
    // ] {
    //     phrases.push(rule::phrase(
    //         name,
    //         car,
    //         None,
    //         [
    //             (re, 255, false, vec![]),
    //             (r"\[", 255, true, vec![]),
    //             (r"\]", 4, false, vec![]),
    //         ],
    //     ));
    // }

    // phrases.push(rule::phrase(
    //     "parentheses",
    //     Some(scarlet_creators::parentheses as CreateFn),
    //     None,
    //     [(r"\(", 255, true, vec![]), (r"\)", 1, false, vec![])],
    // ));
    // phrases.push(rule::phrase(
    //     "struct",
    //     Some(scarlet_creators::structt as CreateFn),
    //     None,
    //     [(r"\{", 255, true, vec![]), (r"\}", 1, false, vec![])],
    // ));

    // for (name, car, re) in [
    //     (
    //         "label access",
    //         Some(
    //             scarlet_creators::atomic_struct_member::<{
    // AtomicStructMember::Label }> as CreateFn,         ),
    //         r"\.LABEL",
    //     ),
    //     (
    //         "value access",
    //         Some(
    //             scarlet_creators::atomic_struct_member::<{
    // AtomicStructMember::Value }> as CreateFn,         ),
    //         r"\.VALUE",
    //     ),
    //     (
    //         "rest access",
    //         Some(
    //             scarlet_creators::atomic_struct_member::<{
    // AtomicStructMember::Rest }> as CreateFn,         ),
    //         r"\.REST",
    //     ),
    //     (
    //         "check is populated struct",
    //         Some(scarlet_creators::is_populated_struct as CreateFn),
    //         r"\.IS_POPULATED_STRUCT",
    //     ),
    //     (
    //         "shown",
    //         Some(scarlet_creators::shown as CreateFn),
    //         r"\.SHOWN",
    //     ),
    // ] {
    //     phrases.push(rule::phrase(name, car, Some(4), [(re, 255, false,
    // vec![])])); }
    // phrases.push(rule::phrase(
    //     "as builtin item",
    //     Some(scarlet_creators::builtin_item as CreateFn),
    //     Some(4),
    //     [
    //         (r"\.AS_BUILTIN_ITEM", 4, false, vec![]),
    //         (r"\[", 0, true, vec![]),
    //         (r"\]", 4, false, vec![]),
    //     ],
    // ));
    // phrases.push(rule::phrase(
    //     "member access",
    //     None,
    //     Some(4),
    //     [(r"\.", 4, true, vec![])],
    // ));
    // phrases.push(rule::phrase(
    //     "substitution",
    //     Some(scarlet_creators::substitution),
    //     Some(4),
    //     [(r"\[", 255, true, vec![]), (r"\]", 4, false, vec![])],
    // ));

    // phrases.push(rule::phrase(
    //     "target specifier",
    //     None,
    //     Some(253),
    //     [(r"IS", 253, true, vec![])],
    // ));
    // phrases.push(rule::phrase(
    //     "equal operator",
    //     Some(scarlet_creators::equal as CreateFn),
    //     Some(65),
    //     [(r"=", 65, true, vec![])],
    // ));
    // // rules.push(rule::phrase(Some(20), [(r"\+", 20, true, vec![])]));
    // // rules.push(rule::phrase(Some(9), [(r"\^", 10, true, vec![])]));

    // phrases
}
