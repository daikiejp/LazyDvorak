use crate::types::PressedKey;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub struct KeyboardRenderer;

impl KeyboardRenderer {
    pub fn render_dvorak(shifted: bool, pressed_key: &Option<PressedKey>) -> Vec<Line<'static>> {
        if shifted {
            Self::build_shifted_dvorak(pressed_key)
        } else {
            Self::build_normal_dvorak(pressed_key)
        }
    }

    pub fn render_qwerty(shifted: bool, pressed_key: &Option<PressedKey>) -> Vec<Line<'static>> {
        if shifted {
            Self::build_shifted_qwerty(pressed_key)
        } else {
            Self::build_normal_qwerty(pressed_key)
        }
    }

    fn build_normal_dvorak(pressed_key: &Option<PressedKey>) -> Vec<Line<'static>> {
        vec![
            Line::from(""),
            Line::from(
                "  ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────────┐",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "`", "$", "&", "[", "{", "}", "(", "=", "*", ")", "+", "]", "!", "⌫",
                ],
                &[4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 10],
            ),
            Line::from(
                "  ├────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Tab", ";", ",", ".", "p", "y", "f", "g", "c", "r", "l", "/", "@", "\\",
                ],
                &[7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
            ),
            Line::from(
                "  ├───────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴─────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Caps", "a", "o", "e", "u", "i", "d", "h", "t", "n", "s", "-", "Enter",
                ],
                &[8, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 9],
            ),
            Line::from(
                "  ├────────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴─────────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Shift", "'", "q", "j", "k", "x", "b", "m", "w", "v", "z", "Shift",
                ],
                &[11, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 11],
            ),
            Line::from(
                "  ├──────┬────┴─┬──┴────┴────┴────┴────┴────┴────┴────┴─┬──┴──┬─┴────┬──────┤",
            ),
            Self::build_modifier_row(pressed_key),
            Line::from(
                "  └──────┴──────┴───────────────────────────────────────┴─────┴──────┴──────┘",
            ),
        ]
    }

    fn build_shifted_dvorak(pressed_key: &Option<PressedKey>) -> Vec<Line<'static>> {
        vec![
            Line::from(""),
            Line::from(
                "  ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────────┐",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "~", "%", "7", "5", "3", "1", "9", "0", "2", "4", "6", "8", "`", "⌫",
                ],
                &[4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 10],
            ),
            Line::from(
                "  ├────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Tab", ":", "<", ">", "P", "Y", "F", "G", "C", "R", "L", "?", "^", "|",
                ],
                &[7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
            ),
            Line::from(
                "  ├───────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴─────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Caps", "A", "O", "E", "U", "I", "D", "H", "T", "N", "S", "_", "Enter",
                ],
                &[8, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 9],
            ),
            Line::from(
                "  ├────────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴─────────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Shift", "\"", "Q", "J", "K", "X", "B", "M", "W", "V", "Z", "Shift",
                ],
                &[11, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 11],
            ),
            Line::from(
                "  ├──────┬────┴─┬──┴────┴────┴────┴────┴────┴────┴────┴─┬──┴──┬─┴────┬──────┤",
            ),
            Self::build_modifier_row(pressed_key),
            Line::from(
                "  └──────┴──────┴───────────────────────────────────────┴─────┴──────┴──────┘",
            ),
        ]
    }

    fn build_normal_qwerty(pressed_key: &Option<PressedKey>) -> Vec<Line<'static>> {
        vec![
            Line::from(""),
            Line::from(
                "  ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────────┐",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "⌫",
                ],
                &[4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 10],
            ),
            Line::from(
                "  ├────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Tab", "q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "[", "]", "\\",
                ],
                &[7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
            ),
            Line::from(
                "  ├───────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴─────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Caps", "a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'", "Enter",
                ],
                &[8, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 9],
            ),
            Line::from(
                "  ├────────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴─────────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Shift", "z", "x", "c", "v", "b", "n", "m", ",", ".", "/", "Shift",
                ],
                &[11, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 11],
            ),
            Line::from(
                "  ├──────┬────┴─┬──┴────┴────┴────┴────┴────┴────┴────┴─┬──┴──┬─┴────┬──────┤",
            ),
            Self::build_modifier_row(pressed_key),
            Line::from(
                "  └──────┴──────┴───────────────────────────────────────┴─────┴──────┴──────┘",
            ),
        ]
    }

    fn build_shifted_qwerty(pressed_key: &Option<PressedKey>) -> Vec<Line<'static>> {
        vec![
            Line::from(""),
            Line::from(
                "  ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────────┐",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "~", "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+", "⌫",
                ],
                &[4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 10],
            ),
            Line::from(
                "  ├────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "{", "}", "|",
                ],
                &[7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
            ),
            Line::from(
                "  ├───────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴─────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Caps", "A", "S", "D", "F", "G", "H", "J", "K", "L", ":", "\"", "Enter",
                ],
                &[8, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 9],
            ),
            Line::from(
                "  ├────────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬───────────┤",
            ),
            Self::build_row(
                pressed_key,
                vec![
                    "Shift", "Z", "X", "C", "V", "B", "N", "M", "<", ">", "?", "Shift",
                ],
                &[11, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 11],
            ),
            Line::from(
                "  ├──────┬────┴─┬──┴────┴────┴────┴────┴────┴────┴────┴─┬──┴──┬─┴────┬──────┤",
            ),
            Self::build_modifier_row(pressed_key),
            Line::from(
                "  └──────┴──────┴───────────────────────────────────────┴─────┴──────┴──────┘",
            ),
        ]
    }

    fn build_modifier_row(pressed_key: &Option<PressedKey>) -> Line<'static> {
        let mut spans = vec![Span::raw("  │")];

        let is_mac = cfg!(target_os = "macos");

        let modifiers = if is_mac {
            vec![
                ("Ctrl", 6),
                ("Opt", 6),
                ("Space", 39),
                ("Cmd", 5),
                ("Opt", 6),
                ("Ctrl", 6),
            ]
        } else {
            vec![
                ("Ctrl", 6),
                ("Alt", 6),
                ("Space", 39),
                ("Alt", 5),
                ("Fn", 6),
                ("Ctrl", 6),
            ]
        };

        for (key, width) in modifiers {
            let is_pressed = if let Some(pk) = pressed_key {
                pk.is_active() && Self::modifier_matches(&pk.display, key)
            } else {
                false
            };

            let style = if is_pressed {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let padding = if width > key.len() {
                (width - key.len()) / 2
            } else {
                0
            };

            let padded_key = format!(
                "{}{}{}",
                " ".repeat(padding),
                key,
                " ".repeat(width - key.len() - padding)
            );

            spans.push(Span::styled(padded_key, style));
            spans.push(Span::raw("│"));
        }

        Line::from(spans)
    }

    fn build_row(
        pressed_key: &Option<PressedKey>,
        keys: Vec<&str>,
        widths: &[usize],
    ) -> Line<'static> {
        let mut spans = vec![Span::raw("  │")];

        for (i, key) in keys.iter().enumerate() {
            let is_pressed = if let Some(pk) = pressed_key {
                pk.is_active() && Self::key_matches(&pk.display, key)
            } else {
                false
            };

            let style = if is_pressed {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let width = widths.get(i).copied().unwrap_or(4);
            let padding = if width > key.len() {
                (width - key.len()) / 2
            } else {
                0
            };

            let padded_key = format!(
                "{}{}{}",
                " ".repeat(padding),
                key,
                " ".repeat(width - key.len() - padding)
            );

            spans.push(Span::styled(padded_key, style));
            spans.push(Span::raw("│"));
        }

        Line::from(spans)
    }

    fn key_matches(pressed: &str, key: &str) -> bool {
        let pressed_lower = pressed.to_lowercase();

        let special_keys = ["tab", "caps", "enter", "shift", "backspace", "⌫"];
        let key_lower = key.to_lowercase();

        if special_keys.contains(&key_lower.as_str()) {
            return pressed_lower.contains(&key_lower);
        }

        if pressed.contains('<') && pressed.contains('>') {
            if let Some(char_part) = pressed.split('-').last() {
                let clean_char = char_part.trim_end_matches('>');
                if clean_char.len() == 1 && key.len() == 1 {
                    return clean_char.eq_ignore_ascii_case(key);
                }
            }
        }

        if key.len() == 1 && pressed.len() == 1 {
            return pressed.eq_ignore_ascii_case(key);
        }

        pressed == key
    }

    fn modifier_matches(pressed: &str, modifier: &str) -> bool {
        let pressed_lower = pressed.to_lowercase();
        let modifier_lower = modifier.to_lowercase();

        match modifier_lower.as_str() {
            "ctrl" => pressed_lower.contains("ctrl") || pressed_lower.contains("<c-"),
            "alt" | "opt" => {
                pressed_lower.contains("alt")
                    || pressed_lower.contains("opt")
                    || pressed_lower.contains("<a-")
                    || pressed_lower.contains("<m-")
            }
            "cmd" | "command" => {
                pressed_lower.contains("cmd")
                    || pressed_lower.contains("command")
                    || pressed_lower.contains("<d-")
            }
            "shift" => pressed_lower.contains("shift") || pressed_lower.contains("<s-"),
            "fn" => pressed_lower.contains("fn"),
            "space" => pressed_lower.contains("space") || pressed == " ",
            _ => false,
        }
    }
}
