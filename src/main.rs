use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::HorizontalAlignment::Center;
use ratatui::layout::Layout;
use ratatui::widgets::{Block, Borders, Paragraph};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

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
                term.draw(|frame| self.render(frame))
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

    fn render(&self, frame: &mut Frame) {
        let [head, body] = Layout::vertical([Length(2), Fill(1)]).areas(frame.area());

        let title_block = Block::new().borders(Borders::BOTTOM);
        let title_bar = Paragraph::new(self.title.clone())
            .block(title_block)
            .alignment(Center);

        frame.render_widget(title_bar, head);
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
