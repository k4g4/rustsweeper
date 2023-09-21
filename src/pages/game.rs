use leptos::*;
use std::collections::HashMap;

const ROWS: usize = 8;
const COLUMNS: usize = 16;

#[derive(Default)]
struct GameState(HashMap<(usize, usize), WriteSignal<bool>>);

impl GameState {
    fn register_cell(&mut self, row: usize, column: usize, set_cell_state: WriteSignal<bool>) {
        self.0.insert((row, column), set_cell_state);
    }

    fn dig(&mut self, row: usize, column: usize) {
        self.0[&(row, column)](true)
    }
}

/// Renders the game.
#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (_, game_state) = create_signal(cx, GameState::default());

    view! { cx,
        <h1>Rustsweeper</h1>
        <div class="buttons">
            <div class="button-item">
                <a href="/">Return</a>
            </div>
        </div>

        <div class="game-board">
            <Board game_state />
        </div>
    }
}

/// The game board.
#[component]
fn Board(cx: Scope, game_state: WriteSignal<GameState>) -> impl IntoView {
    view! { cx,
        <table>
            { (0..ROWS).map(|row| view!{ cx, <Row row game_state /> }).collect_view(cx) }
        </table>
    }
}

/// A game board row.
#[component]
fn Row(cx: Scope, row: usize, game_state: WriteSignal<GameState>) -> impl IntoView {
    view! { cx,
        <tr>
            { (0..COLUMNS).map(|column| view!{ cx, <Cell row column game_state /> }).collect_view(cx) }
        </tr>
    }
}

/// A cell on the board.
#[component]
fn Cell(cx: Scope, row: usize, column: usize, game_state: WriteSignal<GameState>) -> impl IntoView {
    let (cell_state, set_cell_state) = create_signal(cx, false);
    game_state.update(|game_state| game_state.register_cell(row, column, set_cell_state));

    view! { cx,
        <td on:click=move |_| game_state.update(|game_state| game_state.dig(row, column)) class:dug=cell_state />
    }
}
