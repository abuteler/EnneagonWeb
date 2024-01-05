mod model;
mod view;
mod controller;
pub use model::*;
pub use view::*;
// pub use controller::Controls;

use leptos::*;
use leptos::leptos_dom::{
    ev::{keydown, KeyboardEvent},
    helpers::window_event_listener,
};

pub const CELLS_PER_ROW: u8 = 10;
pub const CELLS_PER_COL: u8 = 16;

#[component]
pub fn Vectris() -> impl IntoView {
    provide_context(create_rw_signal(GameState::default()));
    // let (count, set_count) = create_signal(0);

    let state = expect_context::<RwSignal<GameState>>();
    let status = create_read_slice(state, |state| state.status);
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
      <main class="relative flex min-h-[575px] h-screen flex-col items-center justify-evenly p-8 md:p-24">
        <div class="relative flex flex-col gap-2 place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#577ce2] after:dark:opacity-40 before:lg:h-[360px] z-[-1]">
          <header>
            <h1 class="text-2xl">Vectris</h1>
          </header>
          <Show
            when= move || matches!(status.get(), Status::InMenus)
            fallback= move || view! { <Game /> }
          >
            <Menu />
          </Show>
          <p>Status: {move || status.get().to_string()}</p>
          <p>Pressed key: <span class="text-sky-400">{ move || key_pressed.get() }</span></p>
        </div>
      </main>
    }
}
