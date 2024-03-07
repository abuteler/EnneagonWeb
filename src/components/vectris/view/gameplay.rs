use leptos::*;

use super::{GameMenu, GameCanvas};
use super::super::{GameState, ControlState};
use leptos::leptos_dom::ev::{keydown, KeyboardEvent};

#[component]
pub fn GamePlay() -> impl IntoView {
  let state = expect_context::<GameState>();
  // set listeners for game controls
  let (key_pressed, set_key_pressed) = create_signal(String::new());
  let handler = window_event_listener(keydown, move |ev: KeyboardEvent| {
    let key = &ev.key_code();
    set_key_pressed.set(key.to_string());
    match key {
      // arrows
      38 => state.on_key_up(),
      37 => state.on_key_left(),
      39 => state.on_key_right(),
      40 => state.on_key_down(),
      // wasd
      87 => state.on_key_up(),
      65 => state.on_key_left(),
      68 => state.on_key_right(),
      83 => state.on_key_down(),
      // TODO: add space bar to free dive
      _ => {}
    };
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
