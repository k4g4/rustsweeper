use leptos::*;
use leptos_router::*;

use crate::app_error::AppError;
use crate::game_logic::{CellInteraction, CellKind, GameParams, GameState};
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

const ROWS: isize = 12;
const COLUMNS: isize = 18;
const MINES: isize = ((ROWS * COLUMNS) as f64 * 0.25) as isize;

/// Renders the game.
#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (_, game_state) = create_signal(cx, GameState::new(ROWS, COLUMNS, MINES));

    window_event_listener(ev::contextmenu, |event| event.prevent_default());

    let params = use_query::<GameParams>(cx);

    params.with(|result| match result {
        Ok(GameParams { difficulty }) => {
            game_state.update(|game_state| game_state.set_difficulty(*difficulty));

            view! { cx,
                <h1>Rustsweeper</h1>
                <div class="buttons">
                    <div class="button-item">
                        <a href="/">Return</a>
                    </div>
                </div>

                <Score game_state />

                <Board game_state />
            }
            .into_view(cx)
        }
        Err(_) => {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::InvalidDifficulty);

            view! { cx,
                <Error outside_errors />
            }
            .into_view(cx)
        }
    })
}

/// Displays the current score, or Game Over if you lost.
#[component]
fn Score(cx: Scope, game_state: WriteSignal<GameState>) -> impl IntoView {
    let (score, set_score) = create_signal(cx, String::new());
    game_state.update(|game_state| game_state.register_score(set_score));

    view! { cx,
        <h2 class="score">{score}</h2>
    }
}

/// The game board.
#[component]
fn Board(cx: Scope, game_state: WriteSignal<GameState>) -> impl IntoView {
    view! { cx,
        <div class="game-board">
            { (0..ROWS).map(|row| view!{ cx, <Row row game_state /> }).collect_view(cx) }
        </div>
    }
}

/// A game board row.
#[component]
fn Row(cx: Scope, row: isize, game_state: WriteSignal<GameState>) -> impl IntoView {
    (0..COLUMNS)
        .map(|column| view! { cx, <Cell row column game_state /> })
        .collect_view(cx)
}

/// A cell on the board.
#[component]
fn Cell(cx: Scope, row: isize, column: isize, game_state: WriteSignal<GameState>) -> impl IntoView {
    let (cell_state, set_cell_state) =
        create_signal(cx, (CellInteraction::Untouched, CellKind::Clear(0)));

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
