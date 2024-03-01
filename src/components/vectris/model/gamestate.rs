
use super::{Shape, Status, Cell, GRID_COLS, GRID_ROWS};

#[derive(Copy, Clone, Debug)]
pub struct GameState {
    pub burned_lines: u32,
    pub current_shape: Shape,
    pub next_shape: Shape,
    pub status: Status,
    pub matrix: [[Cell; GRID_COLS as usize]; GRID_ROWS as usize],
}

impl GameState {
    pub fn new() -> Self {
        let mut m: [[Cell; GRID_COLS as usize]; GRID_ROWS as usize] = Default::default();
        for r in 0..GRID_ROWS-1 {
            for c in 0..GRID_COLS-1 {
                m[r as usize][c as usize] = Cell::new(r, c, None);
            }
        };
        Self {
            burned_lines: 0,
            current_shape: Shape::default(),
            next_shape: Shape::default(),
            status: Status::InMenus,
            matrix: m
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

// impl Controls for GameState {}