use leptos::*;
use leptos::leptos_dom::ev::{keydown, KeyboardEvent};
use leptos_use::{use_interval_fn, use_timeout_fn, UseTimeoutFnReturn, utils::Pausable};
use crate::components::vectris::{GameCanvas, GameMenu, GameState, Landingable, Navigateable, Outcome, Scores, Status, UpNext};

#[component]
pub fn GamePlay() -> impl IntoView {
  let state = expect_context::<GameState>();
  let status = state.status;
  let flag_landed = state.flag_landed;
  let gravity_interval = state.gravity_interval;
  // initialize gravity interval (with a signal for difficulty increase as score increases)
  let Pausable { is_active, pause, resume } = use_interval_fn(
    move || {state.on_move_down();},
    gravity_interval
  );
  // set listeners for game controls
  let (key_pressed, set_key_pressed) = create_signal(String::new());
  let controls_handle = window_event_listener(keydown, move |ev: KeyboardEvent| {
    let key = &ev.key_code();
    // only bother if key is valid input
    if [19, 32, 37, 38, 39, 40, 65, 68, 80, 83, 87].contains(key) {
      if is_active.get() {
        set_key_pressed.set(key.to_string());
        match key {
          // arrows
          37 => state.on_move_left(),
          38 => state.on_rotate(),
          39 => state.on_move_right(),
          40 => {state.on_move_down();()},
          // wasd
          65 => state.on_move_left(),
          68 => state.on_move_right(),
          83 => {state.on_move_down();()},
          87 => state.on_rotate(),
          // space bar to free dive
          32 => state.on_free_dive(),
          // Pause key
          19 => if is_active.get() {status.set(Status::Paused)} else {status.set(Status::Playing)}
          // P
          80 => if is_active.get() {status.set(Status::Paused)} else {status.set(Status::Playing)}
          // _ catch all
          _ => {}
        };
      }
    }
  });
  // subscribe to the eventual landing of a shape
  create_effect(move |_| {
    if flag_landed.get() {
      let outcome = state.process_landing();
      // process affected lines -> complete? flag for xplosion -> delay -> execute xplosion -> update matrix -> resume
      if !outcome.rows_to_burn.is_empty() {
        let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
          move |_| {
              let o = outcome.clone();
              state.execute_outcome(o);
          },
          100.0
        );
        start(());
      }
    }
  });
  // subscribe to game status changes and bind to gravity interval
  create_effect(move |_|{
    let status = status.get();
    match status {
      Status::Paused => { if is_active.get() { pause() } },
      Status::Playing => { if !is_active.get() { resume() } },
      Status::GameOver => {
        /* TODO */
        if is_active.get() { pause() };
      },
      _ => {},
    }
  });
  // bind removal of window listener on leptos component cleanup
  on_cleanup(move || controls_handle.remove());

  view! {
    <section id="gameplay-container">
      <GameMenu />
      <div class="flex gap-6">
        <GameCanvas />
        <section id="right-panel" class="flex flex-col gap-6">
          <UpNext />
          <Scores />
        </section>
      </div>
      <p>Pressed key code: <span class="text-sky-400">{ move || key_pressed.get() }</span></p>
    </section>
  }
}
