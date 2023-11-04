use leptos::*;
use leptos_router::*;

use crate::{
    app_error::AppError,
    game_settings::{Difficulty, Size},
    pages::Error,
    utils::Title,
};

const MAX_SCORES: usize = 10;

/// Displays the scoreboard.
#[component]
pub fn Scores() -> impl IntoView {
    let (difficulty, set_difficulty) = create_query_signal::<Difficulty>("difficulty");
    let (size, set_size) = create_query_signal::<Size>("size");
    provide_context((difficulty, size));
    provide_context((set_difficulty, set_size));

    match (difficulty(), size()) {
        (Some(difficulty), Some(size)) => view! {
            <ScoreFilters difficulty size />

            <Scoreboard />

            <div class="btns">
                <div class="btn">
                    <A href="/">
                        "Return"
                    </A>
                </div>
            </div>
        }
        .into_view(),

        _ => {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);

            view! {
                <Error outside_errors />
            }
            .into_view()
        }
    }
}

#[component]
fn ScoreFilters(difficulty: Difficulty, size: Size) -> impl IntoView {
    let (set_difficulty, set_size) =
        expect_context::<(SignalSetter<Option<Difficulty>>, SignalSetter<Option<Size>>)>();

    view! {
        <div class="panel">
            <div class="panel-label">
                "Filters"
            </div>
            <table class="panel-table">
                <tr class="panel-row">
                    <td>
                        <select>
                        {
                            [
                                Difficulty::Easy,
                                Difficulty::Normal,
                                Difficulty::Hard,
                            ].iter().map(|curr_difficulty| {
                                view! {
                                    <option
                                        value=curr_difficulty.to_string()
                                        selected=move || difficulty == *curr_difficulty
                                        on:click=move |_| set_difficulty(Some(*curr_difficulty))
                                    >
                                    {curr_difficulty.title()}
                                    </option>
                                }
                            }).collect_view()
                        }
                        </select>
                    </td>
                    <td>
                        <select>
                        {
                            [
                                Size::Small,
                                Size::Medium,
                                Size::Large,
                            ].iter().map(|curr_size| {
                                view! {
                                    <option
                                        value=curr_size.to_string()
                                        selected=move || size == *curr_size
                                        on:click=move |_| set_size(Some(*curr_size))
                                    >
                                    {curr_size.title()}
                                    </option>
                                }
                            }).collect_view()
                        }
                        </select>
                    </td>
                </tr>
            </table>
        </div>
    }
}

#[component]
fn Scoreboard() -> impl IntoView {
    let (difficulty, size) = expect_context::<(Memo<Option<Difficulty>>, Memo<Option<Size>>)>();

    view! {
        <div>
            <table class="scoreboard">
                <tr class="header">
                    <th class="n">
                        "#"
                    </th>
                    <th class="name">
                        "Name"
                    </th>
                    <th class="time">
                        "Time"
                    </th>
                </tr>
                {
                    (1..=MAX_SCORES).map(|n| view! {
                        <tr class={ if n % 2 == 0 { "even" } else { "odd" }}>
                            <td class="n">
                                { n.to_string() }
                            </td>
                            <td class="name">
                                {move || difficulty().map(|difficulty| difficulty.to_string())}
                            </td>
                            <td class="time">
                                {move || size().map(|size| size.to_string())}
                            </td>
                        </tr>
                    }).collect_view()
                }
            </table>
        </div>
    }
}
