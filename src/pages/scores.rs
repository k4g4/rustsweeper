use leptos::*;
use leptos_router::*;

use crate::{app_error::AppError, game_logic::GameParams, pages::Error, utils::Title};

const MAX_SCORES: usize = 10;

/// Displays the scoreboard.
#[component]
pub fn Scores() -> impl IntoView {
    use_query::<GameParams>().with_untracked(|params| match params {
        Ok(GameParams { difficulty, size }) => view! {
            <h2 class="info">
                "Difficulty: " {difficulty.title()}
                <br />
                "Size: " {size.title()}
            </h2>

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

        Err(error) => {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::ParamsError(error.clone()));

            view! {
                <Error outside_errors />
            }
            .into_view()
        }
    })
}

#[component]
fn Scoreboard() -> impl IntoView {
    view! {
        <div>
            <table class="scoreboard">
                <tr class="header">
                    <th class="n">
                        "#"
                    </th>
                    <th>
                        "Name"
                    </th>
                    <th>
                        "Time"
                    </th>
                </tr>
                {
                    (1..=MAX_SCORES).map(|n| view! {
                        <tr class={ if n % 2 == 0 { "even" } else { "odd" }}>
                            <td class="n">
                                { n.to_string() }
                            </td>
                            <td>
                                "Andres"
                            </td>
                            <td>
                                "20:32"
                            </td>
                        </tr>
                    }).collect_view()
                }
            </table>
        </div>
    }
}
