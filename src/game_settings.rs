use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use cfg_if::cfg_if;
use rand::seq::SliceRandom;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Copy, Clone, Default, Debug)]
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

#[derive(Clone, Debug)]
pub struct Username {
    pub name: String,
    pub stable: bool,
}

fn random_name() -> &'static str {
    include!("../names.json")
        .choose(&mut rand::thread_rng())
        .expect("array is nonempty")
}

impl Username {
    pub fn new(name: String) -> Self {
        Self { name, stable: true }
    }

    pub fn random() -> Self {
        Self {
            name: random_name().into(),
            stable: true,
        }
    }
}

impl From<Option<String>> for Username {
    fn from(value: Option<String>) -> Self {
        if let Some(name) = value {
            Self { name, stable: true }
        } else {
            Self {
                name: random_name().into(),
                stable: false,
            }
        }
    }
}

#[derive(Error, Debug)]
pub struct ParseDifficultyError;

impl Display for ParseDifficultyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing game difficulty")
    }
}

#[derive(PartialEq, Copy, Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    #[default]
    Easy,
    Normal,
    Hard,
}

impl FromStr for Difficulty {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}

#[derive(Error, Debug)]
pub struct ParseSizeError;

impl Display for ParseSizeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing gameboard size")
    }
}

#[derive(PartialEq, Copy, Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    #[default]
    Small,
    Medium,
    Large,
}

impl FromStr for Size {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}

cfg_if! { if #[cfg(feature = "ssr")] {

pub fn fetch_setting<T: FromStr + Default>(setting: &str) -> Option<T> {
    leptos::use_context().and_then(|leptos_axum::RequestParts { headers, ..}| {
        let jar = axum_extra::extract::CookieJar::from_headers(&headers);
        jar.get(setting).and_then(|cookie| cookie.value().parse().ok())
    })
}

pub fn apply_setting<T: ToString>(_setting: &str, _value: &T) {
    unimplemented!()
}

} else if #[cfg(target_arch = "wasm32")] {

pub fn fetch_setting<T: FromStr>(setting: &str) -> Option<T> {
    Some(wasm_cookies::get(setting)?.ok()?.parse().ok()?)
}

pub fn apply_setting<T: ToString>(setting: &str, value: &T) {
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

pub fn apply_setting<T: ToString>(_setting: &str, _value: &T) {
    unimplemented!()
}

}}
