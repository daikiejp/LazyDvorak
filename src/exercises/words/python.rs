pub fn get_python_words() -> Vec<String> {
    vec![
        "def", "class", "if", "elif", "else", "for", "while", "break", "continue", "return",
        "yield", "import", "from", "as", "try", "except", "finally", "raise", "with", "lambda",
        "pass", "assert", "global", "nonlocal", "del", "True", "False", "None",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
