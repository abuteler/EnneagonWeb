
use leptos::{logging::log, *};

use super::{shape, Cell, CellState, Shape, Shapes, Status, GRID_COLS, GRID_ROWS};
use super::super::ControlState;

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
        m[r][c] = create_rw_signal(Cell::new(c, r, None, CellState::Empty));
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
    self.current_shape.set(self.next_shape.get());
    self.next_shape.set(Shape::new());
  }
  fn on_key_left (&self) {
    log!(" > move left! <" );
  }
  fn on_key_right (&self) {
    log!(" > move right! <" );
  }
  fn on_key_up (&self) {
    let mut shape = self.current_shape.get();
    if shape.class != Shapes::Square {
      // log!(" > rotate: {:?}", shape);
      // step one: take the second cell's coordinates as my origin
      let (origin_x, origin_y) = shape.cells[1].coordinates;
      for cell in shape.cells.iter_mut() {
        // use of auxiliary signed coordinates b/c linear transformation will do (-y, x)
        let (mut signed_x, mut signed_y): (isize, isize);
        // step two: offset all cells by the new "origin" coordinates
        signed_x = cell.coordinates.0 as isize - origin_x as isize;
        signed_y = cell.coordinates.1 as isize - origin_y as isize;
        // note that cell #2 will end up being (0,0)
        if cell.coordinates != (0,0) {
          // step three: apply the Linear Transformation (-y, x), which rotates 90 degrees counter clockwise
          // See http://en.wikipedia.org/wiki/Transformation_matrix#Examples_in_2D_graphics
          (signed_x, signed_y) = (-signed_y, signed_x);
        }
        // step four: take back to their original coordinates in the matrix
        cell.coordinates.0 = (signed_x + origin_x as isize) as usize;
        cell.coordinates.1 = (signed_y + origin_y as isize) as usize;
      }
      // TODO: check for collisions!
      self.clear_coordinates(self.current_shape.get());
      self.current_shape.set(shape);

    }
  }
  fn on_key_down (&self) {
    log!(" > down! <" );
    let floor = GRID_ROWS;
    let mut shape = self.current_shape.get();
    let mut grounded = false;

    for cell in shape.cells.iter_mut() {
      let (col, row) = cell.coordinates;
      if (row+1 == floor || self.matrix[row+1][col].get().state == CellState::Solid) {
        grounded = true;
        break;
      }
      cell.coordinates.1 +=1;
    }
    if grounded {
      self.next_cycle()
    } else {
      self.clear_coordinates(self.current_shape.get());
      self.current_shape.set(shape);  
    };
  }
}
