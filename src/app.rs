use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app_error::AppError;
use crate::app_settings::{Settings, Theme};
use crate::pages::{Error, Game, HomePage};

pub const TAILWIND_BUTTON: &str = concat!(
    "inline-block font-semibold mx-3 ",
    "text-white cursor-pointer [&>*]:cursor-pointer ",
    "rounded-lg p-2 bg-sky-600 ring-1 ring-sky-600/5 ",
    "shadow-xl shadow-gray-400 hover:bg-sky-700 hover:ring-sky-700");

const LIGHTBULB_SVG: &str = include_str!("../svgs/lightbulb.svg");
const MOON_SVG: &str = include_str!("../svgs/moon.svg");

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let settings = Settings::fetch();
    let (settings, set_settings) = create_signal(settings);
    provide_context((settings, set_settings));

    view! {
        <Stylesheet id="leptos" href="/pkg/rustsweeper.css" />

        <Title text="Rustsweeper" />

        <Body class=move || {
            settings.with(|Settings { theme, ..}|
                format!("text-center bg-slate-100 text-black {theme}"))
        }/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <Error outside_errors />
            }
            .into_view()
        }>
            <main>
                <div class="text-4xl my-5 mx-auto font-bold">"Rustsweeper"</div>
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
