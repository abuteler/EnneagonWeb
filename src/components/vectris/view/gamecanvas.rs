use leptos::{logging::log, *};

use super::super::{GameState, Cell, Color};

#[component]
pub fn GameCanvas() -> impl IntoView {
  let state = expect_context::<RwSignal<GameState>>();
  let matrix = create_read_slice(state, |state| state.matrix);

  let iterable_matrix = move || matrix().into_iter().enumerate();

  create_effect(move |_| {
    log!("hey");
    matrix().into_iter().enumerate().for_each(|el| {
      log!("row {:?} in create_effect: {:?}", el.0, el.1);
    });
    matrix()
  });

  view! {
    <section id="game-canvas" class="border-solid border-2 border-white p-1 bg-slate-100">
      <For
        each= move || matrix().into_iter().enumerate()
        key=|(index, _row)| *index
        children= move |(r_index, _)| {
            let row = create_memo(move |_| {
              matrix.with(|matrix| matrix.get(r_index).map(|r| r))
            });
            // create_effect(move |_| {
            //   log!("hey row {:?} {:?}", r_index, row);
            //   matrix() // RECIBE el update, pero no actualiza el contenido de row !!
            //   // iterable_matrix();
            //   // row
            // });
            view! {
              <div class="flex flex-row gap-px pb-px">
                <For
                  each= move || row.get().unwrap().into_iter().enumerate()
                  key=|(index, cell)| index
                  children= move |(c_index, cell)| {
                    let memoized_cell = create_memo(move |_| {
                      matrix.with(|matrix| matrix.get(r_index).map(|r| r[c_index]).expect("errored because cell"))
                    });
                    create_effect(move |_| {
                      log!(" > hey cell {:?} {:?}", c_index, cell);
                      iterable_matrix();
                      cell
                    });
                    view! {
                      <CellView cell=memoized_cell />
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
pub fn CellView(cell: Memo<Cell>) -> impl IntoView {
  let Cell { coordinates, color} = cell.get();
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
      when= move || match color {
        Some(_) => true,
        None => false
      }
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
