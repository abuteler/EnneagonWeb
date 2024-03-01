use super::{Cell, Color, GRID_COLS};

#[derive(Copy, Clone, Debug, Default)]
pub struct Shape {
    pub cells: [Cell; 4],
}

impl Shape {
    pub fn new() -> Self {
        use rand::prelude::*;
        // randomize a number between 1 and 7
        let num: u8 = rand::thread_rng().gen_range(1..7);
        let x: u8 = GRID_COLS / 2;
        match num {
            // Square
            1 => Shape {
                cells: [
                    Cell::new(x, 0, Some(Color::Violet)),
                    Cell::new(x, 1, Some(Color::Violet)),
                    Cell::new(x + 1, 0, Some(Color::Violet)),
                    Cell::new(x + 1, 1, Some(Color::Violet)),
                ],
            },
            // Column
            2 => Shape {
                cells: [
                    Cell::new(x, 0, Some(Color::Green)),
                    Cell::new(x, 1, Some(Color::Green)),
                    Cell::new(x, 2, Some(Color::Green)),
                    Cell::new(x, 3, Some(Color::Green)),
                ],
            },
            // Left L
            3 => Shape {
                cells: [
                    Cell::new(x+1, 0, Some(Color::Blue)),
                    Cell::new(x+1, 1, Some(Color::Blue)),
                    Cell::new(x+1, 2, Some(Color::Blue)),
                    Cell::new(x, 2, Some(Color::Blue)),
                ],
            },
            // Right L
            4 => Shape {
                cells: [
                    Cell::new(x, 0, Some(Color::Yellow)),
                    Cell::new(x, 1, Some(Color::Yellow)),
                    Cell::new(x, 2, Some(Color::Yellow)),
                    Cell::new(x+1, 2, Some(Color::Yellow)),
                ],
            },
            // Left "lightning"
            5 => Shape {
                cells: [
                    Cell::new(x+1, 0, Some(Color::Red)),
                    Cell::new(x+1, 1, Some(Color::Red)),
                    Cell::new(x, 1, Some(Color::Red)),
                    Cell::new(x, 2, Some(Color::Red)),
                ],
            },
            // Right "lightning"
            6 => Shape {
                cells: [
                    Cell::new(x, 0, Some(Color::LightBlue)),
                    Cell::new(x, 1, Some(Color::LightBlue)),
                    Cell::new(x+1, 1, Some(Color::LightBlue)),
                    Cell::new(x+1, 2, Some(Color::LightBlue)),
                ],
            },
            // Tripod; num = 7; wildcard to avoid compiler complaints.
            _ => Shape { 
                cells: [
                    Cell::new(x, 0, Some(Color::Pink)),
                    Cell::new(x, 1, Some(Color::Pink)),
                    Cell::new(x, 2, Some(Color::Pink)),
                    Cell::new(x+1, 1, Some(Color::Pink)),
                ],
            }
        }
    }
}
