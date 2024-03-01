use leptos::*;
use super::{GameMenu, GameCanvas};
use leptos::leptos_dom::ev::{keydown, KeyboardEvent};

#[component]
pub fn GamePlay() -> impl IntoView {
  // set listeners for game controls
  let (key_pressed, set_key_pressed) = create_signal(String::new());
  let handler = window_event_listener(keydown, move |ev: KeyboardEvent| {
      let key = &ev.key_code();
      set_key_pressed.set(key.to_string());
      // match key {
      //     37 => vectris.on_key_left(),
      //     38 => vectris.on_key_up(),
      //     39 => vectris.on_key_right(),
      //     40 => vectris.on_key_down(),
      //     _ => {}
      // }
  });
  on_cleanup(move || handler.remove());

  // let initialize_game = move |_| {
  //   let mut draw_shape = matrix.get();
  //   new_matrix[0][4] = Cell::new(0,4,Some(Color::Green));
  //   new_matrix[0][5] = Cell::new(0,5,Some(Color::Green));
  //   new_matrix[1][4] = Cell::new(1,4,Some(Color::Green));
  //   new_matrix[1][5] = Cell::new(1,5,Some(Color::Green));

  //   new_matrix[15][0] = Cell::new(1,5,Some(Color::Violet));
  //   new_matrix[15][1] = Cell::new(1,5,Some(Color::Violet));
  //   new_matrix[14][2] = Cell::new(1,5,Some(Color::LightBlue));
  //   new_matrix[15][2] = Cell::new(1,5,Some(Color::LightBlue));
  //   new_matrix[15][3] = Cell::new(1,5,Some(Color::LightBlue));
  //   new_matrix[15][4] = Cell::new(1,5,Some(Color::LightBlue));
  //   new_matrix[15][5] = Cell::new(1,5,Some(Color::Pink));
  //   new_matrix[15][6] = Cell::new(1,5,Some(Color::Pink));
  //   //
  //   new_matrix[12][9] = Cell::new(1,5,Some(Color::Blue));
  //   new_matrix[13][9] = Cell::new(1,5,Some(Color::Blue));
  //   new_matrix[14][9] = Cell::new(1,5,Some(Color::Blue));
  //   new_matrix[14][8] = Cell::new(1,5,Some(Color::Blue));
  //   new_matrix[15][8] = Cell::new(1,5,Some(Color::Yellow));
  //   new_matrix[15][9] = Cell::new(1,5,Some(Color::Yellow));
  //   set_matrix(new_matrix);
  // };

  view! {
    <section id="game-canvas">
      <GameMenu />
      <GameCanvas />
      <p>Pressed key code: <span class="text-sky-400">{ move || key_pressed.get() }</span></p>
    </section>
  }
}
