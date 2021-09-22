pub(crate) fn indented(source: &str) -> String {
    source.replace("\n", "\n    ")
}
