use leptos::*;
use super::super::{GameState, Status};

#[component]
pub fn GameNav() -> impl IntoView {
  let state = expect_context::<RwSignal<GameState>>();
  let set_status = create_write_slice(
      state,
      |state, status| {
          state.status = status
      },
  );
  
  let quit_game = move |_| {
    // reset state?
    set_status.set(Status::InMenus)
  };
  view! {
    <nav>
      <ul class="flex flex-row gap-3">
        <li>Pause</li>
        <li><a href="#menus" on:click=quit_game>Quit</a></li>
      </ul>
    </nav>
  }
}