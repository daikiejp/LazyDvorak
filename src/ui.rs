use crate::app::App;
use crate::keyboard::KeyboardRenderer;
use crate::types::{AppMode, KeyboardLayout};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),
            Constraint::Min(15),
            Constraint::Length(3),
        ])
        .split(f.size());

    render_top_panel(f, chunks[0], app);
    render_keyboard_panel(f, chunks[1], app);
    render_footer(f, chunks[2], app);
}

fn render_top_panel(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    render_left_panel(f, chunks[0], app);
    render_stats_panel(f, chunks[1], app);
}

fn render_left_panel(f: &mut Frame, area: Rect, app: &App) {
    let content = build_left_panel_content(app);
    let title = get_panel_title(app);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn build_left_panel_content(app: &App) -> Vec<Line<'_>> {
    match app.mode {
        AppMode::Menu => build_menu_content(app),
        AppMode::Settings => build_settings_content(app),
        AppMode::About => build_about_content(app),
        AppMode::CountSelection => build_count_selection_content(app),
        AppMode::WordsCommandsMenu
        | AppMode::WordsMenu
        | AppMode::CodeTestMenu
        | AppMode::SentencesMenu => build_submenu_content(app),
        _ => build_practice_content(app),
    }
}

fn build_menu_content(app: &App) -> Vec<Line<'_>> {
    let menu_items = vec![
        &app.translations.words_commands,
        &app.translations.sentence_practice,
        &app.translations.real_code_test,
        &app.translations.settings,
        &app.translations.about,
    ];

    let mut lines = vec![Line::from("")];
    for (i, item) in menu_items.iter().enumerate() {
        lines.push(Line::from(vec![
            Span::styled(
                if app.menu_selected == i {
                    " ❯ "
                } else {
                    "   "
                },
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(item.to_string()),
        ]));
    }
    lines
}

fn build_settings_content(app: &App) -> Vec<Line<'_>> {
    let current_lang = if app.translations.main_menu == "Main Menu" {
        "English"
    } else if app.translations.main_menu == "Menú Principal" {
        "Español"
    } else {
        "日本語"
    };

    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if app.submenu_selected == 0 {
                    " ❯ "
                } else {
                    "   "
                },
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!(
                "{}: {}",
                app.translations.keyboard_layout, app.current_layout_name
            )),
        ]),
        Line::from(vec![
            Span::styled(
                if app.submenu_selected == 1 {
                    " ❯ "
                } else {
                    "   "
                },
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!("{}: {}", app.translations.language, current_lang)),
        ]),
    ]
}

fn build_about_content(app: &App) -> Vec<Line<'_>> {
    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " LazyDvorak ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("v0.1.0", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
        Line::from(Span::raw(app.translations.about_text.as_str())),
        Line::from(""),
        Line::from(vec![
            Span::styled("GitHub: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "https://github.com/daikiejp/lazydvorak",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Made by: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "DaikieJP",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Website: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "https://daikie.jp",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ]),
    ]
}

fn build_submenu_content(app: &App) -> Vec<Line<'_>> {
    let items = if app.mode == AppMode::WordsCommandsMenu {
        vec![
            &app.translations.simple_words,
            &app.translations.vim_commands,
            &app.translations.words_by_language,
        ]
    } else if app.mode == AppMode::SentencesMenu {
        vec![
            &app.translations.sentences_normal,
            &app.translations.sentences_dvorak,
            &app.translations.sentences_qwerty,
        ]
    } else if app.mode == AppMode::WordsMenu || app.mode == AppMode::CodeTestMenu {
        vec![
            &app.translations.lua,
            &app.translations.ruby,
            &app.translations.typescript,
            &app.translations.rust,
            &app.translations.python,
        ]
    } else {
        vec![]
    };

    let mut lines = vec![Line::from("")];
    for (i, item) in items.iter().enumerate() {
        lines.push(Line::from(vec![
            Span::styled(
                if app.submenu_selected == i {
                    " ❯ "
                } else {
                    "   "
                },
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(item.to_string()),
        ]));
    }
    lines
}

fn build_count_selection_content(app: &App) -> Vec<Line<'_>> {
    let counts = vec!["All", "10", "25", "50", "100"];

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            &app.translations.select_count,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    for (i, count) in counts.iter().enumerate() {
        lines.push(Line::from(vec![
            Span::styled(
                if app.count_selected == i {
                    " ❯ "
                } else {
                    "   "
                },
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(count.to_string()),
        ]));
    }
    lines
}

fn build_practice_content(app: &App) -> Vec<Line<'_>> {
    let mut lines = vec![Line::from("")];

    if let Some(limit) = app.exercise_count.to_usize() {
        lines.push(Line::from(vec![Span::styled(
            format!("Exercise: {}/{} ", app.exercises_completed + 1, limit),
            Style::default().fg(Color::Magenta),
        )]));
        lines.push(Line::from(""));
    }

    let mut target_spans = vec![Span::styled(
        format!("{}: ", app.translations.target),
        Style::default().fg(Color::Gray),
    )];

    for (i, ch) in app.target_text.chars().enumerate() {
        let display_char = format_special_char(ch);
        let style = if i < app.typed_text.len() {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::DIM)
        } else if i == app.current_key_index {
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };
        target_spans.push(Span::styled(display_char, style));
    }
    lines.push(Line::from(target_spans));
    lines.push(Line::from(""));

    let mut typed_spans = vec![Span::styled(
        format!("{}: ", app.translations.typed),
        Style::default().fg(Color::Gray),
    )];

    for ch in app.typed_text.chars() {
        let display_char = format_special_char(ch);
        typed_spans.push(Span::styled(
            display_char,
            Style::default().fg(Color::Green),
        ));
    }
    lines.push(Line::from(typed_spans));

    lines
}

fn format_special_char(ch: char) -> String {
    match ch {
        '\n' => "↵".to_string(),
        '\t' => "⇥".to_string(),
        ' ' => "·".to_string(),
        _ => ch.to_string(),
    }
}

fn get_panel_title(app: &App) -> String {
    match app.mode {
        AppMode::Menu => format!(" LazyDvorak - {} ", app.translations.main_menu),
        AppMode::Settings => app.translations.settings.clone(),
        AppMode::About => app.translations.about.clone(),
        AppMode::CountSelection => app.translations.select_count.clone(),
        AppMode::WordsCommandsMenu
        | AppMode::WordsMenu
        | AppMode::CodeTestMenu
        | AppMode::SentencesMenu => app.translations.select_language_label.clone(),
        _ => app.translations.practice.clone(),
    }
}

fn render_stats_panel(f: &mut Frame, area: Rect, app: &App) {
    let wpm = app.stats.calculate_wpm();
    let accuracy = app.stats.calculate_accuracy();

    let content = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("✓ {}: ", app.translations.correct),
                Style::default().fg(Color::Green),
            ),
            Span::raw(format!("{}", app.stats.correct)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("✗ {}: ", app.translations.errors),
                Style::default().fg(Color::Red),
            ),
            Span::raw(format!("{}", app.stats.errors)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("📊 {}: ", app.translations.accuracy),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(format!("{:.1}%", accuracy)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("⚡ {}: ", app.translations.wpm),
                Style::default().fg(Color::Magenta),
            ),
            Span::raw(format!("{:.1}", wpm)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("{}: ", app.translations.layout),
                Style::default().fg(Color::Gray),
            ),
            Span::styled(&app.current_layout_name, Style::default().fg(Color::Green)),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(app.translations.statistics.as_str())
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, area);
}

fn render_keyboard_panel(f: &mut Frame, area: Rect, app: &App) {
    let mut modifiers_str = String::new();
    if app.shift_pressed {
        modifiers_str.push_str(&format!(" [{}]", app.translations.shift));
    }
    if app.ctrl_pressed {
        modifiers_str.push_str(&format!(" [{}]", app.translations.ctrl));
    }
    if app.alt_pressed {
        modifiers_str.push_str(&format!(" [{}]", app.translations.alt));
    }

    let keyboard_lines = match app.keyboard_layout {
        KeyboardLayout::Dvorak => {
            KeyboardRenderer::render_dvorak(app.shift_pressed, &app.last_pressed_key)
        }
        KeyboardLayout::Qwerty => {
            KeyboardRenderer::render_qwerty(app.shift_pressed, &app.last_pressed_key)
        }
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!(
            "{} - {}{}",
            app.translations.keyboard, app.current_layout_name, modifiers_str
        ))
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));

    let paragraph = Paragraph::new(keyboard_lines)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let instructions = match app.mode {
        AppMode::Menu => &app.translations.nav_menu,
        AppMode::Settings
        | AppMode::About
        | AppMode::CountSelection
        | AppMode::WordsCommandsMenu
        | AppMode::WordsMenu
        | AppMode::CodeTestMenu
        | AppMode::SentencesMenu => &app.translations.nav_submenu,
        _ => &app.translations.nav_practice,
    };

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_footer = Paragraph::new(Line::from(vec![Span::styled(
        instructions,
        Style::default().fg(Color::DarkGray),
    )]))
    .alignment(Alignment::Left);

    let right_content = vec![
        Span::styled("GitHub: ", Style::default().fg(Color::DarkGray)),
        Span::styled("LazyDvorak", Style::default().fg(Color::DarkGray)),
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled("by DaikieJP", Style::default().fg(Color::DarkGray)),
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled("v0.1.0", Style::default().fg(Color::DarkGray)),
    ];

    let right_footer = Paragraph::new(Line::from(right_content)).alignment(Alignment::Right);

    f.render_widget(left_footer, footer_chunks[0]);
    f.render_widget(right_footer, footer_chunks[1]);
}
