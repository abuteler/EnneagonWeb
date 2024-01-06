use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::*;
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    // logging::log!("foo bar");
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/enneagon_web.css"/>

        // sets the document title
        <Title text="Welcome to EnneagonStudios"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=Landing/>
                    <Route path="/soon" view=ComingSoon/>
                    <Route path="/vectris" view=Vectris/>
                    // <Route path="/home" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

