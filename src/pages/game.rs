use leptos::*;
use leptos_router::*;

use crate::app_error::AppError;
use crate::game_logic::{CellInteraction, CellKind, GameParams, GameState};
use crate::game_settings::Size;
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
pub fn Game() -> impl IntoView {
    window_event_listener(ev::contextmenu, |ev| ev.prevent_default());

    use_query::<GameParams>().with_untracked(|params| match params {
        Ok(params) => {
            let game_state = GameState::new(*params);
            let (rows, columns) = game_state.dimensions();
            let new_game_enabled = game_state.new_game_enabled_signal();

            let (game_state_read, game_state_write) = create_signal(game_state);
            provide_context(game_state_read);
            provide_context(game_state_write);

            view! {
                <div class="btns">
                    <div class=move || { format!("btn {}", if new_game_enabled() { "" } else { "disabled" }) }>
                        <A
                            href=""

                            on:click=move |ev| {
                                ev.prevent_default();

                                if new_game_enabled() {
                                    game_state_write.update(|game_state| game_state.reset());
                                }
                            }

                            class=move || { if new_game_enabled() { "" } else { "disabled" } }
                        >
                            "New Game"
                        </A>
                    </div>
                    <div class="btn">
                        <A href="/">
                            "Return"
                        </A>
                    </div>
                </div>

                <Info />

                <Board rows columns size=params.size />
            }
            .into_view()
        }

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

/// Displays the timer and current score.
#[component]
fn Info() -> impl IntoView {
    let info = use_context::<ReadSignal<GameState>>()
        .expect("game state exists")
        .with_untracked(|game_state| game_state.info_signal());

    view! {
        <h2 class="info">
            { move || info.with(|info| info.to_view()) }
        </h2>
    }
}

/// The game board.
#[component]
fn Board(rows: isize, columns: isize, size: Size) -> impl IntoView {
    view! {
        <div class={ format!("game-board {size}") }>
            { (0..rows).map(|row| view!{ <Row row columns /> }).collect_view() }
        </div>
    }
}

/// A game board row.
#[component]
fn Row(row: isize, columns: isize) -> impl IntoView {
    (0..columns)
        .map(|column| view! { <Cell row column /> })
        .collect_view()
}

/// A cell on the board.
#[component]
fn Cell(row: isize, column: isize) -> impl IntoView {
    let (cell_state, set_cell_state) =
        create_signal((CellInteraction::Untouched, CellKind::Clear(0)));
    let game_state_write = use_context::<WriteSignal<GameState>>().expect("game state exists");

    game_state_write.update(|game_state| game_state.register_cell(row, column, set_cell_state));

    view! {
        <div
            on:mouseup=move |event| {
                match event.button() {
                    0 => { //left click
                        game_state_write.update(|game_state| game_state.dig(row, column));
                    }
                    2 => { //right click
                        game_state_write.update(|game_state| game_state.flag(row, column));
                    }
                    _ => {}
                }
            }

            class=move || {
                match cell_state() {
                    (CellInteraction::Flagged, _) => "cell flagged".into(),
                    (_, CellKind::Mine) => "cell mine".into(),
                    (_, CellKind::Clear(num)) => format!("cell num-{num}"),
                }
            }

            class:cleared=move || {
                matches!(cell_state().0, CellInteraction::Cleared)
            }

            style:grid-row-start={row+1}
            style:grid-column-start={column+1}

            inner_html=move || {
                let (interaction, cell_kind) = cell_state();

                match interaction {
                    CellInteraction::Untouched => {
                        ""
                    }
                    CellInteraction::Cleared => {
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
