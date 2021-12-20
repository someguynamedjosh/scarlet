use super::rule::{self, Rule};

pub fn rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    rules.push(rule::phrase(None, [(r"\bUNIQUE\b", 0, false)]));
    rules.push(rule::phrase(
        None,
        [
            (r"\b(VARIABLE|VAR|V)\b", 255, false),
            (r"\[", 255, true),
            (r"\]", 4, false),
        ],
    ));
    rules.push(rule::phrase(Some(65), [(r"=", 65, true)]));
    rules.push(rule::phrase(Some(20), [(r"\+", 20, true)]));
    rules.push(rule::phrase(Some(9), [(r"\^", 10, true)]));
    rules.push(rule::phrase(None, [(r"\(", 255, true), (r"\)", 1, false)]));
    rules.push(rule::phrase(
        Some(4),
        [(r"\[", 255, true), (r"\]", 4, false)],
    ));

    rules
}
