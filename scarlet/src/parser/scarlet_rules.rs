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
            Some(scarlet_creators::unique),
            0 => r"\bUNIQUE\b"
        ),
        phrase!(
            "keyword AXIOM_OF_EQUALITY",
            Some(scarlet_creators::unique),
            0 => r"\bAXIOM_OF_EQUALITY\b"
        ),
        phrase!(
            "variable",
            Some(scarlet_creators::variable),
            0 => r"\b(VARIABLE|VAR|V)\b" , r"\[", 255, r"\]"
        ),
        phrase!(
            "populated struct",
            Some(scarlet_creators::populated_struct),
            0 => r"\bPOPULATED_STRUCT\b" , r"\[", 255, r"\]"
        ),
        phrase!(
            "if then else",
            Some(scarlet_creators::if_then_else),
            0 => r"\bIF_THEN_ELSE\b" , r"\[", 255, r"\]"
        ),
        phrase!(
            "parentheses",
            Some(scarlet_creators::parentheses),
            0 => r"\(", 255, r"\)"
        ),
        phrase!(
            "struct",
            Some(scarlet_creators::structt),
            0 => r"\{", 255, r"\}"
        ),
        phrase!(
            "label access",
            Some(scarlet_creators::atomic_struct_member::<{AtomicStructMember::Label}>),
            4 => 4, r"\.LABEL"
        ),
        phrase!(
            "value access",
            Some(scarlet_creators::atomic_struct_member::<{AtomicStructMember::Value}>),
            4 => 4, r"\.VALUE"
        ),
        phrase!(
            "rest access",
            Some(scarlet_creators::atomic_struct_member::<{AtomicStructMember::Rest}>),
            4 => 4, r"\.REST"
        ),
        phrase!(
            "is populated struct",
            Some(scarlet_creators::is_populated_struct),
            4 => 4, r"\.IS_POPULATED_STRUCT"
        ),
        phrase!(
            "shown",
            Some(scarlet_creators::shown),
            4 => 4, r"\.SHOWN"
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
