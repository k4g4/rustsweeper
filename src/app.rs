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
    let (settings, set_settings) = create_signal(settings);
    provide_context((settings, set_settings));

    view! {
        <Stylesheet id="leptos" href="/pkg/rustsweeper.css"/>

        <Title text="Rustsweeper"/>

        <Body class=move || settings.with(|settings| settings.theme.to_string()) />

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
                        let new_theme = settings.with(|settings| settings.theme.toggle());

                        set_settings.update(|settings| {
                            settings.theme = new_theme;
                        });

                        Settings::set("theme", &new_theme);
                    }

                    inner_html=move || {
                        settings.with(|settings| match settings.theme {
                            Theme::Light => {
                                MOON_SVG
                            }
                            Theme::Dark => {
                                LIGHTBULB_SVG
                            }
                        })
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
