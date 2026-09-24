use ratatui::style::Color;
use ratatui::{layout::Rect, style::Stylize, widgets::Block};

use crate::actions::RatActions;

pub struct Rat {
    pos: [u16; 2],
    size: u16,
    field: [u16; 2],
    color: Color,
    actions: Vec<RatActions>,
}

fn gen_color(color: String) -> Option<Color> {
    if color.starts_with("#") {
        if let Ok(hex) = u32::from_str_radix(&color[1..], 16) {
            return Some(Color::Rgb(
                ((hex >> 16) & 0xFF) as u8,
                ((hex >> 8) & 0xFF) as u8,
                (hex & 0xFF) as u8,
            ));
        }
    } else {
        if let Ok(color) = color.parse() {
            return Some(color);
        }
    }

    None
}

impl Rat {
    pub fn new(pos: [u16; 2], size: u16, field: [u16; 2]) -> Self {
        Self {
            pos: [pos[0] * 2, pos[1]],
            size,
            field: [field[0] * 2, field[1]],
            color: Color::White,
            actions: vec![RatActions::Nothing; 10],
        }
    }

    fn get_width(&self) -> u16 {
        self.size * 2
    }

    pub fn add_action(&mut self, action: RatActions, times: usize) {
        for _ in 0..times {
            self.actions.push(action.clone());
        }
    }

    fn get_height(&self) -> u16 {
        self.size
    }

    pub fn get_block(&self) -> Block<'_> {
        Block::default().bg(self.color.clone())
    }

    fn move_dir(&mut self, dir: String) {
        match dir.as_str() {
            "right" => {
                if self.pos[0] + 2 < self.field[0] {
                    self.pos[0] += 2;
                }
            }
            "left" => self.pos[0] = self.pos[0].saturating_sub(2),
            "down" => {
                if self.pos[1] + 1 < self.field[1] {
                    self.pos[1] += 1;
                }
            }
            "up" => self.pos[1] = self.pos[1].saturating_sub(1),
            _ => {}
        }
    }

    fn move_to(&mut self, new_pos: [u16; 2]) {
        if new_pos[0] * 2 > self.field[0] || new_pos[1] > self.field[1] {
            return;
        }

        self.pos[0] = new_pos[0] * 2;
        self.pos[1] = new_pos[1];
    }

    fn set_color(&mut self, color: String) {
        if let Some(color) = gen_color(color) {
            self.color = color
        }
    }

    pub fn next_step(&mut self) {
        let Some(action) = self.actions.get(0) else {
            return;
        };

        match action.clone() {
            RatActions::MoveDir(dir) => self.move_dir(dir),
            RatActions::MoveTo(pos) => self.move_to(pos),
            RatActions::SetColor(color) => self.set_color(color),
            RatActions::Nothing => {}
        }

        self.actions.remove(0);
    }

    pub fn calc(&self, area: Rect) -> Rect {
        Rect {
            x: area.x + self.pos[0],
            y: area.y + self.pos[1],
            width: self.get_width(),
            height: self.get_height(),
        }
    }
}
