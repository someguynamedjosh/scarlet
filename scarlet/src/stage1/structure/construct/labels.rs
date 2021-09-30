pub const ROOT_CONSTRUCT_LABELS: &[&str] = &[
    "identifier",
    "any",
    "the",
    "u8",
    "variant",
    "pick",
    "builtin_item",
];
const TEXT_CONSTRUCT_LABELS: &[&str] = &["identifier", "u8"];
const ALIASES: &[(&str, &str)] = &[
    ("iv", "is_variant"),
    ("T", "Type"),
    ("F", "FromItems"),
    ("From", "FromItems"),
    ("d", "defining"),
    ("r", "replacing"),
    ("p", "pick"),
    ("pick_by_conditions", "pick"),
    ("bt", "base_type_is"),
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
