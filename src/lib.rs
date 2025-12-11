mod ai;
mod client;
mod parser;
mod pokeapi;
mod tools;

pub use anyhow::Result;
pub use client::BattleClient;
pub use colored::Colorize;

pub use pokeapi::fetch_pokemon_info;
