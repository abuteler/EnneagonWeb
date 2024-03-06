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
    pub coordinates: (usize, usize),
    pub color: Option<Color>,
}

impl Cell {
    pub fn new(x: usize, y: usize, color: Option<Color>) -> Self {
        Self {
            coordinates: (x, y),
            color,
        }
    }
}
