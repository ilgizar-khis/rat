use ratatui::layout::Rect;

pub struct Rat {
    pos: [u16; 2],
    size: u16,
    field: [u16; 2],
}

impl Rat {
    pub fn new(pos: [u16; 2], size: u16, field: [u16; 2]) -> Self {
        Self {
            pos: [pos[0] * 2, pos[1]],
            size,
            field: [field[0] * 2, field[1]],
        }
    }

    fn get_width(&self) -> u16 {
        self.size * 2
    }

    fn get_height(&self) -> u16 {
        self.size
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
