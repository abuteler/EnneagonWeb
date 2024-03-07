use leptos::*;
use super::super::{GameState, Status};

#[component]
pub fn GameMenu() -> impl IntoView {
  let state = expect_context::<GameState>();
  let status = state.status;
  let neoize = state.neoize;
  let quit_game = move |_| {
    // reset state?
    status.set(Status::InMenus);
  };
  let see_the_matrix = move |_| {
    neoize.set(!neoize.get());
  };
  view! {
    <nav>
      <ul class="flex flex-row gap-3 mb-2">
        <li>Pause</li>
        <li><a href="" on:click=quit_game>Quit</a></li>
        <li class="flex grow justify-end pr-3"><a href="" on:click=see_the_matrix>π</a></li>
      </ul>
    </nav>
  }
}