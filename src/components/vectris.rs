use leptos::*;

#[component]
pub fn Vectris() -> impl IntoView {
    view! {
      <main class="relative flex min-h-[575px] h-screen flex-col items-center justify-evenly p-8 md:p-24">
        <div class="relative flex flex-col gap-2 place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#577ce2] after:dark:opacity-40 before:lg:h-[360px] z-[-1]">
          "Getting there..."
        </div>
      </main>
    }
}
