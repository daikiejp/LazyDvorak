use crate::exercises::*;
use crate::stats::Stats;
use crate::translations::Translations;
use crate::types::{AppMode, ExerciseCount, KeyboardLayout, PressedKey};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::Rng;
use std::time::Instant;

pub struct App {
    pub mode: AppMode,
    pub keyboard_layout: KeyboardLayout,
    pub current_layout_name: String,
    pub stats: Stats,
    pub target_text: String,
    pub typed_text: String,
    pub current_key_index: usize,
    pub last_pressed_key: Option<PressedKey>,
    pub menu_selected: usize,
    pub submenu_selected: usize,
    pub count_selected: usize,
    pub pending_mode: Option<AppMode>,
    pub nvim_keymaps: Vec<String>,
    pub shift_pressed: bool,
    pub ctrl_pressed: bool,
    pub alt_pressed: bool,
    pub translations: Translations,
    pub exercise_count: ExerciseCount,
    pub exercises_completed: usize,
}

impl App {
    pub fn new(lang: String, layout: String) -> Self {
        let keyboard_layout = match layout.as_str() {
            "qwerty" => KeyboardLayout::Qwerty,
            _ => KeyboardLayout::Dvorak,
        };

        let current_layout_name = match keyboard_layout {
            KeyboardLayout::Dvorak => "Dvorak Programmer",
            KeyboardLayout::Qwerty => "QWERTY",
        }
        .to_string();

        Self {
            mode: AppMode::Menu,
            keyboard_layout,
            current_layout_name,
            stats: Stats::new(),
            target_text: String::new(),
            typed_text: String::new(),
            current_key_index: 0,
            last_pressed_key: None,
            menu_selected: 0,
            submenu_selected: 0,
            count_selected: 0,
            pending_mode: None,
            nvim_keymaps: Vec::new(),
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            translations: Translations::new(&lang),
            exercise_count: ExerciseCount::All,
            exercises_completed: 0,
        }
    }

    pub fn load_nvim_keymaps(&mut self) {
        if let Ok(content) = std::fs::read_to_string("/tmp/lazy-dvorak-keymaps.json") {
            if let Ok(keymaps) = serde_json::from_str::<Vec<String>>(&content) {
                self.nvim_keymaps = keymaps;
            }
        }

        if self.nvim_keymaps.is_empty() {
            self.nvim_keymaps = vec![
                "<leader>ff".to_string(),
                "<C-n>".to_string(),
                "<Space>b".to_string(),
                "<C-w>v".to_string(),
                "<A-j>".to_string(),
            ];
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        self.shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
        self.ctrl_pressed = key.modifiers.contains(KeyModifiers::CONTROL);
        self.alt_pressed = key.modifiers.contains(KeyModifiers::ALT);

        let display = self.create_key_display(&key);
        if !display.is_empty() {
            self.last_pressed_key = Some(PressedKey::new(display));
        }

        match self.mode {
            AppMode::Menu => self.handle_menu_key(key),
            AppMode::Settings => self.handle_settings_key(key),
            AppMode::About => self.handle_about_key(key),
            AppMode::CountSelection => self.handle_count_selection_key(key),
            AppMode::WordsCommandsMenu
            | AppMode::WordsMenu
            | AppMode::CodeTestMenu
            | AppMode::SentencesMenu => self.handle_submenu_key(key),
            _ => self.handle_practice_key(key),
        }
    }

    fn create_key_display(&self, key: &KeyEvent) -> String {
        match key.code {
            KeyCode::Char(c) => {
                let mut mods = Vec::new();
                if self.ctrl_pressed {
                    mods.push("C");
                }
                if self.alt_pressed {
                    mods.push("A");
                }
                if self.shift_pressed && !c.is_uppercase() {
                    mods.push("S");
                }

                if mods.is_empty() {
                    c.to_string()
                } else {
                    format!("<{}-{}>", mods.join("-"), c)
                }
            }
            KeyCode::Backspace => "⌫".to_string(),
            KeyCode::Enter => "↵".to_string(),
            KeyCode::Tab => "⇥".to_string(),
            KeyCode::Esc => "ESC".to_string(),
            KeyCode::Up => "↑".to_string(),
            KeyCode::Down => "↓".to_string(),
            KeyCode::Left => "←".to_string(),
            KeyCode::Right => "→".to_string(),
            _ => String::new(),
        }
    }

    fn handle_menu_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Up if self.menu_selected > 0 => self.menu_selected -= 1,
            KeyCode::Down if self.menu_selected < 4 => self.menu_selected += 1,
            KeyCode::Enter => match self.menu_selected {
                0 => {
                    self.mode = AppMode::WordsCommandsMenu;
                    self.submenu_selected = 0;
                }
                1 => {
                    self.mode = AppMode::SentencesMenu;
                    self.submenu_selected = 0;
                }
                2 => {
                    self.mode = AppMode::CodeTestMenu;
                    self.submenu_selected = 0;
                }
                3 => {
                    self.mode = AppMode::Settings;
                    self.submenu_selected = 0;
                }
                4 => {
                    self.mode = AppMode::About;
                }
                5 => return false,
                _ => {}
            },
            _ => {}
        }
        true
    }

    fn handle_settings_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => self.mode = AppMode::Menu,
            KeyCode::Up if self.submenu_selected > 0 => self.submenu_selected -= 1,
            KeyCode::Down if self.submenu_selected < 1 => self.submenu_selected += 1,
            KeyCode::Left | KeyCode::Right | KeyCode::Enter => match self.submenu_selected {
                0 => {
                    self.keyboard_layout = match self.keyboard_layout {
                        KeyboardLayout::Dvorak => KeyboardLayout::Qwerty,
                        KeyboardLayout::Qwerty => KeyboardLayout::Dvorak,
                    };
                    self.current_layout_name = match self.keyboard_layout {
                        KeyboardLayout::Dvorak => "Dvorak Programmer",
                        KeyboardLayout::Qwerty => "QWERTY",
                    }
                    .to_string();
                }
                1 => {
                    let new_lang = if self.translations.main_menu == "Main Menu" {
                        "es"
                    } else if self.translations.main_menu == "Menú Principal" {
                        "ja"
                    } else {
                        "en"
                    };
                    self.translations = Translations::new(new_lang);
                }
                _ => {}
            },
            _ => {}
        }
        true
    }

    fn handle_about_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => self.mode = AppMode::Menu,
            _ => {}
        }
        true
    }

    fn handle_submenu_key(&mut self, key: KeyEvent) -> bool {
        let max_items = match self.mode {
            AppMode::WordsCommandsMenu => 2,
            AppMode::SentencesMenu => 2,
            _ => 4,
        };

        match key.code {
            KeyCode::Esc => self.mode = AppMode::Menu,
            KeyCode::Up if self.submenu_selected > 0 => self.submenu_selected -= 1,
            KeyCode::Down if self.submenu_selected < max_items => self.submenu_selected += 1,
            KeyCode::Enter => {
                if self.mode == AppMode::WordsCommandsMenu {
                    match self.submenu_selected {
                        0 => self.show_count_selection(AppMode::WordsSimple),
                        1 => self.show_count_selection(AppMode::VimCommands),
                        2 => {
                            self.mode = AppMode::WordsMenu;
                            self.submenu_selected = 0;
                        }
                        _ => {}
                    }
                } else if self.mode == AppMode::SentencesMenu {
                    let target_mode = match self.submenu_selected {
                        0 => AppMode::SentencesNormal,
                        1 => AppMode::SentencesDvorak,
                        2 => AppMode::SentencesQwerty,
                        _ => AppMode::SentencesNormal,
                    };
                    self.show_count_selection(target_mode);
                } else if self.mode == AppMode::WordsMenu {
                    let target_mode = match self.submenu_selected {
                        0 => AppMode::WordsLua,
                        1 => AppMode::WordsRuby,
                        2 => AppMode::WordsTypescript,
                        3 => AppMode::WordsRust,
                        4 => AppMode::WordsPython,
                        _ => AppMode::WordsLua,
                    };
                    self.show_count_selection(target_mode);
                } else if self.mode == AppMode::CodeTestMenu {
                    let target_mode = match self.submenu_selected {
                        0 => AppMode::CodeTestLua,
                        1 => AppMode::CodeTestRuby,
                        2 => AppMode::CodeTestTypescript,
                        3 => AppMode::CodeTestRust,
                        4 => AppMode::CodeTestPython,
                        _ => AppMode::CodeTestLua,
                    };
                    self.show_count_selection(target_mode);
                }
            }
            _ => {}
        }
        true
    }

    fn show_count_selection(&mut self, target_mode: AppMode) {
        self.pending_mode = Some(target_mode);
        self.mode = AppMode::CountSelection;
        self.count_selected = 0;
    }

    fn handle_count_selection_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.mode = AppMode::Menu;
                self.pending_mode = None;
            }
            KeyCode::Up if self.count_selected > 0 => self.count_selected -= 1,
            KeyCode::Down if self.count_selected < 4 => self.count_selected += 1,
            KeyCode::Enter => {
                self.exercise_count = match self.count_selected {
                    0 => ExerciseCount::All,
                    1 => ExerciseCount::Count10,
                    2 => ExerciseCount::Count25,
                    3 => ExerciseCount::Count50,
                    4 => ExerciseCount::Count100,
                    _ => ExerciseCount::All,
                };

                if let Some(target) = self.pending_mode.take() {
                    self.exercises_completed = 0;
                    self.mode = target;
                    self.start_exercise();
                }
            }
            _ => {}
        }
        true
    }

    fn handle_practice_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.mode = AppMode::Menu;
                self.stats.reset();
                self.last_pressed_key = None;
                self.exercises_completed = 0;
                return true;
            }
            KeyCode::Char(c) => {
                self.process_typed_char(c);
            }
            KeyCode::Backspace => {
                if !self.typed_text.is_empty() {
                    self.typed_text.pop();
                    if self.current_key_index > 0 {
                        self.current_key_index -= 1;
                    }
                }
            }
            KeyCode::Enter => {
                self.process_typed_char('\n');
            }
            KeyCode::Tab => {
                self.process_typed_char('\t');
            }
            _ => {}
        }
        true
    }

    fn process_typed_char(&mut self, c: char) {
        if self.stats.start_time.is_none() {
            self.stats.start_time = Some(Instant::now());
        }

        if self.target_text.is_empty() {
            return;
        }

        let expected = self.target_text.chars().nth(self.current_key_index);

        if let Some(expected_char) = expected {
            if c == expected_char {
                self.typed_text.push(c);
                self.current_key_index += 1;
                self.stats.correct += 1;

                if self.current_key_index >= self.target_text.len() {
                    self.stats.total_chars += self.target_text.len() as u32;
                    self.exercises_completed += 1;

                    if let Some(limit) = self.exercise_count.to_usize() {
                        if self.exercises_completed >= limit {
                            self.mode = AppMode::Menu;
                            return;
                        }
                    }

                    self.next_exercise();
                }
            } else {
                self.stats.errors += 1;
            }
        }
    }

    pub fn start_exercise(&mut self) {
        let words = match self.mode {
            AppMode::VimCommands => get_vim_commands(),
            AppMode::WordsSimple => get_simple_words(),
            AppMode::WordsLua => get_lua_words(),
            AppMode::WordsRuby => get_ruby_words(),
            AppMode::WordsRust => get_rust_words(),
            AppMode::WordsTypescript => get_typescript_words(),
            AppMode::WordsPython => get_python_words(),
            AppMode::SentencesNormal => get_sentences_normal(),
            AppMode::SentencesDvorak => get_sentences_dvorak(),
            AppMode::SentencesQwerty => get_sentences_qwerty(),
            AppMode::CodeTestLua => get_code_tests("lua"),
            AppMode::CodeTestRuby => get_code_tests("ruby"),
            AppMode::CodeTestRust => get_code_tests("rust"),
            AppMode::CodeTestTypescript => get_code_tests("typescript"),
            AppMode::CodeTestPython => get_code_tests("python"),
            AppMode::CustomKeymaps => self.nvim_keymaps.clone(),
            _ => vec![],
        };

        if !words.is_empty() {
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0..words.len());
            self.target_text = words[idx].clone();
        }

        self.typed_text.clear();
        self.current_key_index = 0;
    }

    fn next_exercise(&mut self) {
        self.start_exercise();
    }
}
