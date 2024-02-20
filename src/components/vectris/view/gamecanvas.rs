use leptos::*;

use super::super::{GameState, Cell, Color};

#[component]
pub fn GameCanvas() -> impl IntoView {
  let state = expect_context::<RwSignal<GameState>>();
  let matrix = create_read_slice(state, |state| state.matrix);

  view! {
    <section id="game-canvas" class="border-solid border-2 border-white p-1 bg-slate-100">
      <For
        each= move || matrix.get().into_iter().enumerate()
        key=|(index, row)| *index
        children= move |(index, row)| {
            // let row = create_memo(move |_| {
            //   matrix.with(|rows| rows.get(index).map(|r| r).expect("rows to be well defined"))
            // });
            view! {
              <div class="flex flex-row gap-px pb-px">
                <For
                  each= move || row.into_iter().enumerate()
                  key=|(index, cell)| cell.coordinates
                  children= move |(index, cell)| {
                    // let memoized_cell = create_memo(move |_| {
                    //   row[index]
                    // });
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
pub fn CellView(cell: Cell) -> impl IntoView {
  let Cell { coordinates, filled, color} = cell;
  let shared = "w-7 h-7";
  let computed = match color {
    Some(Color::Violet) => format!("{} bg-[rgb(150,0,160)]", shared),
    Some(Color::Green) => format!("{} bg-[rgb(0,150,0)]", shared),
    Some(Color::Blue) => format!("{} bg-[rgb(0,0,180)]", shared),
    Some(Color::Yellow) => format!("{} bg-[rgb(210,190,0)]", shared),
    Some(Color::Red) => format!("{} bg-[rgb(180,0,0)]", shared),
    Some(Color::LightBlue) => format!("{} bg-[rgb(140,180,210)]", shared),
    Some(Color::Pink) => format!("{} bg-[rgb(230,0,200)]", shared),
    None => shared.to_string(),
  };
  let (x, y) = coordinates;
  view! {
    <Show
      when= move || filled
      fallback= move || view! { <EmptyCell /> }
    >
      <ShadedCell classes=computed.clone() />
    </Show>
  }
}


#[component]
pub fn EmptyCell() -> impl IntoView {
  view! {
    <div class="w-7 h-7">
    </div>
  }
}

#[component]
pub fn ShadedCell(classes: String) -> impl IntoView {
  view! {
    <div class=classes>
    </div>
  }
}
