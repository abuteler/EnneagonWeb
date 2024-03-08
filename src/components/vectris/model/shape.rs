use super::{Cell, CellState, Color, GRID_COLS};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shapes {
  Square,
  Pillar,
  LeftL,
  RightL,
  LeftLightning,
  RightLightning,
  Tripod,
}

#[derive(Copy, Clone, Debug)]
pub struct Shape {
  pub class: Shapes,
  pub cells: [Cell; 4],
}

impl Shape {
  pub fn new() -> Self {
    use rand::prelude::*;
    // randomize a number between 1 and 7
    let num: usize = rand::thread_rng().gen_range(1..7);
    let x: usize = GRID_COLS / 2 - 1;
    match num {
      1 => Shape {
        class: Shapes::Square,
        cells: [
          Cell::new(x, 0, Some(Color::Violet), CellState::Fluid),
          Cell::new(x, 1, Some(Color::Violet), CellState::Fluid),
          Cell::new(x + 1, 0, Some(Color::Violet), CellState::Fluid),
          Cell::new(x + 1, 1, Some(Color::Violet), CellState::Fluid),
        ],
      },
      2 => Shape {
        class: Shapes::Pillar,
        cells: [
          Cell::new(x, 0, Some(Color::Green), CellState::Fluid),
          Cell::new(x, 1, Some(Color::Green), CellState::Fluid),
          Cell::new(x, 2, Some(Color::Green), CellState::Fluid),
          Cell::new(x, 3, Some(Color::Green), CellState::Fluid),
        ],
      },
      3 => Shape {
        class: Shapes::LeftL,
        cells: [
          Cell::new(x+1, 0, Some(Color::Blue), CellState::Fluid),
          Cell::new(x+1, 1, Some(Color::Blue), CellState::Fluid),
          Cell::new(x+1, 2, Some(Color::Blue), CellState::Fluid),
          Cell::new(x, 2, Some(Color::Blue), CellState::Fluid),
        ],
      },
      4 => Shape {
        class: Shapes::RightL,
        cells: [
          Cell::new(x, 0, Some(Color::Yellow), CellState::Fluid),
          Cell::new(x, 1, Some(Color::Yellow), CellState::Fluid),
          Cell::new(x, 2, Some(Color::Yellow), CellState::Fluid),
          Cell::new(x+1, 2, Some(Color::Yellow), CellState::Fluid),
        ],
      },
      5 => Shape {
        class: Shapes::LeftLightning,
        cells: [
          Cell::new(x+1, 0, Some(Color::Red), CellState::Fluid),
          Cell::new(x+1, 1, Some(Color::Red), CellState::Fluid),
          Cell::new(x, 1, Some(Color::Red), CellState::Fluid),
          Cell::new(x, 2, Some(Color::Red), CellState::Fluid),
        ],
      },
      6 => Shape {
        class: Shapes::RightLightning,
        cells: [
          Cell::new(x, 0, Some(Color::LightBlue), CellState::Fluid),
          Cell::new(x, 1, Some(Color::LightBlue), CellState::Fluid),
          Cell::new(x+1, 1, Some(Color::LightBlue), CellState::Fluid),
          Cell::new(x+1, 2, Some(Color::LightBlue), CellState::Fluid),
        ],
      },
      // in wildcard form to avoid compiler complaints.
      _ => Shape {
        class: Shapes::Tripod,
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
