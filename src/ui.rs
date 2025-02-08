use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style as SyntectStyle};
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
                Constraint::Percentage(20), // (folders and files)
                Constraint::Percentage(80), // (file content)
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let file_items: Vec<ListItem> = app
        .files
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
        .collect();

    let folders_files = List::new(file_items).block(
        Block::default()
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Browse")
            .title_alignment(Alignment::Center)
            .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
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
        .map(|line| {
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
            Line::from(spans)
        })
        .collect();

    let diff_sql: Vec<Line> = app
        .diff_content
        .as_deref()
        .unwrap_or("")
        .lines()
        .map(|line| {
            let style = if line.starts_with('+') {
                Style::default().fg(Color::Green)
            } else if line.starts_with('-') {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::White)
            };
            Line::from(Span::styled(line.to_string(), style))
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
                .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left); // Ensure left alignment for better readability
    frame.render_widget(content, top_chunks[1]);

    let commands_text = "Commands: q - Quit | f - Format SQL | s - Save";

    let commands = Paragraph::new(commands_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title("Commands")
                .title_alignment(Alignment::Center)
                .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
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
                    .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center);
        frame.render_widget(notification_paragraph, notification_area);
    }
}