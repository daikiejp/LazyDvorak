pub mod code;
pub mod sentences;
pub mod vim;
pub mod words;

pub use vim::get_vim_commands;

pub use words::lua::get_lua_words;
pub use words::python::get_python_words;
pub use words::ruby::get_ruby_words;
pub use words::rust::get_rust_words;
pub use words::simple::get_simple_words;
pub use words::typescript::get_typescript_words;

pub use sentences::dvorak::get_sentences_dvorak;
pub use sentences::normal::get_sentences_normal;
pub use sentences::qwerty::get_sentences_qwerty;

pub fn get_code_tests(lang: &str) -> Vec<String> {
    match lang {
        "lua" => code::lua::get_lua_code_tests(),
        "ruby" => code::ruby::get_ruby_code_tests(),
        "rust" => code::rust::get_rust_code_tests(),
        "typescript" => code::typescript::get_typescript_code_tests(),
        "python" => code::python::get_python_code_tests(),
        _ => vec![],
    }
}
