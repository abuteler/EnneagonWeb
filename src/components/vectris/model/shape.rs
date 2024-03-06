use super::{Cell, CellState, Color, GRID_COLS};

#[derive(Copy, Clone, Debug)]
pub struct Shape {
    pub cells: [Cell; 4],
}

impl Shape {
    pub fn new() -> Self {
        use rand::prelude::*;
        // randomize a number between 1 and 7
        let num: usize = rand::thread_rng().gen_range(1..7);
        let x: usize = GRID_COLS / 2 - 1;
        match num {
            // Square
            1 => Shape {
                cells: [
                    Cell::new(x, 0, Some(Color::Violet), CellState::Fluid),
                    Cell::new(x, 1, Some(Color::Violet), CellState::Fluid),
                    Cell::new(x + 1, 0, Some(Color::Violet), CellState::Fluid),
                    Cell::new(x + 1, 1, Some(Color::Violet), CellState::Fluid),
                ],
            },
            // Column
            2 => Shape {
                cells: [
                    Cell::new(x, 0, Some(Color::Green), CellState::Fluid),
                    Cell::new(x, 1, Some(Color::Green), CellState::Fluid),
                    Cell::new(x, 2, Some(Color::Green), CellState::Fluid),
                    Cell::new(x, 3, Some(Color::Green), CellState::Fluid),
                ],
            },
            // Left L
            3 => Shape {
                cells: [
                    Cell::new(x+1, 0, Some(Color::Blue), CellState::Fluid),
                    Cell::new(x+1, 1, Some(Color::Blue), CellState::Fluid),
                    Cell::new(x+1, 2, Some(Color::Blue), CellState::Fluid),
                    Cell::new(x, 2, Some(Color::Blue), CellState::Fluid),
                ],
            },
            // Right L
            4 => Shape {
                cells: [
                    Cell::new(x, 0, Some(Color::Yellow), CellState::Fluid),
                    Cell::new(x, 1, Some(Color::Yellow), CellState::Fluid),
                    Cell::new(x, 2, Some(Color::Yellow), CellState::Fluid),
                    Cell::new(x+1, 2, Some(Color::Yellow), CellState::Fluid),
                ],
            },
            // Left "lightning"
            5 => Shape {
                cells: [
                    Cell::new(x+1, 0, Some(Color::Red), CellState::Fluid),
                    Cell::new(x+1, 1, Some(Color::Red), CellState::Fluid),
                    Cell::new(x, 1, Some(Color::Red), CellState::Fluid),
                    Cell::new(x, 2, Some(Color::Red), CellState::Fluid),
                ],
            },
            // Right "lightning"
            6 => Shape {
                cells: [
                    Cell::new(x, 0, Some(Color::LightBlue), CellState::Fluid),
                    Cell::new(x, 1, Some(Color::LightBlue), CellState::Fluid),
                    Cell::new(x+1, 1, Some(Color::LightBlue), CellState::Fluid),
                    Cell::new(x+1, 2, Some(Color::LightBlue), CellState::Fluid),
                ],
            },
            // Tripod; num = 7; wildcard to avoid compiler complaints.
            _ => Shape { 
                cells: [
                    Cell::new(x, 0, Some(Color::Pink), CellState::Fluid),
                    Cell::new(x, 1, Some(Color::Pink), CellState::Fluid),
                    Cell::new(x, 2, Some(Color::Pink), CellState::Fluid),
                    Cell::new(x+1, 1, Some(Color::Pink), CellState::Fluid),
                ],
            }
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::new()
    }
}
