use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

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

    let file_content = Paragraph::new(app.file_content.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title("File Content")
                .title_alignment(Alignment::Center)
                .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left); // Ensure left alignment for better readability
    frame.render_widget(file_content, top_chunks[1]);

    let commands = Paragraph::new("Commands: q - Quit | f - Format SQL | s - Save")
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
            frame.area().width.saturating_sub(30),
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