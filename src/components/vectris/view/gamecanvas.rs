use leptos::{logging::log, *};

use super::super::{GameState, Cell, Color};

#[component]
pub fn GameCanvas() -> impl IntoView {
  let matrix = expect_context::<GameState>().matrix;

  view! {
    <section id="game-canvas" class="border-solid border-2 border-white p-1 bg-slate-100">
      <For
        each= move || matrix.into_iter().enumerate()
        key=|(index, _row)| *index
        children= move |(_index, row)| {
            view! {
              <div class="flex flex-row gap-px pb-px">
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
  let shade = create_memo(move |_| {
    let base_style = "w-7 h-7";
    log!(" > derived signal logs color: {:?}", cell.get().color);
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

  view! {
    <div class={shade}>
    </div>
  }
}

