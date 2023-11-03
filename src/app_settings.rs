use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::game_logic::GameParamsError;
use cfg_if::cfg_if;

#[derive(Copy, Clone, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dark => "dark",
                Self::Light => "light",
            }
        )
    }
}

impl FromStr for Theme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dark" => Ok(Self::Dark),
            "light" => Ok(Self::Light),
            _ => Err(()),
        }
    }
}

const EASY: &str = "easy";
const NORMAL: &str = "normal";
const HARD: &str = "hard";

#[derive(PartialEq, Copy, Clone, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Normal,
    Hard,
}

impl FromStr for Difficulty {
    type Err = GameParamsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            EASY => Self::Easy,
            NORMAL => Self::Normal,
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
                Self::Normal => NORMAL,
                Self::Hard => HARD,
            }
        )
    }
}

const SMALL: &str = "small";
const MEDIUM: &str = "medium";
const LARGE: &str = "large";

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

cfg_if! { if #[cfg(feature = "ssr")] {

pub fn fetch_setting<T: FromStr + Default>(setting: &str) -> Option<T> {
    leptos::use_context().and_then(|leptos_axum::RequestParts { headers, ..}| {
        let jar = axum_extra::extract::CookieJar::from_headers(&headers);
        jar.get(setting).and_then(|cookie| cookie.value().parse().ok())
    })
}

pub fn apply_setting<T: Display>(_setting: &str, _value: &T) {
    unimplemented!()
}

} else if #[cfg(target_arch = "wasm32")] {

pub fn fetch_setting<T: FromStr>(setting: &str) -> Option<T> {
    Some(wasm_cookies::get(setting)?.ok()?.parse().ok()?)
}

pub fn apply_setting<T: Display>(setting: &str, value: &T) {
    wasm_cookies::set(
        setting,
        &value.to_string(),
        &wasm_cookies::CookieOptions::default()
            .expires_after(chrono::Duration::weeks(999).to_std().expect("convert to std duration")));
}

} else {
// stubs for rust-analyzer, shouldn't actually get called

pub fn fetch_setting<T: FromStr + Default>(_setting: &str) -> Option<T> {
    Default::default()
}

pub fn apply_setting<T: Display>(_setting: &str, _value: &T) {
    unimplemented!()
}

}}
