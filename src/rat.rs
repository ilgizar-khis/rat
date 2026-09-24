pub struct Rat {
    pos: [u16; 2],
    size: u16,
    field: [u16; 2],
}

impl Rat {
    pub fn new(pos: [u16; 2], size: u16, field: [u16; 2]) -> Self {
        Self { pos, size, field }
    }
}
