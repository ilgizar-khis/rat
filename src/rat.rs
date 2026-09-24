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
}
