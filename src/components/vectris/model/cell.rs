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
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum CellState {
    Empty, #[default]
    Solid,
    Fluid,
    Exploding,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Cell {
    pub coordinates: (usize, usize),
    pub color: Option<Color>,
    pub state: CellState
}

impl Cell {
    pub fn new(x: usize, y: usize, color: Option<Color>, state: CellState) -> Self {
        Self {
            coordinates: (x, y),
            color,
            state,
        }
    }
}
