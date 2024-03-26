use leptos::*;
use super::super::{GameState, Status};

#[component]
pub fn GameMenu() -> impl IntoView {
  let state = expect_context::<GameState>();
  let status = state.status;
  let flag_neoize = state.flag_neoize;
  let (toggle_label, set_toggle_label) = create_signal("Pause");
  let label_style = move || {
    match toggle_label.get() {
      "Resume" => "text-teal-300",
      _ => "text-inherit"
    }
  };
  let toggle_pause = move |_| {
    if status.get() != Status::GameOver {
      if status.get() == Status::Playing {
        status.set(Status::Paused);
        set_toggle_label("Resume");
      } else {
        status.set(Status::Playing);
        set_toggle_label("Pause");
      }
    }
  };
  let quit_game = move |_| {
    // reset state?
    status.set(Status::InMenus);
  };
  let see_the_matrix = move |_| {
    flag_neoize.set(!flag_neoize.get());
  };
  view! {
    <nav>
      <ul class="flex flex-row gap-3 mb-2">
      <li><a href="" on:click=quit_game>Quit</a></li>
      <li><a class={label_style} href="" on:click=toggle_pause>{toggle_label}</a></li>
        <li class="flex grow justify-end pr-3"><a href="" on:click=see_the_matrix>π</a></li>
      </ul>
    </nav>
  }
}
