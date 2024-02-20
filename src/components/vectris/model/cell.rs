#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Violet,
    Green,
    Blue,
    Yellow,
    Red,
    LightBlue,
    Pink,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Cell {
    pub coordinates: (u8, u8),
    pub filled: bool,
    pub color: Option<Color>,
}

impl Cell {
    pub fn new(x: u8, y: u8, filled: bool, color: Option<Color>) -> Self {
        Self {
            coordinates: (x, y),
            filled,
            color,
        }
    }
}
