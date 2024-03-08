use leptos::*;
use leptos_use::{use_interval_fn, utils::Pausable};
use crate::components::vectris::Status;

use super::{GameMenu, GameCanvas};
use super::super::{GameState, ControlState};
use leptos::leptos_dom::ev::{keydown, KeyboardEvent};

#[component]
pub fn GamePlay() -> impl IntoView {
  let state = expect_context::<GameState>();
  let status = state.status;

  // initialize gravity interval (with a signal for difficulty increase as score increases)
  let (interval, set_interval) = create_signal(800);
  let Pausable { is_active, pause, resume } = use_interval_fn(move || {state.on_move_down();}, interval.get());

  // set game status change behavior
  create_effect(move |_|{
    let status = status.get();
    match status {
      Status::Paused => { if is_active.get() {pause()} },
      Status::Playing => { if !is_active.get() {resume()} },
      Status::GameOver => { /* TODO */},
      _ => {},
    }
  });

  // set listeners for game controls
  let (key_pressed, set_key_pressed) = create_signal(String::new());
  let handler = window_event_listener(keydown, move |ev: KeyboardEvent| {
    let key = &ev.key_code();
    // only bother if key is valid input
    if [32, 37, 38, 39, 40, 65, 68, 83, 87].contains(key) {
      set_key_pressed.set(key.to_string());
      match key {
        // arrows
        37 => state.on_move_left(),
        38 => state.on_rotate(),
        39 => state.on_move_right(),
        40 => {state.on_move_down(); ()},
        // wasd
        65 => state.on_move_left(),
        68 => state.on_move_right(),
        83 => {state.on_move_down(); ()},
        87 => state.on_rotate(),
        // space bar to free dive
        32 => state.on_free_dive(),
        _ => {}
      };
    }
  });

  on_cleanup(move || handler.remove());
  view! {
    <section id="game-canvas">
      <GameMenu />
      <GameCanvas />
      <p>Pressed key code: <span class="text-sky-400">{ move || key_pressed.get() }</span></p>
    </section>
  }
}
