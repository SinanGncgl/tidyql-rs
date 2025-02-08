use anyhow::Result;
use log::error;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::io;

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
            if app.is_searching {
                match key.code {
                    KeyCode::Char(c) => {
                        app.search_query.push(c);
                        app.search_files()?;
                    }
                    KeyCode::Backspace => {
                        app.search_query.pop();
                        app.search_files()?;
                    }
                    KeyCode::Enter => {
                        app.is_searching = false;
                        if !app.search_results.is_empty() {
                            app.files = app.search_results.clone();
                            app.selected_index = 0;
                            app.select_file(0)?;
                        }
                    }
                    KeyCode::Esc => {
                        app.is_searching = false;
                    }
                    _ => {}
                }
            } else {
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
                    KeyCode::Char('/') => {
                        app.is_searching = true;
                        app.search_query.clear();
                    }
                    KeyCode::Right => {
                        app.navigate_into_folder()?;
                    }
                    KeyCode::Left => {
                        app.navigate_back()?;
                    }
                    _ => {}
                }
            }
        }
    }
}
