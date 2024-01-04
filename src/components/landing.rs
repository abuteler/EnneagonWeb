use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
      <main class="flex min-h-screen flex-col items-center justify-evenly p-24">
        <a class="group" href="/soon">
          <div class="relative flex flex-col place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#577ce2] after:dark:opacity-40 before:lg:h-[360px] z-[-1]">
            <img
              src="/assets/logo_landing.svg"
              alt="Enneagon Studios Logo"
              width={180}
              height={180}
              priority
            />
            <div class="relative flex gap-8 place-items-end ml-12 pt-8">
              <img src="/assets/mui_laptop.svg" alt="Laptop icon" width={45} height={45} priority />
              <img src="/assets/mui_tablet.svg" alt="Tablet icon" width={34} height={34} priority />
              <img src="/assets/mui_smartphone.svg" alt="Smartphone icon" width={16} height={16} class="pb-1" priority />
              <div class="w-5 h-6 invisible"></div>
              <img
                src="/assets/mui_present_to_all.svg"
                alt="Present to all icon"
                width={24}
                height={24}
                class="absolute right-7 mb-10 transition-[margin-bottom] ease duration-700 group-hover:mb-52"
                priority
              />
              <span class="text-xl animate-blink">_</span>
            </div>
          </div>
        </a>
      </main>
    }
}
