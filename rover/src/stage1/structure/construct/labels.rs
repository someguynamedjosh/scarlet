pub const ROOT_CONSTRUCT_LABELS: &[&str] = &[
    "identifier",
    "Type",
    "any",
    "the",
    "i32",
    "variant",
    "pick",
    "builtin_item",
];
const TEXT_CONSTRUCT_LABELS: &[&str] = &["identifier", "i32", "builtin_item"];
const ALIASES: &[(&str, &str)] = &[
    ("iv", "is_variant"),
    ("is_same_variant_as", "is_variant"),
    ("T", "Type"),
    ("F", "From"),
    ("FromVariables", "From"),
    ("d", "defining"),
    ("r", "replacing"),
    ("p", "pick"),
    ("pick_by_conditions", "pick"),
    ("t", "type_is"),
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
