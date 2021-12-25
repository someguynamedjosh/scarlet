use super::{phrase::Phrase, scarlet_creators};
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

pub fn phrases() -> Vec<Phrase> {
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
            "as builtin item",
            Some(scarlet_creators::builtin_item),
            4 => 4, r"\.AS_BUILTIN_ITEM", r"\[", 255, r"\]"
        ),
        phrase!(
            "member access",
            None,
            4 => 4, r"\.", 4
        ),
        phrase!(
            "substitution",
            Some(scarlet_creators::substitution),
            4 => 4, r"\[", 255, r"\]"
        ),
        phrase!(
            "is",
            None,
            250 => 250, r"IS", 250
        ),
        phrase!(
            "equal operator",
            Some(scarlet_creators::equal),
            65 => 65, r"=", 65
        ),
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
        phrase!(
            "multiple constructs",
            None,
            255 => 255, r",", 255
        ),
        phrase!(
            "identifier",
            Some(scarlet_creators::identifier),
            0 => r"[a-zA-Z0-9_]+"
        ),
    ]
}
