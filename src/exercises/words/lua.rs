pub fn get_lua_words() -> Vec<String> {
    vec![
        "function", "local", "end", "if", "then", "else", "elseif", "for", "while", "do", "repeat",
        "until", "return", "break", "nil", "true", "false", "and", "or", "not", "require", "pairs",
        "ipairs", "table", "string", "math", "print", "type",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
