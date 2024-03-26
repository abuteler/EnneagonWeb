use leptos::*;

use crate::components::vectris::{GameState, Cell, CellView};

#[component]
pub fn GameCanvas() -> impl IntoView {
  let game_state = expect_context::<GameState>();
  let matrix = game_state.matrix;
  let avatar = game_state.current_shape;

  create_effect(move |_| {
    // subscribed to changes in the current_shape signal
    for cell in avatar.get().cells.into_iter() {
      let Cell { coordinates: (row, col), color, state } = cell;
      matrix[row][col].update(|m_cell| {
        m_cell.color = color;
        m_cell.state = state;
      });
    };
  });

  view! {
    <section id="game-canvas" class="border-solid border-8 border-white p-[2px] bg-slate-100 border-l-gray-300 border-r-gray-400 border-b-vectris-floor">
      <For
        each= move || matrix.into_iter().enumerate()
        key=|(index, _row)| *index
        children= move |(_index, row)| {
            view! {
              <div class="flex flex-row">
                <For
                  each= move || row.into_iter().enumerate()
                  key=|(index, _cell)| *index
                  children= move |(_index, cell)| {
                    view! {
                      <CellView cell=cell compact=false />
                    }
                  }
                />
              </div>
            }
        }
      />
      </section>
  }
}
