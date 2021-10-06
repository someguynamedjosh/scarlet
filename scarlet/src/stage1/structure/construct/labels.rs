pub const PREFIX_CONSTRUCT_LABELS: &[&str] = &["defining", "FromValues"];
pub const ROOT_CONSTRUCT_LABELS: &[&str] = &["identifier", "any", "u8", "variant", "builtin_item"];
pub const POSTFIX_CONSTRUCT_LABELS: &[&str] = &["is_variant", "substituting", "type_is"];
const TEXT_CONSTRUCT_LABELS: &[&str] = &["identifier", "u8"];
const ALIASES: &[(&str, &str)] = &[
    ("d", "defining"),
    ("F", "FromValues"),
    ("From", "FromValues"),
    ("pick_by_conditions", "pick"),
    ("iv", "is_variant"),
    ("s", "substituting"),
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
