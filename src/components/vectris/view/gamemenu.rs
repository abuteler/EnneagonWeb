use leptos::*;
use super::super::{GameState, Status};

#[component]
pub fn GameMenu() -> impl IntoView {
  let state = expect_context::<GameState>();
  let status = state.status;
  let quit_game = move |_| {
    // reset state?
    status.set(Status::InMenus);
  };

  view! {
    <nav>
      <ul class="flex flex-row gap-3 mb-2">
        <li>Pause</li>
        <li><a href="" on:click=quit_game>Quit</a></li>
      </ul>
    </nav>
  }
}