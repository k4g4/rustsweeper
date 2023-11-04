use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app_error::AppError;
use crate::game_settings::{apply_setting, fetch_setting, Theme, Username};
use crate::pages::{Error, Game, HomePage, Scores};

const LIGHTBULB_SVG: &str = include_str!("../svgs/lightbulb.svg");
const MOON_SVG: &str = include_str!("../svgs/moon.svg");

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let theme_setting = fetch_setting::<Theme>("theme");
    let (theme, set_theme) = create_signal(theme_setting.unwrap_or_default());
    if theme_setting.is_none() {
        Effect::new(move |_| {
            if let Ok(Some(mql)) = leptos::window().match_media("(prefers-color-scheme: dark)") {
                if mql.matches() {
                    set_theme(Theme::Dark);
                }
            }
        });
    }

    let (username, set_username) = create_signal(Username::from(fetch_setting("username")));
    provide_context(username);
    provide_context(set_username);

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css" />
        <Stylesheet id="leptos" href="/pkg/rustsweeper.css" />

        <Title text="Rustsweeper" />

        <Html class=move || theme().to_string() />

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
                        let new_theme = theme().toggle();
                        set_theme(new_theme);
                        apply_setting("theme", &new_theme);
                    }

                    inner_html=move || {
                        match theme() {
                            Theme::Light => MOON_SVG,
                            Theme::Dark => LIGHTBULB_SVG,
                        }
                    }
                />
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="game" view=Game />
                    <Route path="scores" view=Scores />
                </Routes>
            </main>
        </Router>
    }
}
