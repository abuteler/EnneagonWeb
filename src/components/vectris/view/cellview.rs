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
