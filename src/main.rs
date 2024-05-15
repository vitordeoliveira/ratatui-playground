use std::{error::Error, io::stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::ui;

mod ui;

#[derive(Debug, Default)]
pub struct App {
    left_size: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::default();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => app.left_size += 1,
                    KeyCode::Left => {
                        if app.left_size > 0 {
                            app.left_size -= 1
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // cleanup
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
