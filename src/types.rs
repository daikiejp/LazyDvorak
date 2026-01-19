use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Menu,
    Settings,
    About,
    WordsCommandsMenu,
    VimCommands,
    WordsMenu,
    WordsSimple,
    WordsLua,
    WordsRuby,
    WordsRust,
    WordsTypescript,
    WordsPython,
    SentencesMenu,
    SentencesNormal,
    SentencesDvorak,
    SentencesQwerty,
    CodeTestMenu,
    CodeTestLua,
    CodeTestRuby,
    CodeTestRust,
    CodeTestTypescript,
    CodeTestPython,
    CustomKeymaps,
    CountSelection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardLayout {
    Dvorak,
    Qwerty,
}

#[derive(Debug, Clone)]
pub struct PressedKey {
    pub display: String,
    pub timestamp: Instant,
}

impl PressedKey {
    pub fn new(display: String) -> Self {
        Self {
            display,
            timestamp: Instant::now(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.timestamp.elapsed().as_millis() <= 200
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExerciseCount {
    All,
    Count10,
    Count25,
    Count50,
    Count100,
}

impl ExerciseCount {
    pub fn to_usize(&self) -> Option<usize> {
        match self {
            ExerciseCount::All => None,
            ExerciseCount::Count10 => Some(10),
            ExerciseCount::Count25 => Some(25),
            ExerciseCount::Count50 => Some(50),
            ExerciseCount::Count100 => Some(100),
        }
    }

    pub fn display(&self) -> &str {
        match self {
            ExerciseCount::All => "All",
            ExerciseCount::Count10 => "10",
            ExerciseCount::Count25 => "25",
            ExerciseCount::Count50 => "50",
            ExerciseCount::Count100 => "100",
        }
    }
}
