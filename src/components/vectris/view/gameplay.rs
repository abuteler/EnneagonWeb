use leptos::*;
use super::{GameMenu, GameCanvas};
use leptos::leptos_dom::ev::{keydown, KeyboardEvent};

#[component]
pub fn GamePlay() -> impl IntoView {
  // set listeners for game controls
  let (key_pressed, set_key_pressed) = create_signal(String::new());
  let handler = window_event_listener(keydown, move |ev: KeyboardEvent| {
      let key = &ev.key_code();
      set_key_pressed.set(key.to_string());
      // match key {
      //     37 => vectris.on_key_left(),
      //     38 => vectris.on_key_up(),
      //     39 => vectris.on_key_right(),
      //     40 => vectris.on_key_down(),
      //     _ => {}
      // }
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
