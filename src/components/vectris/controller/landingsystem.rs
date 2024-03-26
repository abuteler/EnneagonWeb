use crate::components::vectris::Shape;

#[derive(Clone, Debug)]
pub struct Outcome {
  pub rows_to_burn: Vec<usize>
}

pub trait Landingable {
  fn clear_coordinates (&self, shape: Shape);
  fn solidify_shape (&self, shape: Shape);
  fn process_landing (&self) -> Outcome;
  fn initialize_next_cycle (&self);
  fn execute_outcome (&self, outcome: Outcome);
}
