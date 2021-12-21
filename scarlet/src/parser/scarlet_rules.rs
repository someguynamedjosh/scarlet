use super::{
    rule::{self, Rule},
    scarlet_creators,
    stack::CreateFn,
};

pub fn rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    for (name, car, re) in [
        (
            "keyword UNIQUE",
            Some(scarlet_creators::unique as CreateFn),
            r"\bUNIQUE\b",
        ),
        ("keyword AXIOM_OF_EQUALITY", None, r"\bAXIOM_OF_EQUALITY\b"),
    ] {
        rules.push(rule::phrase(name, car, None, [(re, 0, false, vec![])]));
    }
    for (name, car, re) in [
        ("variable", None, r"\b(VARIABLE|VAR|V)\b"),
        ("populated struct", None, r"\bPOPULATED_STRUCT\b"),
        ("if/then/else", None, r"\bIF_THEN_ELSE\b"),
    ] {
        rules.push(rule::phrase(
            name,
            car,
            None,
            [
                (re, 255, false, vec![]),
                (r"\[", 255, true, vec![]),
                (r"\]", 4, false, vec![]),
            ],
        ));
    }

    rules.push(rule::phrase(
        "parentheses",
        None,
        None,
        [(r"\(", 255, true, vec![]), (r"\)", 1, false, vec![])],
    ));
    rules.push(rule::phrase(
        "struct",
        None,
        None,
        [(r"\{", 255, true, vec![]), (r"\}", 1, false, vec![])],
    ));

    for (name, car, re) in [
        ("label access", None, r"\.LABEL"),
        ("value access", None, r"\.VALUE"),
        ("rest access", None, r"\.REST"),
        ("check is populated struct", None, r"\.IS_POPULATED_STRUCT"),
    ] {
        rules.push(rule::phrase(name, car, Some(4), [(re, 255, false, vec![])]));
    }
    rules.push(rule::phrase(
        "member access",
        None,
        Some(4),
        [(r"\.", 4, true, vec![])],
    ));
    rules.push(rule::phrase(
        "substitution",
        None,
        Some(4),
        [(r"\[", 255, true, vec![]), (r"\]", 4, false, vec![])],
    ));

    rules.push(rule::phrase(
        "target specifier",
        None,
        Some(253),
        [(r"IS", 253, true, vec![])],
    ));
    rules.push(rule::phrase(
        "equal operator",
        Some(scarlet_creators::equal as CreateFn),
        Some(65),
        [(r"=", 65, true, vec![])],
    ));
    // rules.push(rule::phrase(Some(20), [(r"\+", 20, true, vec![])]));
    // rules.push(rule::phrase(Some(9), [(r"\^", 10, true, vec![])]));

    rules
}
