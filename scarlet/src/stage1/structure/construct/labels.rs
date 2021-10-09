pub const PREFIX_CONSTRUCT_LABELS: &[&str] = &["defining", "FromValues", "on", "target"];
pub const ROOT_CONSTRUCT_LABELS: &[&str] = &["identifier", "any", "u8", "instance_of", "builtin_item"];
pub const POSTFIX_CONSTRUCT_LABELS: &[&str] = &["displayed", "matching", "same_instance", "substituting", "type_is"];
const TEXT_CONSTRUCT_LABELS: &[&str] = &["identifier", "u8"];
const ALIASES: &[(&str, &str)] = &[
    ("def", "defining"),
    ("dis", "displayed"),
    ("From", "FromValues"),
    ("mat", "matching"),
    ("pick_by_conditions", "pick"),
    ("sv", "same_instance"),
    ("same_instance_as", "same_instance"),
    ("sub", "substituting"),
    ("vo", "instance_of"),
];

pub fn is_text_label(label: &str) -> bool {
    TEXT_CONSTRUCT_LABELS.iter().any(|i| *i == label)
}

pub fn resolve_alias(original: &str) -> &str {
    for (alias, real_label) in ALIASES {
        if *alias == original {
            return *real_label;
        }
    }
    original
}
