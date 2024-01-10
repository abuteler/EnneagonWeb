mod model;
mod view;
mod controller;
pub use model::*;
pub use view::*;
// pub use controller::Controls;

use leptos::*;


pub const GRID_COLS: u8 = 10;
pub const GRID_ROWS: u8 = 16;

#[component]
pub fn Vectris() -> impl IntoView {
  // set context for reactivity in this component tree
  provide_context(create_rw_signal(GameState::default()));

  // use the context locally to switch between Menus and Game modes
  let state = expect_context::<RwSignal<GameState>>();
  let status = create_read_slice(state, |state| state.status);

  
  view! {
    <main class="relative flex min-h-[575px] h-screen flex-col items-center gap-12 p-8 md:p-24">
      <header>
        <h1 class="text-3xl text-orange-300 font-mono">Vectris</h1>
      </header>
      <Show
        when= move || matches!(status.get(), Status::InMenus)
        fallback= move || view! { <Game /> }
      >
        <Menu />
      </Show>
    </main>
  }
}
