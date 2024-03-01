use leptos::*;
use super::super::{GameState, Status, Cell, Color};

#[component]
pub fn GameMenu() -> impl IntoView {
  let state = expect_context::<RwSignal<GameState>>();
  let set_status = create_write_slice(
      state,
      |state, status| {
          state.status = status
      },
  );
  let (matrix, set_matrix) = create_slice(
      state,
      |state| state.matrix,
      |state, matrix| {
          state.matrix = matrix
      },
  );
  let quit_game = move |_| {
    // reset state?
    set_status.set(Status::InMenus)
  };

  let manual_reactivity_test = move |_| {
    let mut new_matrix = matrix.get();
    new_matrix[0][4] = Cell::new(0,4,Some(Color::Green));
    new_matrix[0][5] = Cell::new(0,5,Some(Color::Green));
    new_matrix[1][4] = Cell::new(1,4,Some(Color::Green));
    new_matrix[1][5] = Cell::new(1,5,Some(Color::Green));

    new_matrix[15][0] = Cell::new(1,5,Some(Color::Violet));
    new_matrix[15][1] = Cell::new(1,5,Some(Color::Violet));
    new_matrix[14][2] = Cell::new(1,5,Some(Color::LightBlue));
    new_matrix[15][2] = Cell::new(1,5,Some(Color::LightBlue));
    new_matrix[15][3] = Cell::new(1,5,Some(Color::LightBlue));
    new_matrix[15][4] = Cell::new(1,5,Some(Color::LightBlue));
    new_matrix[15][5] = Cell::new(1,5,Some(Color::Pink));
    new_matrix[15][6] = Cell::new(1,5,Some(Color::Pink));
    //
    new_matrix[12][9] = Cell::new(1,5,Some(Color::Blue));
    new_matrix[13][9] = Cell::new(1,5,Some(Color::Blue));
    new_matrix[14][9] = Cell::new(1,5,Some(Color::Blue));
    new_matrix[14][8] = Cell::new(1,5,Some(Color::Blue));
    new_matrix[15][8] = Cell::new(1,5,Some(Color::Yellow));
    new_matrix[15][9] = Cell::new(1,5,Some(Color::Yellow));
    set_matrix(new_matrix);
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