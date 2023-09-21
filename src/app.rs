use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rustsweeper.css"/>

        // sets the document title
        <Title text="Rustsweeper"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <h1>"Hello, world!"</h1>
        <div class="buttons">
            <div class="button-item">
                <button on:click=move |_| set_count.update(|count| *count += 1)>
                    "Clicked " {count} " time" { move || if count()==1 {""} else {"s"} }
                </button>
            </div>
            <div class="button-item">
                <button on:click=move |_| set_count(0)>"Reset"</button>
            </div>
        </div>
    }
}
