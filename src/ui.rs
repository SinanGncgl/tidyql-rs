use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as SyntectStyle, ThemeSet};
use syntect::parsing::SyntaxSet;

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(frame.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(23), // (folders and files)
                Constraint::Percentage(77), // (file content)
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let file_items: Vec<ListItem> = if app.is_searching {
        app.search_results
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let style = if i == app.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(Color::Blue)
                        .add_modifier(Modifier::BOLD | Modifier::ITALIC)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(file.to_string_lossy().to_string()).style(style)
            })
            .collect()
    } else {
        app.files
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let style = if i == app.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(Color::Blue)
                        .add_modifier(Modifier::BOLD | Modifier::ITALIC)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(file.to_string_lossy().to_string()).style(style)
            })
            .collect()
    };

    let folders_files = List::new(file_items).block(
        Block::default()
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(format!("Browse: {}", app.current_dir.display()))
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
    );
    frame.render_widget(folders_files, top_chunks[0]);

    // Syntax highlighting for SQL content
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("sql").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    let highlighted_sql: Vec<Line> = app
        .file_content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let ranges: Vec<(SyntectStyle, &str)> = h.highlight_line(line, &ps).unwrap();
            let spans: Vec<Span> = ranges
                .into_iter()
                .map(|(style, text)| {
                    Span::styled(
                        text.to_string(),
                        Style::default().fg(Color::Rgb(
                            style.foreground.r,
                            style.foreground.g,
                            style.foreground.b,
                        )),
                    )
                })
                .collect();
            let line_number = Span::styled(
                format!("{:4} ", i + 1),
                Style::default().fg(Color::DarkGray),
            );
            let mut line_with_number = vec![line_number];
            line_with_number.extend(spans);
            Line::from(line_with_number)
        })
        .collect();

    let diff_sql: Vec<Line> = app
        .diff_content
        .as_deref()
        .unwrap_or("")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let style = if line.starts_with('+') {
                Style::default().fg(Color::Green)
            } else if line.starts_with('-') {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::White)
            };
            let line_number = Span::styled(
                format!("{:4} ", i + 1),
                Style::default().fg(Color::DarkGray),
            );
            let line_with_number = vec![line_number, Span::styled(line.to_string(), style)];
            Line::from(line_with_number)
        })
        .collect();

    let content_text = if app.diff_content.is_some() {
        Text::from(diff_sql)
    } else {
        Text::from(highlighted_sql)
    };

    let content = Paragraph::new(content_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title("File Content")
                .title_alignment(Alignment::Center)
                .title_style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left); // Ensure left alignment for better readability
    frame.render_widget(content, top_chunks[1]);

    let commands_text =
        "Commands: q - Quit | f - Format SQL | s - Save | / - Search | → - Enter Folder | ← - Back";

    let commands = Paragraph::new(commands_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title("Commands")
                .title_alignment(Alignment::Center)
                .title_style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);
    frame.render_widget(commands, chunks[1]);

    // Notification bubble at the top right
    if let Some(ref notification) = app.notification {
        let notification_area = Rect::new(
            frame.area().width.saturating_sub(30), // Adjust width as needed
            0,
            30,
            3,
        );
        let notification_paragraph = Paragraph::new(notification.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title("Notification")
                    .title_alignment(Alignment::Center)
                    .title_style(
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center);
        frame.render_widget(notification_paragraph, notification_area);
    }

    // Search popup
    if app.is_searching {
        let search_area = Rect::new(
            (frame.area().width - 50) / 2,
            (frame.area().height - 3) / 2,
            50,
            3,
        );
        let search_text = format!("Search: {}", app.search_query);
        let search_paragraph = Paragraph::new(search_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title("Search")
                    .title_alignment(Alignment::Center)
                    .title_style(
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left);
        frame.render_widget(search_paragraph, search_area);
    }
}
