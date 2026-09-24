use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::HorizontalAlignment::Center;
use ratatui::layout::Layout;
use ratatui::widgets::{Block, Borders, Paragraph};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

use actions::RatActions;
use rat::Rat;

mod actions;
mod rat;

struct App {
    title: String,
    rats: HashMap<String, Rat>,
    last_id: usize,
}

impl App {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            rats: HashMap::new(),
            last_id: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        ratatui::run(|term| {
            loop {
                term.draw(|frame| self.render(frame))
                    .map_err(|e| e.to_string())?;

                for rat in self.rats.values_mut() {
                    rat.next_step();
                }

                if event::poll(Duration::from_millis(100)).map_err(|e| e.to_string())? {
                    if self.key_handle()? {
                        break Ok(());
                    }
                }
            }
        })
    }

    fn key_handle(&mut self) -> Result<bool, String> {
        if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    pub fn add_rat(&mut self, rat: Rat) -> String {
        self.last_id += 1;
        let id = format!("rat #{}", self.last_id);
        self.rats.insert(id.clone(), rat);
        id
    }

    fn render(&self, frame: &mut Frame) {
        let [head, body] = Layout::vertical([Length(2), Fill(1)]).areas(frame.area());

        let title_block = Block::new().borders(Borders::BOTTOM);
        let title_bar = Paragraph::new(self.title.clone())
            .block(title_block)
            .alignment(Center);

        for rat in self.rats.values() {
            let rect = rat.calc(body);
            let block = rat.get_block();
            frame.render_widget(block, rect);
        }

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

    let mut rat = Rat::new([0, 0], 1, [30, 30]);
    rat.add_action(RatActions::MoveTo([10, 10]), 1);
    rat.add_action(RatActions::MoveDir("right".to_string()), 10);
    let id = app.add_rat(rat);
    app.run()?;
    Ok(())
}
