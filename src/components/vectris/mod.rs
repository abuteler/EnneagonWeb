mod model;
mod view;
mod controller;
pub use model::*;
pub use view::*;
// pub use controller::Controls;
use leptos::*;

pub const GRID_COLS: usize = 10;
pub const GRID_ROWS: usize = 16;

#[component]
pub fn Vectris() -> impl IntoView {
  // the following line creates a default instance of GameState and provides it as context
  // to the component tree, so that each sub component can subscribe to the reactive signals
  // that are defined on a per property basis, depending on specific needs.
  provide_context(GameState::default());

  // i.e. get the status signal from the provided context to switch between Main Menu and Game modes
  let status = expect_context::<GameState>().status;
  
  view! {
    <main class="relative flex min-h-[575px] h-screen flex-col items-center gap-12 p-8 md:p-24">
      <header>
        <h1 class="text-3xl text-orange-400 font-mono">Vectris</h1>
      </header>
      <Show
        when= move || matches!(status.get(), Status::InMenus)
        fallback= move || view! { <GamePlay /> }
      >
        <MainMenu />
      </Show>
    </main>
  }
}
