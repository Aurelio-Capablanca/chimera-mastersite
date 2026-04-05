use crate::frontend::view::login::Login;
use leptos::logging;
use leptos::{prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::hooks::use_navigate;
use leptos_router::path;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/chimera-mastersite.css"/>

        // sets the document title
        <Title text="welcome visitor !"/>
        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!("login") view=Login/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    let navigate = use_navigate();
    let go_to_login = move |_| {
        logging::log!("Admin button clicked! Attempting navigation...");
        navigate("/login", Default::default());
    };

    view! {
        <div class="welcome-container">
            <button on:click=go_to_login>"are you admin?"</button>
            <h1>"Welcome Visitor !"</h1>            
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
