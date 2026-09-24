#[derive(Clone)]
pub enum RatActions {
    MoveDir(String),
    MoveTo([u16; 2]),
    Nothing,
}
