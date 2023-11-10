use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::{
    app_error::AppError,
    game_settings::{Difficulty, Size},
    pages::Error,
    utils::Title,
};

const MAX_SCORES: usize = 10;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Entry {
    name: String,
    time: String,
}

#[server(GetScores)]
async fn get_scores(difficulty: Difficulty, size: Size) -> Result<Vec<Entry>, ServerFnError> {
    let _pool = expect_context::<sqlx::SqlitePool>();

    Ok(vec![Entry {
        name: difficulty.to_string(),
        time: size.to_string(),
    }])
}

/// Displays the scoreboard.
#[component]
pub fn Scores() -> impl IntoView {
    let (difficulty, set_difficulty) = create_query_signal::<Difficulty>("difficulty");
    let (size, set_size) = create_query_signal::<Size>("size");
    provide_context((difficulty, size));
    provide_context((set_difficulty, set_size));

    match (difficulty.get_untracked(), size.get_untracked()) {
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
    let filters = move || (difficulty().unwrap_or_default(), size().unwrap_or_default());
    let score_getter = create_resource(filters, |(difficulty, size)| async move {
        get_scores(difficulty, size).await.unwrap_or_default()
    });

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
                <Transition fallback=move || view! { <ScoreRows scores=vec![] /> }>
                    {move || view! { <ScoreRows scores=score_getter().unwrap_or_default() /> }}
                </Transition>
            </table>
        </div>
    }
}

#[component]
fn ScoreRows(mut scores: Vec<Entry>) -> impl IntoView {
    scores.resize_with(MAX_SCORES, Default::default);

    scores
        .into_iter()
        .zip(1..=MAX_SCORES)
        .map(|(Entry { name, time }, n)| {
            view! {
                <tr class={ if n % 2 == 0 { "even" } else { "odd" }}>
                    <td class="n">
                        { n.to_string() }
                    </td>
                    <td class="name">
                        {name}
                    </td>
                    <td class="time">
                        {time}
                    </td>
                </tr>
            }
        })
        .collect_view()
}
