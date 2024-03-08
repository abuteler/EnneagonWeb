use leptos::logging::log;
use crate::components::vectris::Shape;

pub trait ControlState {
  fn clear_coordinates (&self, shape: Shape);
  fn solidify_shape (&self, shape: Shape);
  fn next_cycle (&self);
  fn on_rotate (&self) {
    log!(" > rotate shape! <" );
  }
  fn on_move_down (&self) -> bool;
  fn on_move_left (&self);
  fn on_move_right (&self);
  fn on_free_dive (&self);
}