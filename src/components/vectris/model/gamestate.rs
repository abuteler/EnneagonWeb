
use leptos::{create_rw_signal, RwSignal};

use super::{Shape, Status, Cell, GRID_COLS, GRID_ROWS};

#[derive(Copy, Clone, Debug)]
pub struct GameState {
    pub burned_lines: RwSignal<u32>,
    pub current_shape: RwSignal<Shape>,
    pub next_shape: RwSignal<Shape>,
    pub status: RwSignal<Status>,
    pub matrix: [[RwSignal<Cell>; GRID_COLS]; GRID_ROWS],
}

impl GameState {
    pub fn new() -> Self {
        let mut m: [[RwSignal<Cell>; GRID_COLS]; GRID_ROWS] = Default::default();
        for r in 0..GRID_ROWS-1 {
            for c in 0..GRID_COLS-1 {
                m[r][c] = create_rw_signal(Cell::new(r, c, None));
            }
        };
        Self {
            burned_lines: create_rw_signal(0),
            current_shape: create_rw_signal(Shape::default()),
            next_shape: create_rw_signal(Shape::default()),
            status: create_rw_signal(Status::InMenus),
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