use leptos::*;
use leptos_router::*;

use crate::game_logic::Difficulty;

/// Renders the home page.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let (difficulty, set_difficulty) = create_signal(cx, Difficulty::Easy);
    let new_game_href = move || format!("game?difficulty={}", difficulty());

    view! { cx,
        <h1>"Rustsweeper"</h1>
        <div class="settings">
            <label for="difficulty">"Difficulty: "</label>
            <select
                name="difficulty"

                on:input=move |ev| {
                    set_difficulty(event_target_value(&ev).parse().unwrap_or_default());
                }

                prop:value=move || difficulty().to_string()

                value=move || difficulty().to_string()
            >
                <option value=|| Difficulty::Easy.to_string()>"Easy"</option>
                <option value=|| Difficulty::Medium.to_string()>"Medium"</option>
                <option value=|| Difficulty::Hard.to_string()>"Hard"</option>
            </select>
        </div>
        <div class="buttons">
            <div class="button-item">
                <A href=new_game_href>"New Game"</A>
            </div>
        </div>
    }
}
