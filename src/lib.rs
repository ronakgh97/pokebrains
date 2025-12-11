mod ai;
mod client;
mod logs;
mod pokeapi;
mod tools;

pub use anyhow::Result;
pub use client::BattleClient;
pub use colored::Colorize;

pub use pokeapi::fetch_pokemon_info;
