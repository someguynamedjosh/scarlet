use super::rule::{self, Rule};

pub fn rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    for re in [r"\bUNIQUE\b", r"\bAXIOM_OF_EQUALITY\b"] {
        rules.push(rule::phrase(None, [(re, 0, false, vec![])]));
    }
    for re in [
        r"\b(VARIABLE|VAR|V)\b",
        r"\bPOPULATED_STRUCT\b",
        r"\bIF_THEN_ELSE\b",
    ] {
        rules.push(rule::phrase(
            None,
            [
                (re, 255, false, vec![]),
                (r"\[", 255, true, vec![]),
                (r"\]", 4, false, vec![]),
            ],
        ));
    }

    rules.push(rule::phrase(
        None,
        [(r"\(", 255, true, vec![]), (r"\)", 1, false, vec![])],
    ));
    rules.push(rule::phrase(
        None,
        [(r"\{", 255, true, vec![]), (r"\}", 1, false, vec![])],
    ));

    for re in [r"\.LABEL", r"\.VALUE", r"\.REST", r"\.IS_POPULATED_STRUCT"] {
        rules.push(rule::phrase(Some(4), [(re, 255, false, vec![])]));
    }
    rules.push(rule::phrase(Some(4), [(r"\.", 4, true, vec![])]));
    rules.push(rule::phrase(
        Some(4),
        [(r"\[", 255, true, vec![]), (r"\]", 4, false, vec![])],
    ));

    rules.push(rule::phrase(Some(253), [(r"IS", 253, true, vec![])]));
    rules.push(rule::phrase(Some(65), [(r"=", 65, true, vec![])]));
    // rules.push(rule::phrase(Some(20), [(r"\+", 20, true, vec![])]));
    // rules.push(rule::phrase(Some(9), [(r"\^", 10, true, vec![])]));

    rules
}
