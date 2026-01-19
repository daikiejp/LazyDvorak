pub fn get_rust_words() -> Vec<String> {
    vec![
        "fn", "let", "mut", "const", "static", "struct", "enum", "impl", "trait", "pub", "use",
        "mod", "crate", "if", "else", "match", "loop", "while", "for", "in", "return", "break",
        "continue", "as", "type", "where", "async", "await", "move",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
