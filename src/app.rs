use leptos::logging::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app_error::AppError;
use crate::pages::{Error, Game, HomePage};

// #[cfg(feature = "ssr")]
// async fn get_theme() {
//     use axum_extra::extract::cookie::{CookieJar, Cookie};
//     use leptos_axum::extract;

// }

#[component]
pub fn App() -> impl IntoView {
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
        <Stylesheet id="leptos" href="/pkg/rustsweeper.css"/>

        <Title text="Rustsweeper"/>

        <Body class=theme />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <Error outside_errors />
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="game" view=Game />
                </Routes>
            </main>
        </Router>
    }
}
