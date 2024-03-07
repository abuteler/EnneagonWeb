use leptos::logging::log;
use crate::components::vectris::Shape;

pub trait ControlState {
  fn clear_coordinates (&self, shape: Shape);
  fn solidify_shape (&self, shape: Shape);
  fn next_cycle (&self);
  fn on_key_up (&self) {
    log!(" > rotate shape! <" );
  }
  fn on_key_down (&self);
  fn on_key_left (&self);
  fn on_key_right (&self);
  fn on_key_free_dive (&self);
}