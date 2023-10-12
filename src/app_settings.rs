use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::game_logic::{Difficulty, Size};
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

#[derive(Copy, Clone, Default)]
pub struct Settings {
    pub theme: Theme,
    pub difficulty: Difficulty,
    pub size: Size,
}

impl Settings {
    cfg_if! { if #[cfg(feature = "ssr")] {

    pub fn fetch() -> Self {
        if let Some(leptos_axum::RequestParts { headers, ..}) = leptos::use_context() {
            let jar = axum_extra::extract::CookieJar::from_headers(&headers);

            Self {
                theme: jar.get("theme").map_or(Default::default(), |cookie|
                    cookie.value().parse().unwrap_or_default()),
                difficulty: jar.get("difficulty").map_or(Default::default(), |cookie|
                    cookie.value().parse().unwrap_or_default()),
                size: jar.get("size").map_or(Default::default(), |cookie|
                    cookie.value().parse().unwrap_or_default()),
            }
        } else {
            Self::default()
        }
    }

    pub fn set<T: Display>(_name: &str, _value: &T) {}

    } else if #[cfg(target_arch = "wasm32")] {

    pub fn fetch() -> Self {
        let theme = if let Some(Ok(theme)) = wasm_cookies::get("theme") {
            theme.parse().unwrap_or_default()
        } else if let Ok(Some(mql)) = leptos::window().match_media("(prefers-color-scheme: dark)") {
            if mql.matches() {
                Theme::Dark
            } else {
                Theme::Light
            }
        } else {
            Theme::default()
        };

        let difficulty = if let Some(Ok(difficulty)) = wasm_cookies::get("difficulty") {
            difficulty.parse().unwrap_or_default()
        } else {
            Difficulty::default()
        };

        let size = if let Some(Ok(size)) = wasm_cookies::get("size") {
            size.parse().unwrap_or_default()
        } else {
            Size::default()
        };

        Self {
            theme,
            difficulty,
            size,
        }
    }

    pub fn set<T: Display>(name: &str, value: &T) {
        wasm_cookies::set(
            name,
            &value.to_string(),
            &wasm_cookies::CookieOptions::default()
                .expires_after(chrono::Duration::weeks(999).to_std().expect("convert to std duration")));
    }

    } else {
    // stubs for rust-analyzer, shouldn't actually get called

    pub fn fetch() -> Self {
        Self::default()
    }

    pub fn set<T: Display>(_name: &str, _value: &T) {}

    }}
}
