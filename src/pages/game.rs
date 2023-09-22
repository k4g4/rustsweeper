use leptos::*;
use rand::Rng;

const NUM_SVGS: [&str; 9] = [
    "",
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

const ROWS: isize = 8;
const COLUMNS: isize = 16;

const ADJACENTS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const MINE_PROB: f64 = 0.4;

#[derive(Copy, Clone)]
enum CellInteraction {
    Untouched,
    Dug,
    Flagged,
}

#[derive(Copy, Clone)]
enum CellKind {
    Mine,
    Clear(u32),
}

struct CellState {
    interaction: CellInteraction,
    kind: CellKind,
    signal: Option<WriteSignal<(CellInteraction, CellKind)>>,
}

impl CellState {
    fn is_mine(&self) -> bool {
        matches!(self.kind, CellKind::Mine)
    }

    fn is_clear(&self) -> bool {
        matches!(self.kind, CellKind::Clear(_))
    }

    fn is_untouched(&self) -> bool {
        matches!(self.interaction, CellInteraction::Untouched)
    }

    fn _is_dug(&self) -> bool {
        matches!(self.interaction, CellInteraction::Dug)
    }

    fn is_flagged(&self) -> bool {
        matches!(self.interaction, CellInteraction::Flagged)
    }
}

struct GameState {
    dug_count: u32,
    game_over: bool,
    cell_states: Vec<CellState>,
}

impl GameState {
    fn new() -> Self {
        let mut this = Self {
            dug_count: 0,
            game_over: false,
            cell_states: Vec::with_capacity((ROWS * COLUMNS) as usize),
        };

        let mut rng = rand::thread_rng();

        for _ in 0..ROWS * COLUMNS {
            this.cell_states.push(CellState {
                interaction: CellInteraction::Untouched,
                kind: if rng.gen_bool(MINE_PROB) {
                    CellKind::Mine
                } else {
                    CellKind::Clear(0)
                },
                signal: None,
            });
        }

        this.assign_mine_counts();

        this
    }

    fn assign_mine_counts(&mut self) {
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                if self.get(row, column).unwrap().is_clear() {
                    let mines = ADJACENTS
                        .iter()
                        .filter(|(row_offset, column_offset)| {
                            self.get(row + row_offset, column + column_offset)
                                .map_or(false, |cell_state| cell_state.is_mine())
                        })
                        .count();

                    self.get_mut(row, column).unwrap().kind = CellKind::Clear(mines as u32);
                }
            }
        }
    }

    fn get(&self, row: isize, column: isize) -> Option<&CellState> {
        if row < 0 || column < 0 || row >= ROWS || column >= COLUMNS {
            None
        } else {
            Some(&self.cell_states[(row * COLUMNS + column) as usize])
        }
    }

    fn get_mut(&mut self, row: isize, column: isize) -> Option<&mut CellState> {
        if row < 0 || column < 0 || row >= ROWS || column >= COLUMNS {
            None
        } else {
            Some(&mut self.cell_states[(row * COLUMNS + column) as usize])
        }
    }

    fn register_cell(
        &mut self,
        row: isize,
        column: isize,
        set_cell_state: WriteSignal<(CellInteraction, CellKind)>,
    ) {
        self.get_mut(row, column).unwrap().signal = Some(set_cell_state);
    }

    fn dig(&mut self, row: isize, column: isize) {
        // first click is free, wipe out any nearby mines
        if self.dug_count == 0 {
            for (row_offset, column_offset) in std::iter::once((0, 0)).chain(ADJACENTS) {
                if let Some(cell_state) = self.get_mut(row + row_offset, column + column_offset) {
                    cell_state.kind = CellKind::Clear(0);
                }
            }

            self.assign_mine_counts();
        }

        if self.game_over {
            return;
        }

        self.dug_count += 1;

        let Some(cell_state) = self.get_mut(row, column) else {
            self.dug_count -= 1;
            return;
        };

        match cell_state.interaction {
            CellInteraction::Untouched => {
                cell_state.interaction = CellInteraction::Dug;

                cell_state.signal.unwrap()((cell_state.interaction, cell_state.kind));

                if cell_state.is_mine() {
                    self.game_over = true;
                    return;
                }

                // after updating this cell, chain update any adjacent cells if this cell was 0
                if matches!(cell_state.kind, CellKind::Clear(0)) {
                    for (row_offset, column_offset) in ADJACENTS {
                        self.dig(row + row_offset, column + column_offset);
                    }
                }
            }
            CellInteraction::Dug => {
                // when digging on a numbered space, check if enough flags adjacent and dig non-flags
                if let CellKind::Clear(mines) = cell_state.kind {
                    let flags = ADJACENTS
                        .iter()
                        .filter(|(row_offset, column_offset)| {
                            self.get(row + row_offset, column + column_offset)
                                .map_or(false, |cell_state| cell_state.is_flagged())
                        })
                        .count();

                    if mines == flags as u32 {
                        for (row_offset, column_offset) in ADJACENTS {
                            if let Some(cell_state) =
                                self.get(row + row_offset, column + column_offset)
                            {
                                if cell_state.is_untouched() {
                                    self.dig(row + row_offset, column + column_offset);
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                self.dug_count -= 1;
            }
        }
    }

    fn flag(&mut self, row: isize, column: isize) {
        if self.game_over {
            return;
        }

        let Some(cell_state) = self.get_mut(row, column) else {
            return;
        };

        match cell_state.interaction {
            CellInteraction::Untouched => {
                cell_state.interaction = CellInteraction::Flagged;
            }
            CellInteraction::Dug => {
                return;
            }
            CellInteraction::Flagged => {
                cell_state.interaction = CellInteraction::Untouched;
            }
        }

        cell_state.signal.unwrap()((cell_state.interaction, cell_state.kind));
    }
}

/// Renders the game.
#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (_, game_state) = create_signal(cx, GameState::new());

    window_event_listener(ev::contextmenu, |event| event.prevent_default());

    view! { cx,
        <h1>Rustsweeper</h1>
        <div class="buttons">
            <div class="button-item">
                <a href="/">Return</a>
            </div>
        </div>

        <Board game_state />
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
                    0 => {
                        game_state.update(|game_state| game_state.dig(row, column));
                    }
                    2 => {
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
