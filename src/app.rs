use leptos::logging::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app_error::AppError;
use crate::pages::{Error, Game, HomePage};

const LIGHTBULB_SVG: &str = include_str!("../svgs/lightbulb.svg");
const MOON_SVG: &str = include_str!("../svgs/moon.svg");

#[cfg(feature = "ssr")]
async fn get_theme() {
    use axum_extra::extract::cookie::{CookieJar, Cookie};
    use leptos_axum::extract;

    extract(|jar: CookieJar| async move {
        jar
    })
    .await;
}

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
                <button
                    class="theme-toggle"

                    on:click=move |_| {
                        set_theme((if theme() == "light" { "dark" } else { "light" }).to_string())
                    }

                    inner_html=move || {
                        match theme().as_str() {
                            "light" => {
                                MOON_SVG
                            }
                            "dark" => {
                                LIGHTBULB_SVG
                            }
                            _ => { "" }
                        }
                    }
                />
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="game" view=Game />
                </Routes>
            </main>
        </Router>
    }
}
