use leptos::*;

/// Renders the home page.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>Welcome to Rustsweeper!</h1>
        <div class="buttons">
            <div class="button-item">
                <a href="game">New game</a>
            </div>
        </div>
    }
}
