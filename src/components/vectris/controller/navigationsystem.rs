pub trait Navigateable {
  fn on_rotate (&self);
  fn on_move_left (&self);
  fn on_move_right (&self);
  fn on_move_down (&self) -> bool; // boolean to break the fall on a free dive
  fn on_free_dive (&self);
}
