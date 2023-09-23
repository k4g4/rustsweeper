use leptos::WriteSignal;
use rand::Rng;

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

#[derive(Copy, Clone)]
pub enum CellInteraction {
    Untouched,
    Dug,
    Flagged,
}

#[derive(Copy, Clone)]
pub enum CellKind {
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

    fn is_dug(&self) -> bool {
        matches!(self.interaction, CellInteraction::Dug)
    }

    fn is_flagged(&self) -> bool {
        matches!(self.interaction, CellInteraction::Flagged)
    }
}

pub struct GameState {
    rows: isize,
    columns: isize,
    mines: isize,
    started: bool,
    game_over: bool,
    cell_states: Vec<CellState>,
    set_score: Option<WriteSignal<String>>,
}

impl GameState {
    pub fn new(rows: isize, columns: isize, mines: isize) -> Self {
        Self {
            rows,
            columns,
            mines,
            started: false,
            game_over: false,
            cell_states: (0..rows * columns)
                .map(|_| CellState {
                    interaction: CellInteraction::Untouched,
                    kind: CellKind::Clear(0),
                    signal: None,
                })
                .collect(),
            set_score: None,
        }
    }

    fn start(&mut self, row: isize, column: isize) {
        let mut rng = rand::thread_rng();

        let exclude = Vec::from_iter(std::iter::once((0, 0)).chain(ADJACENTS).filter_map(
            |(row_offset, column_offset)| self.index(row + row_offset, column + column_offset),
        ));

        for _ in 0..self.mines {
            let cell_state = loop {
                let index = rng.gen_range(0..self.rows * self.columns) as usize;

                if exclude.contains(&index) {
                    continue;
                }

                let cell_state = self.cell_states.get_mut(index).unwrap();

                if !cell_state.is_mine() {
                    break cell_state;
                }
            };

            cell_state.kind = CellKind::Mine;
        }

        for row in 0..self.rows {
            for column in 0..self.columns {
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

        self.started = true;
    }

    fn index(&self, row: isize, column: isize) -> Option<usize> {
        if row < 0 || column < 0 || row >= self.rows || column >= self.columns {
            None
        } else {
            Some((row * self.columns + column) as usize)
        }
    }

    fn get(&self, row: isize, column: isize) -> Option<&CellState> {
        self.index(row, column)
            .map(|index| &self.cell_states[index])
    }

    fn get_mut(&mut self, row: isize, column: isize) -> Option<&mut CellState> {
        self.index(row, column)
            .map(|index| &mut self.cell_states[index])
    }

    pub fn register_score(&mut self, set_score: WriteSignal<String>) {
        self.set_score = Some(set_score);
    }

    pub fn register_cell(
        &mut self,
        row: isize,
        column: isize,
        set_cell_state: WriteSignal<(CellInteraction, CellKind)>,
    ) {
        self.get_mut(row, column)
            .expect("row and column within bounds")
            .signal = Some(set_cell_state);
    }

    fn update_score(&mut self) {
        let mut dug_count = 0;
        let mut failed = false;

        for cell_state in &self.cell_states {
            if cell_state.is_dug() {
                if cell_state.is_mine() {
                    failed = true;
                } else {
                    dug_count += 1;
                }
            }
        }

        (self.set_score.unwrap())(if failed {
            self.game_over = true;
            "Game over!".into()
        } else if dug_count == self.rows * self.columns - self.mines {
            self.game_over = true;
            "You won!".into()
        } else {
            format!(
                "{} point{}",
                dug_count,
                if dug_count == 1 { "" } else { "s" }
            )
        });
    }

    pub fn dig(&mut self, row: isize, column: isize) {
        // first click is free, wipe out any nearby mines
        if !self.started {
            self.start(row, column);
        }

        if self.game_over {
            return;
        }

        let Some(cell_state) = self.get_mut(row, column) else {
            return;
        };

        match cell_state.interaction {
            CellInteraction::Untouched => {
                cell_state.interaction = CellInteraction::Dug;

                cell_state.signal.unwrap()((cell_state.interaction, cell_state.kind));

                // after updating this cell, chain update any adjacent cells if this cell was 0
                if matches!(cell_state.kind, CellKind::Clear(0)) {
                    for (row_offset, column_offset) in ADJACENTS {
                        self.dig(row + row_offset, column + column_offset);
                    }
                }
            }
            CellInteraction::Dug => {
                // when digging on a numbered space, check if enough flags adjacent and dig non-flags
                if let CellKind::Clear(mines) = self.get(row, column).unwrap().kind {
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
            _ => {}
        }

        self.update_score();
    }

    pub fn flag(&mut self, row: isize, column: isize) {
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
