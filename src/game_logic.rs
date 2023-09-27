use leptos::*;
use leptos_router::*;
use rand::Rng;
use std::{fmt::Display, str::FromStr};

use crate::app_error::AppError;

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

#[derive(PartialEq, Copy, Clone, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Medium,
    Hard,
}

impl FromStr for Difficulty {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "easy" => Self::Easy,
            "medium" => Self::Medium,
            "hard" => Self::Hard,
            _ => return Err(AppError::InvalidDifficulty),
        })
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Easy => "easy",
                Self::Medium => "medium",
                Self::Hard => "hard",
            }
        )
    }
}

#[derive(Params, PartialEq)]
pub struct GameParams {
    pub difficulty: Difficulty,
}

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
    mines: Option<isize>,
    started: bool,
    game_over: bool,
    cell_states: Vec<CellState>,
    set_score: Option<WriteSignal<String>>,
    start_timer: Option<Box<dyn FnOnce()>>,
    stop_timer: Option<Box<dyn FnOnce() -> u32>>,
}

impl GameState {
    const EASY_PROB: f64 = 0.15;
    const MEDIUM_PROB: f64 = 0.25;
    const HARD_PROB: f64 = 0.35;

    pub fn new(rows: isize, columns: isize) -> Self {
        Self {
            rows,
            columns,
            mines: None,
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
            start_timer: None,
            stop_timer: None,
        }
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.mines = Some(
            ((self.rows * self.columns) as f64
                * match difficulty {
                    Difficulty::Easy => Self::EASY_PROB,
                    Difficulty::Medium => Self::MEDIUM_PROB,
                    Difficulty::Hard => Self::HARD_PROB,
                }) as isize,
        );
    }

    fn start(&mut self, row: isize, column: isize) {
        self.start_timer.take().expect("registered start timer")();

        let mut rng = rand::thread_rng();

        let exclude = Vec::from_iter(std::iter::once((0, 0)).chain(ADJACENTS).filter_map(
            |(row_offset, column_offset)| self.index(row + row_offset, column + column_offset),
        ));

        for _ in 0..self.mines.expect("difficulty set") {
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
        (row >= 0 && column >= 0 && row < self.rows && column < self.columns)
            .then_some((row * self.columns + column) as usize)
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

    pub fn register_start_stop_timer(
        &mut self,
        start_timer: Box<dyn FnOnce()>,
        stop_timer: Box<dyn FnOnce() -> u32>,
    ) {
        self.start_timer = Some(start_timer);
        self.stop_timer = Some(stop_timer);
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

        (self.set_score.expect("registered score"))(if failed {
            self.game_over = true;
            let time = self.stop_timer.take().expect("registered stop timer")();
            format!("Game over! {} seconds and {} points", time, dug_count)
        } else if dug_count == self.rows * self.columns - self.mines.expect("difficulty set") {
            self.game_over = true;
            "You won!".into()
        } else {
            format!(
                "{} point{}",
                dug_count,
                if dug_count == 1 { "" } else { "s" },
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

        cell_state.signal.expect("cell signal registered")((
            cell_state.interaction,
            cell_state.kind,
        ));
    }
}
