use leptos::*;

use crate::components::vectris::{Cell, CellState, Color, GameState};

#[component]
pub fn CellView(cell: RwSignal<Cell>, compact: bool) -> impl IntoView {
  let game_state = expect_context::<GameState>();
  let flag_neoize = game_state.flag_neoize;
  let shade = create_memo(move |_| {
    let cell_size = match compact {
      false => "w-7 h-7",
      true => "w-5 h-5"
    };
    let cell_border = "border-[2px] border-t-slate-100 border-l-slate-200 border-r-slate-400 border-b-slate-500";
    let base_style = format!("{} flex m-[0.05rem] justify-center text-amber-500", cell_size);
    match cell.get().state {
      CellState::Exploding => format!("{} {} animate-xplode", base_style, Color::Gold.to_rgb_str()),
      _ => {
        match cell.get().color {
          Some(val) => format!("{} {} {}", base_style, val.to_rgb_str(), cell_border),
          None => base_style.to_string(),
        }
      }
    }
  });
  let cell_state = move || {
    if flag_neoize.get() && !compact {
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
