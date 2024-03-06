use leptos::logging::log;
pub trait Controls {
  fn on_key_up (&self) {
    log!(" > rotate shape! <" );
  }
  fn on_key_down (&self);
  fn on_key_left (&self);
  fn on_key_right (&self);
}