const ROOT_CONSTRUCTS: &[&str] = &["identifier", "Type", "any", "the", "i32", "variant", "pick"];
const TEXT_CONSTRUCTS: &[&str] = &["identifier", "i32"];
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
    ("tis", "type_is"),
    ("ix", "type_is_exactly"),
];

pub fn is_root_label(label: &str) -> bool {
    ROOT_CONSTRUCTS.iter().any(|i| *i == label)
}

pub fn is_text_label(label: &str) -> bool {
    TEXT_CONSTRUCTS.iter().any(|i| *i == label)
}

pub fn resolve_alias(original: &str) -> &str {
    for (alias, real_label) in ALIASES {
        if *alias == original {
            return *real_label;
        }
    }
    original
}
