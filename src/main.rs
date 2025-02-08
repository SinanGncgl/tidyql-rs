use anyhow::Result;
use log::error;
use env_logger;
use std::io;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;

fn main() -> Result<()> {
    env_logger::init();
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new()?;
    let res = run_app(&mut terminal, &mut app);

    if let Err(e) = res {
        error!("Error: {}", e);
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut app::App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {
                    if app.selected_index < app.files.len() - 1 {
                        app.selected_index += 1;
                        app.select_file(app.selected_index)?;
                    }
                }
                KeyCode::Up => {
                    if app.selected_index > 0 {
                        app.selected_index -= 1;
                        app.select_file(app.selected_index)?;
                    }
                }
                KeyCode::Char('f') => {
                    app.format_sql()?;
                }
                KeyCode::Char('s') => {
                    app.save_formatted_file()?;
                }
                _ => {}
            }
        }
    }
}