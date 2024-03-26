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
    let num: usize = rand::thread_rng().gen_range(1..8);
    let c: usize = GRID_COLS / 2 - 1;
    match num {
      1 => Shape {
        class: Shapes::Square,
        cells: [
          Cell::new(0, c, Some(Color::Violet), CellState::Fluid),
          Cell::new(1, c, Some(Color::Violet), CellState::Fluid),
          Cell::new(0, c + 1, Some(Color::Violet), CellState::Fluid),
          Cell::new(1, c + 1, Some(Color::Violet), CellState::Fluid),
        ],
      },
      2 => Shape {
        class: Shapes::Pillar,
        cells: [
          Cell::new(0, c, Some(Color::Green), CellState::Fluid),
          Cell::new(1, c, Some(Color::Green), CellState::Fluid),
          Cell::new(2, c, Some(Color::Green), CellState::Fluid),
          Cell::new(3, c, Some(Color::Green), CellState::Fluid),
        ],
      },
      3 => Shape {
        class: Shapes::LeftL,
        cells: [
          Cell::new(0, c+1, Some(Color::Blue), CellState::Fluid),
          Cell::new(1, c+1, Some(Color::Blue), CellState::Fluid),
          Cell::new(2, c+1, Some(Color::Blue), CellState::Fluid),
          Cell::new(2, c, Some(Color::Blue), CellState::Fluid),
        ],
      },
      4 => Shape {
        class: Shapes::RightL,
        cells: [
          Cell::new(0, c, Some(Color::Orange), CellState::Fluid),
          Cell::new(1, c, Some(Color::Orange), CellState::Fluid),
          Cell::new(2, c, Some(Color::Orange), CellState::Fluid),
          Cell::new(2, c+1, Some(Color::Orange), CellState::Fluid),
        ],
      },
      5 => Shape {
        class: Shapes::LeftLightning,
        cells: [
          Cell::new(0, c+1, Some(Color::Crimson), CellState::Fluid),
          Cell::new(1, c+1, Some(Color::Crimson), CellState::Fluid),
          Cell::new(1, c, Some(Color::Crimson), CellState::Fluid),
          Cell::new(2, c, Some(Color::Crimson), CellState::Fluid),
        ],
      },
      6 => Shape {
        class: Shapes::RightLightning,
        cells: [
          Cell::new(0, c, Some(Color::LightBlue), CellState::Fluid),
          Cell::new(1, c, Some(Color::LightBlue), CellState::Fluid),
          Cell::new(1, c+1, Some(Color::LightBlue), CellState::Fluid),
          Cell::new(2, c+1, Some(Color::LightBlue), CellState::Fluid),
        ],
      },
      // in wildcard form to avoid compiler complaints.
      _ => Shape {
        class: Shapes::Tripod,
        cells: [
          Cell::new(0, c, Some(Color::LightSeaGreen), CellState::Fluid),
          Cell::new(1, c, Some(Color::LightSeaGreen), CellState::Fluid),
          Cell::new(2, c, Some(Color::LightSeaGreen), CellState::Fluid),
          Cell::new(1, c+1, Some(Color::LightSeaGreen), CellState::Fluid),
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
