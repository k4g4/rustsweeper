use leptos::logging::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app_error::AppError;
use crate::app_settings::{Settings, Theme};
use crate::pages::{Error, Game, HomePage};

const LIGHTBULB_SVG: &str = include_str!("../svgs/lightbulb.svg");
const MOON_SVG: &str = include_str!("../svgs/moon.svg");

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let settings = Settings::fetch();

    let (theme, set_theme) = create_signal(Theme::default());
    set_theme(settings.theme);

    provide_context(settings);

    view! {
        <Stylesheet id="leptos" href="/pkg/rustsweeper.css"/>

        <Title text="Rustsweeper"/>

        <Body class=move || theme().to_string() />

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
                        let new_theme = theme().toggle();
                        set_theme(new_theme);

                        cfg_if::cfg_if! { if #[cfg(target_arch = "wasm32")] {

                        wasm_cookies::set(
                            "theme",
                            &new_theme.to_string(),
                            &wasm_cookies::CookieOptions::default()
                                .expires_after(chrono::Duration::weeks(999).to_std().expect("converts fine")));

                        }}
                    }

                    inner_html=move || {
                        match theme() {
                            Theme::Light => {
                                MOON_SVG
                            }
                            Theme::Dark => {
                                LIGHTBULB_SVG
                            }
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
