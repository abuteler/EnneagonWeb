
use super::{Shape, Status, Grid, CELLS_PER_ROW, CELLS_PER_COL};

#[derive(Copy, Clone, Debug, Default)]
pub struct GameState {
    pub burned_lines: u32,
    pub current_shape: Shape,
    pub next_shape: Shape,
    pub status: Status,
    pub grid: Grid,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            burned_lines: 0,
            current_shape: Shape::default(),
            next_shape: Shape::default(),
            status: Status::InMenus,
            grid: Grid::new(CELLS_PER_ROW, CELLS_PER_COL),
        }
    }
}

// impl Controls for GameState {}