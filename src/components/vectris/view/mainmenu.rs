use leptos::*;
use super::super::{GameState, Status};

#[component]
pub fn MainMenu() -> impl IntoView {
    let state = expect_context::<RwSignal<GameState>>();
    let set_status = create_write_slice(
        state,
        |state, status| {
            state.status = status
        },
    );
    
    let start_game = move |_| {
      set_status.set(Status::Playing)
    };

    view! {
      <section id="menu">
        <nav>
          <ul class="flex flex-col gap-3 items-left border-2 rounded p-6 px-12 text-xl font-semibold font-mono bg-gradient-to-r from-sky-500 to-sky-900">
            <li><a href="" on:click=start_game>Play</a></li>
            <li><hr/></li>
            <li>About</li>
            <li><hr/></li>
            <li>Credits</li>
          </ul>
        </nav>
      </section>
    }
}
