use leptos::*;

use crate::components::vectris::{Cell, CellState, CellView, GameState, GRID_COLS};

#[component]
pub fn UpNext() -> impl IntoView {
  let next = expect_context::<GameState>().next_shape;
  const CANVAS_HEIGHT: usize = 6;
  const CANVAS_WIDTH: usize = 4;
  let mut canvas: [[RwSignal<Cell>; CANVAS_WIDTH]; CANVAS_HEIGHT] = Default::default();
  for r in 0..CANVAS_HEIGHT-1 {
    for c in 0..CANVAS_WIDTH-1 {
      canvas[r][c] = create_rw_signal(Cell::new(c, r, None, CellState::Empty));
    }
  };
  create_effect(move |_| {
      // subscribe to changes in the next_shape signal
      // clear previous state
      for r in 0..CANVAS_HEIGHT-1 {
        for c in 0..CANVAS_WIDTH-1 {
          canvas[r][c].update(|canvas_cell| {
            canvas_cell.color = None;
            canvas_cell.state = CellState::Empty;
          });
        };
      };
      for (i,cell) in next.get().cells.into_iter().enumerate() {
        let Cell { coordinates: (col, row), color, state } = cell;
        // adjust original cell coordinates to smaller canvas
        let row_ = row + 1; // because y = 0
        // original x calculation = `GRID_COLS / 2 - 1`, so, move to zero, then +1
        let col_ = col - (GRID_COLS / 2 - 1) + 1;
        canvas[row_][col_].update(|canvas_cell| {
          canvas_cell.color = color;
          canvas_cell.state = state;
        });
      };
  });
  view! {
    <section class="ml-4">
      <h2 class="mt-6 mb-3">Up next...</h2>
      <div class="border-solid border-[3px] bg-orange-100 border-vectris-upnext-border p-0 h-fit">
        <For
          each= move || canvas.into_iter().enumerate()
          key=|(index, _row)| *index
          children= move |(_index, row)| {
            view! {
              <div class="flex flex-row">
                <For
                  each= move || row.into_iter().enumerate()
                  key=|(index, _cell)| *index
                  children= move |(_index, cell)| {
                    view! {
                      <CellView cell=cell compact=true />
                    }
                  }
                />
              </div>
            }
          }
        />
      </div>
    </section>
  }
}
