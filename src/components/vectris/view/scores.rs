use leptos::*;

#[component]
pub fn Scores() -> impl IntoView {
  // TODO
  view! {
    <section>
      <h2>Your Score:</h2>
      <h3 class="text-2xl font-bold">170</h3>
      <table>
        <thead>
          <th colspan="2"><td><h2 class="mt-6 text-amber-400">High Scores</h2></td></th>
        </thead>
        <tbody>
          <tr><td>AAA</td><td class="text-right">990</td></tr>
          <tr><td>BBB</td><td class="text-right">770</td></tr>
          <tr><td>CCC</td><td class="text-right">240</td></tr>
          <tr><td>DDD</td><td class="text-right">150</td></tr>
          <tr><td>EEE</td><td class="text-right">060</td></tr>
        </tbody>
      </table>
    </section>
  }
}