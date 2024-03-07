use leptos::*;
use super::super::{GameState, Status};

#[component]
pub fn MainMenu() -> impl IntoView {
    let status = expect_context::<GameState>().status;
    
    let start_game = move |_| {
      status.set(Status::Playing)
    };

    view! {
      <section id="menu">
        <nav>
          <ul class="flex flex-col gap-3 items-left border-2 rounded p-6 px-12 text-xl font-semibold font-mono bg-gradient-to-r from-sky-500 to-sky-900">
            <li><a href="" on:click=start_game>Play</a></li>
            <li><hr/></li>
            <li>About</li>
            <li><hr/></li>
            <li>Credits</li>
            <li><hr/></li>
            <li><a href="/">Back</a></li>
          </ul>
        </nav>
      </section>
    }
}
