use leptos::*;

/// 404 - Not Found
#[component]
pub fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <main class="flex flex-col gap-4 items-center p-24">
            <h1 class="text-3xl">"Oops! That's a 404."</h1>
            <h2 class="text-xl">"The resource that was requested was not found."</h2>
            <p class="text-base">
                "Check your spelling, or "
                <a href="/" class="underline underline-offset-3">"go back"</a>
                " to square one."
            </p>
        </main>
    }
}
