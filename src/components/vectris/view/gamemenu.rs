use leptos::*;
use super::super::{GameState, Status};

#[component]
pub fn Menu() -> impl IntoView {
    let state = expect_context::<RwSignal<GameState>>();
    let set_status = create_write_slice(
        state,
        |state, status| {
            state.status = status
        },
    );
    logging::log!("this runs fine");
    let start_game = move |_| {
      logging::log!("this doesn't ever, not even bound to an anchor, or a button");
      set_status.set(Status::Playing)
    };

    view! {
      <section id="menu">
        <nav>
          <h2>Menu</h2>
          <ul id="main-menu">
            <li on:click=start_game>Play</li>
            <li>About</li>
            <li>Credits</li>
          </ul>
        </nav>
      </section>
    }
}
