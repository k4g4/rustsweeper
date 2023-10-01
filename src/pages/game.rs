use std::iter;

use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;

use crate::app_error::AppError;
use crate::game_logic::{format_seconds, CellInteraction, CellKind, GameParams, GameState, Size};
use crate::pages::Error;

const NUM_SVGS: [&str; 9] = [
    "", //just so index starts at 1
    include_str!("../../svgs/1.svg"),
    include_str!("../../svgs/2.svg"),
    include_str!("../../svgs/3.svg"),
    include_str!("../../svgs/4.svg"),
    include_str!("../../svgs/5.svg"),
    include_str!("../../svgs/6.svg"),
    include_str!("../../svgs/7.svg"),
    include_str!("../../svgs/8.svg"),
];

const BOMB_SVG: &str = include_str!("../../svgs/bomb.svg");
const FLAG_SVG: &str = include_str!("../../svgs/flag.svg");

/// Renders the game.
#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let params = use_query::<GameParams>(cx);

    window_event_listener(ev::contextmenu, |event| event.prevent_default());

    params.with_untracked(|params| match params {
        Ok(params) => {
            let game_state = GameState::new(*params);
            let (rows, columns) = game_state.dimensions();

            let (_, game_state) = create_signal(cx, game_state);
            provide_context(cx, game_state);

            view! { cx,
                <h1>"Rustsweeper"</h1>
                <div class="buttons">
                    <div class="button-item">
                        <A
                            href=""
                            on:click=move |ev| {
                                ev.prevent_default();
                                location().reload().expect("reloaded");
                            }
                        >
                            "New Game"
                        </A>
                    </div>
                    <div class="button-item">
                        <A href="/">
                            "Return"
                        </A>
                    </div>
                </div>

                <Info />

                <Board rows columns size=params.size />
            }
            .into_view(cx)
        }

        Err(error) => {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::ParamsError(error.clone()));

            view! { cx,
                <Error outside_errors />
            }
            .into_view(cx)
        }
    })
}

/// Displays the timer and current score.
#[component]
fn Info(cx: Scope) -> impl IntoView {
    let (score, set_score) = create_signal(cx, String::new());
    let (seconds, set_seconds) = create_signal(cx, None);
    let _timer = create_local_resource(cx, seconds, move |seconds| async move {
        if seconds.is_none() {
            return;
        }
        TimeoutFuture::new(1_000).await;
        if let Some(seconds) = seconds {
            set_seconds(Some(seconds + 1));
        }
    });
    let start_timer = Box::new(move || {
        set_seconds(Some(0));
    });
    let stop_timer = Box::new(move || {
        let seconds = seconds();
        set_seconds(None);
        seconds.expect("timer started")
    });

    use_context::<WriteSignal<GameState>>(cx)
        .expect("game state exists")
        .update(|game_state| {
            game_state.register_score(set_score);
            game_state.register_timer(start_timer, stop_timer);
        });

    view! { cx,
        <h2 class="info">
            {
                move || {
                    seconds().map(|seconds| {
                        view! { cx,
                            {format_seconds(seconds)}
                            <br />
                        }
                    })
                }
            }
            {
                move || {
                    // iter::repeat and take(3) so that this view has static size of 3.
                    // Otherwise <For /> is necessary here.
                    score().lines().chain(iter::repeat("")).map(|line| {
                        view! { cx,
                            {line.to_string()}
                            <br />
                        }
                    }).take(3).collect_view(cx)
                }
            }
        </h2>
    }
}

/// The game board.
#[component]
fn Board(cx: Scope, rows: isize, columns: isize, size: Size) -> impl IntoView {
    view! { cx,
        <div class={ format!("game-board {size}") }>
            { (0..rows).map(|row| view!{ cx, <Row row columns /> }).collect_view(cx) }
        </div>
    }
}

/// A game board row.
#[component]
fn Row(cx: Scope, row: isize, columns: isize) -> impl IntoView {
    (0..columns)
        .map(|column| view! { cx, <Cell row column /> })
        .collect_view(cx)
}

/// A cell on the board.
#[component]
fn Cell(cx: Scope, row: isize, column: isize) -> impl IntoView {
    let (cell_state, set_cell_state) =
        create_signal(cx, (CellInteraction::Untouched, CellKind::Clear(0)));
    let game_state = use_context::<WriteSignal<GameState>>(cx).expect("gamestate exists");

    game_state.update(|game_state| game_state.register_cell(row, column, set_cell_state));

    view! { cx,
        <div
            on:mouseup=move |event| {
                match event.button() {
                    0 => { //left click
                        game_state.update(|game_state| game_state.dig(row, column));
                    }
                    2 => { //right click
                        game_state.update(|game_state| game_state.flag(row, column));
                    }
                    _ => {}
                }
            }

            class="cell"

            class:dug=move || {
                matches!(cell_state().0, CellInteraction::Dug)
            }

            style:grid-row-start={row+1}
            style:grid-column-start={column+1}

            inner_html=move || {
                let (interaction, cell_kind) = cell_state();

                match interaction {
                    CellInteraction::Untouched => {
                        ""
                    }
                    CellInteraction::Dug => {
                        match cell_kind {
                            CellKind::Mine => {
                                BOMB_SVG
                            }
                            CellKind::Clear(mines) => {
                                NUM_SVGS[mines as usize]
                            },
                        }
                    }
                    CellInteraction::Flagged => {
                        FLAG_SVG
                    }
                }
            }
        />
    }
}
