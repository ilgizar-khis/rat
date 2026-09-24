use std::{env, path::PathBuf, time::Duration};

use crossterm::event::{self, Event, KeyCode};

struct App {
    title: String,
}

impl App {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        ratatui::run(|term| {
            loop {
                term.draw(|frame| frame.render_widget(self.title.clone(), frame.area()))
                    .map_err(|e| e.to_string())?;
                if event::poll(Duration::from_millis(100)).map_err(|e| e.to_string())? {
                    if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
                        match key.code {
                            KeyCode::Char('q') => break Ok(()),
                            _ => {}
                        }
                    }
                }
            }
        })
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        return Err("You need to enter the name of the Lua file.".to_string());
    };

    let path_buf = PathBuf::from(path);

    let mut app = App::new(path);
    app.run()?;
    Ok(())
}
