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

const ROWS: isize = 8;
const COLUMNS: isize = 16;
const MINE_PROB: f64 = 0.4;

#[derive(Copy, Clone)]
enum CellKind {
    Mine,
    Clear(u32),
}

struct CellState {
    dug: bool,
    kind: CellKind,
    signal: Option<WriteSignal<(bool, CellKind)>>,
}

impl CellState {
    fn is_mine(&self) -> bool {
        matches!(self.kind, CellKind::Mine)
    }
    
    fn is_clear(&self) -> bool {
        matches!(self.kind, CellKind::Clear(_))
    }
}

struct GameState(Vec<CellState>);

impl GameState {
    fn new() -> Self {
        let mut this = Self(Vec::with_capacity((ROWS * COLUMNS) as usize));
        let mut rng = rand::thread_rng();

        for _ in 0..ROWS * COLUMNS {
            this.0.push(CellState {
                dug: false,
                kind: if rng.gen_bool(MINE_PROB) {
                    CellKind::Mine
                } else {
                    CellKind::Clear(0)
                },
                signal: None,
            });
        }

        for row in 0..ROWS {
            for column in 0..COLUMNS {
                if this.get(row, column).unwrap().is_clear() {
                    let mines = [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ]
                    .iter()
                    .filter(|(row_offset, column_offset)| {
                        this.get(row + row_offset, column + column_offset)
                            .map_or(false, |cell_state| cell_state.is_mine())
                    })
                    .count();

                    this.get_mut(row, column).unwrap().kind = CellKind::Clear(mines as u32);
                }
            }
        }

        this
    }

    fn get(&self, row: isize, column: isize) -> Option<&CellState> {
        if row < 0 || column < 0 || row >= ROWS || column >= COLUMNS {
            None
        } else {
            Some(&self.0[(row * COLUMNS + column) as usize])
        }
    }

    fn get_mut(&mut self, row: isize, column: isize) -> Option<&mut CellState> {
        if row < 0 || column < 0 || row >= ROWS || column >= COLUMNS {
            None
        } else {
            Some(&mut self.0[(row * COLUMNS + column) as usize])
        }
    }

    fn register_cell(
        &mut self,
        row: isize,
        column: isize,
        set_cell_state: WriteSignal<(bool, CellKind)>,
    ) {
        self.get_mut(row, column).unwrap().signal = Some(set_cell_state);
    }

    fn dig(&mut self, row: isize, column: isize) {
        let cell_state = self.get_mut(row, column).unwrap();
        cell_state.dug = true;
        cell_state.signal.unwrap()((true, cell_state.kind));
    }
}

/// Renders the game.
#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (_, game_state) = create_signal(cx, GameState::new());

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
    let (cell_state, set_cell_state) = create_signal(cx, (false, CellKind::Clear(0)));
    game_state.update(|game_state| game_state.register_cell(row, column, set_cell_state));

    view! { cx,
        <div
            on:click=move |_| {
                game_state.update(|game_state| game_state.dig(row, column));
            }

            class="cell"

            class:dug=move || {
                cell_state().0
            }

            style:grid-row-start={row+1}
            style:grid-column-start={column+1}

            inner_html=move || {
                let (dug, cell_kind) = cell_state();

                if !dug {
                    ""
                } else {
                    match cell_kind {
                        CellKind::Mine => {
                            BOMB_SVG
                        }
                        CellKind::Clear(mines) => {
                            NUM_SVGS[mines as usize]
                        },
                    }
                }
            }
        />
    }
}
