use leptos::*;

use crate::components::vectris::{Cell, CellState, Color, GameState};

#[component]
pub fn CellView(cell: RwSignal<Cell>, compact: bool) -> impl IntoView {
  let game_state = expect_context::<GameState>();
  let neoize = game_state.neoize;
  let shade = create_memo(move |_| {
    let base_style = match compact {
      false => "w-7 h-7 flex m-[0.05rem] justify-center text-amber-500",
      true => "w-5 h-5 flex m-[0.05rem] justify-center text-amber-500"
    };
    match cell.get().state {
      CellState::Exploding => format!("{} {} animate-xplode", base_style, Color::Gold.to_rgb_str()),
      _ => {
        match cell.get().color {
          Some(val) => format!("{} {}", base_style, val.to_rgb_str()),
          None => base_style.to_string(),
        }
      }
    }
  });
  let cell_state = move || {
    if neoize.get() && !compact {
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
