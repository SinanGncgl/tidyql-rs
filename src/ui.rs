use ratatui::{
    layout::{Constraint, Direction, Layout},
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
                    .add_modifier(Modifier::BOLD)
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
            .style(Style::default().fg(Color::White))
            .borders(Borders::ALL)
            .title("Browse")
            .title_alignment(Alignment::Center)
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
                .title("File Content")
                .title_alignment(Alignment::Center),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(file_content, top_chunks[1]);
}
    