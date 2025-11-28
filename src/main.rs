mod app;
mod stats;
mod ui;

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode},
};

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new(".");
    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => app.quit(),
                KeyCode::Char('r') => app.refresh("."),
                _ => {}
            }
        }
        if app.should_quit() {
            break;
        }
    }
    Ok(())
}