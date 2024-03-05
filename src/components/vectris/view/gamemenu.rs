use leptos::*;
use super::super::{GameState, Status, Color};

#[component]
pub fn GameMenu() -> impl IntoView {
  let state = expect_context::<GameState>();
  let status = state.status;
  let matrix = state.matrix;
  let quit_game = move |_| {
    // reset state?
    status.set(Status::InMenus);
  };

  let manual_reactivity_test = move |_| {
    matrix[0][4].update(|cell| {
      cell.color = Some(Color::Green);
    });
    matrix[1][4].update(|cell| {
      cell.color = Some(Color::Green);
    });
    matrix[0][5].update(|cell| {
      cell.color = Some(Color::Green);
    });
    matrix[1][5].update(|cell| {
      cell.color = Some(Color::Green);
    });
  };

  view! {
    <nav>
      <ul class="flex flex-row gap-3 mb-2">
        <li>Pause</li>
        <li><a href="" on:click=quit_game>Quit</a></li>
        <li><a href="" on:click=manual_reactivity_test>Matrix reactivity test!</a></li>
      </ul>
    </nav>
  }
}