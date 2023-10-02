use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use chrono::Duration;
use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;
use rand::Rng;
use thiserror::Error;

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

const EASY: &str = "easy";
const MEDIUM: &str = "medium";
const HARD: &str = "hard";
const SMALL: &str = "small";
const LARGE: &str = "large";

#[derive(Error, Debug)]
pub enum GameParamsError {
    InvalidSize,
    InvalidDifficulty,
}

impl Display for GameParamsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameParamsError::InvalidSize => "Invalid size",
                GameParamsError::InvalidDifficulty => "Invalid difficulty",
            }
        )
    }
}

#[derive(PartialEq, Copy, Clone, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Medium,
    Hard,
}

impl FromStr for Difficulty {
    type Err = GameParamsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            EASY => Self::Easy,
            MEDIUM => Self::Medium,
            HARD => Self::Hard,
            _ => return Err(GameParamsError::InvalidDifficulty),
        })
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Easy => EASY,
                Self::Medium => MEDIUM,
                Self::Hard => HARD,
            }
        )
    }
}

#[derive(PartialEq, Copy, Clone, Default)]
pub enum Size {
    #[default]
    Small,
    Medium,
    Large,
}

impl FromStr for Size {
    type Err = GameParamsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            SMALL => Self::Small,
            MEDIUM => Self::Medium,
            LARGE => Self::Large,
            _ => return Err(GameParamsError::InvalidSize),
        })
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Small => SMALL,
                Self::Medium => MEDIUM,
                Self::Large => LARGE,
            }
        )
    }
}

#[derive(Copy, Clone, PartialEq, Params)]
pub struct GameParams {
    pub difficulty: Difficulty,
    pub size: Size,
}

#[derive(Default, Copy, Clone)]
pub enum GameStatus {
    #[default]
    Idle,
    Started,
    GameOver,
    Victory,
}

#[derive(Default)]
pub struct GameInfo {
    elapsed_seconds: u32,
    score: u32,
    status: GameStatus,
}

impl GameInfo {
    pub fn to_view(&self, cx: Scope) -> impl IntoView {
        let time = {
            let duration = Duration::seconds(self.elapsed_seconds as i64);
            format!(
                "{:02}:{:02}",
                duration.num_minutes() % 99,
                duration.num_seconds() % 60
            )
        };

        match self.status {
            GameStatus::Started => {
                view! { cx,
                    {format!("{} points", self.score)}
                    <br />
                    {time}
                    <br />
                    ""
                    <br />
                }
            }
            GameStatus::GameOver => {
                view! { cx,
                    "Game over!"
                    <br />
                    "Score - "{format!("{} points", self.score)}
                    <br />
                    "Time - " {time}
                    <br />
                }
            }
            GameStatus::Victory => {
                view! { cx,
                    "You won!"
                    <br />
                    "Score - " {format!("{} points", self.score)}
                    <br />
                    "Time - " {time}
                    <br />
                }
            }
            GameStatus::Idle => {
                view! { cx,
                    ""
                    <br />
                    ""
                    <br />
                    ""
                    <br />
                }
            }
        }
    }
}

#[derive(Copy, Clone, Default)]
pub enum CellInteraction {
    #[default]
    Untouched,
    Dug,
    Flagged,
}

#[derive(Copy, Clone)]
pub enum CellKind {
    Mine,
    Clear(u32),
}

impl Default for CellKind {
    fn default() -> Self {
        Self::Clear(0)
    }
}

#[derive(Default, Clone)]
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
    cell_states: Vec<CellState>,
    status: GameStatus,
    info: ReadSignal<GameInfo>,
    set_info: WriteSignal<GameInfo>,
    timer: Action<(), ()>,
}

impl GameState {
    const EASY_PROB: f64 = 0.15;
    const MEDIUM_PROB: f64 = 0.25;
    const HARD_PROB: f64 = 0.35;

    const SMALL_SIZE: (isize, isize) = (8, 12);
    const MEDIUM_SIZE: (isize, isize) = (10, 15);
    const LARGE_SIZE: (isize, isize) = (12, 18);

    pub fn new(cx: Scope, params: GameParams) -> Self {
        let (rows, columns) = match params.size {
            Size::Small => Self::SMALL_SIZE,
            Size::Medium => Self::MEDIUM_SIZE,
            Size::Large => Self::LARGE_SIZE,
        };
        let total = (rows * columns) as usize;

        let (info, set_info) = create_signal(cx, GameInfo::default());

        let timer = create_action(cx, move |&()| async move {
            for second in 0..u32::MAX {
                let mut stop = false;
                set_info.update(|info| {
                    if matches!(info.status, GameStatus::Started) {
                        info.elapsed_seconds = second;
                    } else {
                        stop = true;
                    }
                });
                if stop {
                    break;
                }
                TimeoutFuture::new(1_000).await;
            }
        });

        Self {
            rows: rows,
            columns: columns,
            cell_states: vec![CellState::default(); total],
            mines: (total as f64
                * match params.difficulty {
                    Difficulty::Easy => Self::EASY_PROB,
                    Difficulty::Medium => Self::MEDIUM_PROB,
                    Difficulty::Hard => Self::HARD_PROB,
                }) as isize,
            status: GameStatus::Idle,
            info,
            set_info,
            timer,
        }
    }

    pub fn dimensions(&self) -> (isize, isize) {
        (self.rows, self.columns)
    }

    pub fn info_signal(&self) -> ReadSignal<GameInfo> {
        self.info
    }

    fn start(&mut self, row: isize, column: isize) {
        self.timer.dispatch(());

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

        self.status = GameStatus::Started;
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

        if failed {
            self.status = GameStatus::GameOver;
        } else if dug_count as isize == self.rows * self.columns - self.mines {
            self.status = GameStatus::Victory;
        }

        self.set_info.update(|info| {
            info.score = dug_count;
            info.status = self.status;
        });
    }

    pub fn dig(&mut self, row: isize, column: isize) {
        match self.status {
            GameStatus::Idle => {
                self.start(row, column);
            }

            GameStatus::GameOver | GameStatus::Victory => {
                return;
            }

            _ => {}
        }

        self.dig_inner(row, column);
        self.update_score();
    }

    fn dig_inner(&mut self, row: isize, column: isize) {
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
                        self.dig_inner(row + row_offset, column + column_offset);
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
                                    self.dig_inner(row + row_offset, column + column_offset);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn flag(&mut self, row: isize, column: isize) {
        if matches!(self.status, GameStatus::GameOver | GameStatus::Victory) {
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
