
use leptos::*;
use super::{Cell, CellState, Shape, Shapes, Status, GRID_COLS, GRID_ROWS};
use super::super::ControlState;

#[derive(Copy, Clone, Debug)]
pub struct GameState {
  pub burned_lines: RwSignal<u32>,
  pub current_shape: RwSignal<Shape>,
  pub next_shape: RwSignal<Shape>,
  pub status: RwSignal<Status>,
  pub matrix: [[RwSignal<Cell>; GRID_COLS]; GRID_ROWS],
  pub neoize: RwSignal<bool>,
}

impl GameState {
  pub fn new() -> Self {
    let mut m: [[RwSignal<Cell>; GRID_COLS]; GRID_ROWS] = Default::default();
    for r in 0..GRID_ROWS-1 {
      for c in 0..GRID_COLS-1 {
        m[r][c] = create_rw_signal(Cell::new(c, r, None, CellState::Empty));
      }
    };
    Self {
      burned_lines: create_rw_signal(0),
      current_shape: create_rw_signal(Shape::default()),
      next_shape: create_rw_signal(Shape::default()),
      status: create_rw_signal(Status::InMenus),
      matrix: m,
      neoize: create_rw_signal(false),
    }
  }
}

impl Default for GameState {
  fn default() -> Self {
    Self::new()
  }
}

impl ControlState for GameState {
  fn clear_coordinates(&self, shape: Shape) {
    for cell in shape.cells.iter() {
      let (c,r) = cell.coordinates;
      self.matrix[r][c].update(move |cell| {
        cell.color = None;
        cell.state = CellState::Empty;
      });
    }
  }
  fn solidify_shape (&self, shape: Shape) {
    for cell in shape.cells.iter() {
      let (c,r) = cell.coordinates;
      self.matrix[r][c].update(move |cell| {
        cell.state = CellState::Solid;
      });
    }
  }
  fn next_cycle (&self) {
    self.solidify_shape(self.current_shape.get());
    // TODO: check for complete lines, explode them, update score
    self.current_shape.set(self.next_shape.get());
    self.next_shape.set(Shape::new());
  }
  fn on_move_left (&self) {
    let wall = 0;
    let mut shape = self.current_shape.get();
    let mut walled = false;
    for cell in shape.cells.iter_mut() {
      let (c, r) = cell.coordinates;
      if c == wall || self.matrix[r][c-1].get().state == CellState::Solid {
        walled = true;
        break;
      }
      cell.coordinates.0 -= 1;
    }
    if !walled {
      self.clear_coordinates(self.current_shape.get());
      self.current_shape.set(shape);
    }
  }
  fn on_move_right (&self) {
    let wall = GRID_COLS;
    let mut shape = self.current_shape.get();
    let mut walled = false;
    for cell in shape.cells.iter_mut() {
      let (c, r) = cell.coordinates;
      if c+1 == wall || self.matrix[r][c+1].get().state == CellState::Solid {
        walled = true;
        break;
      }
      cell.coordinates.0 += 1;
    }
    if !walled {
      self.clear_coordinates(self.current_shape.get());
      self.current_shape.set(shape);
    }
  }
  fn on_rotate (&self) {
    let mut shape = self.current_shape.get();
    let mut illegal_rotation = false;
    if shape.class != Shapes::Square {
      // step one: take the second cell's coordinates as my origin
      let (origin_c, origin_r) = shape.cells[1].coordinates;
      for cell in shape.cells.iter_mut() {
        // use of auxiliary signed coordinates b/c linear transformation will do (-y, x)
        let (mut signed_c, mut signed_r): (isize, isize);
        // step two: offset all cells by the new "origin" coordinates
        signed_c = cell.coordinates.0 as isize - origin_c as isize;
        signed_r = cell.coordinates.1 as isize - origin_r as isize;
        // note that cell #2 will end up being (0,0)
        if cell.coordinates != (0,0) {
          // step three: apply the Linear Transformation (-y, x), which rotates 90 degrees counter clockwise
          // See http://en.wikipedia.org/wiki/Transformation_matrix#Examples_in_2D_graphics
          (signed_c, signed_r) = (-signed_r, signed_c);
        }
        // step four: return to original coordinates in the matrix
        cell.coordinates.0 = (signed_c + origin_c as isize) as usize;
        cell.coordinates.1 = (signed_r + origin_r as isize) as usize;
        // step five: check for collisions before committing!
        if cell.coordinates.0 > GRID_COLS-1 || cell.coordinates.1 > GRID_ROWS-1 {
          // out of bounds
          illegal_rotation = true;
          break;
        }
        if self.matrix[cell.coordinates.1][cell.coordinates.0].get().state == CellState::Solid {
          // trampling is not allowed
          illegal_rotation = true;
          break;
        }
      }
      if !illegal_rotation {
        self.clear_coordinates(self.current_shape.get());
        self.current_shape.set(shape);
      }
    }
  }
  fn on_move_down (&self) -> bool {
    let floor = GRID_ROWS;
    let mut shape = self.current_shape.get();
    let mut already_grounded = false;
    for cell in shape.cells.iter_mut() {
      let (c, r) = cell.coordinates;
      if r+1 == floor || self.matrix[r+1][c].get().state == CellState::Solid {
        already_grounded = true;
        break;
      }
      cell.coordinates.1 += 1;
    }
    if already_grounded {
      self.next_cycle();
    } else {
      self.clear_coordinates(self.current_shape.get());
      self.current_shape.set(shape);
    }
    already_grounded
  }
  fn on_free_dive (&self) {
    loop {
      let grounded = self.on_move_down();
      if grounded { break; }
    }
  }
}
