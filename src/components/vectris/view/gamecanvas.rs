use leptos::*;

use crate::components::vectris::CellState;

use super::super::{GameState, Cell, Color};

#[component]
pub fn GameCanvas() -> impl IntoView {
  let game_state = expect_context::<GameState>();
  let matrix = game_state.matrix;
  let avatar = game_state.current_shape;

  create_effect(move |_| {
    // subscribed to changes in the current_shape signal
    // logging::log!(" > avatar: {:?}", avatar.get());
    for cell in avatar.get().cells.into_iter() {
      let Cell { coordinates: (col, row), color, state } = cell;
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
                      <CellView cell=cell />
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

#[component]
pub fn CellView(cell: RwSignal<Cell>) -> impl IntoView {
  let game_state = expect_context::<GameState>();
  let neoize = game_state.neoize;
  let shade = create_memo(move |_| {
    let base_style = "w-7 h-7 flex m-[0.05rem] justify-center text-amber-500";
    match cell.get().color {
      Some(Color::Violet) => format!("{} bg-[rgb(150,0,160)]", base_style),
      Some(Color::Green) => format!("{} bg-[rgb(0,150,0)]", base_style),
      Some(Color::Blue) => format!("{} bg-[rgb(0,0,180)]", base_style),
      Some(Color::Yellow) => format!("{} bg-[rgb(210,190,0)]", base_style),
      Some(Color::Red) => format!("{} bg-[rgb(180,0,0)]", base_style),
      Some(Color::LightBlue) => format!("{} bg-[rgb(140,180,210)]", base_style),
      Some(Color::Pink) => format!("{} bg-[rgb(230,0,200)]", base_style),
      None => base_style.to_string(),
    }
  });
  let cell_state = move || {
    if neoize.get() {
      match cell.get().state {
        CellState::Solid => "s",
        CellState::Empty => "e",
        CellState::Fluid => "f",
        CellState::Exploding => "*",
      }
    } else { "" }
  };
  view! {
    <div class={shade}>
      {cell_state}
    </div>
  }
}

