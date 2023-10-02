use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app_error::AppError;
use crate::pages::{Error, Game, HomePage};

async fn get_theme() {
    
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (theme, set_theme) = create_signal("light".to_string());

    create_effect(move |_| {
        if let Ok(Some(mql)) = window().match_media("(prefers-color-scheme: dark)") {
            if mql.matches() {
                set_theme("dark".to_string());
            }
        }
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rustsweeper.css"/>

        // sets the document title
        <Title text="Rustsweeper"/>

        <Body class=theme />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <Error outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=|| view! { <HomePage /> }/>
                    <Route path="game" view=|| view! { <Game /> }/>
                </Routes>
            </main>
        </Router>
    }
}
