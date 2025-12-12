use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PokemonInfo {
    pub id: i32,
    pub name: String,
    pub height: i32,
    pub weight: i32,
    pub types: Vec<PokemonTypeSlot>,
    pub abilities: Vec<PokemonAbilitySlot>,
    pub moves: Vec<PokemonMoveSlot>,
    pub stats: Vec<PokemonStat>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonTypeSlot {
    pub r#type: NamedAPIResource,
}

#[derive(Debug, Deserialize)]
pub struct PokemonAbilitySlot {
    pub is_hidden: bool,
    pub ability: NamedAPIResource,
}

#[derive(Debug, Deserialize)]
pub struct PokemonMoveSlot { //<- Maybe too much context
    pub r#move: NamedAPIResource,
}

#[derive(Debug, Deserialize)]
pub struct PokemonStat {
    pub base_stat: i32,
    pub stat: NamedAPIResource,
}

#[derive(Debug, Deserialize)]
pub struct NamedAPIResource {
    pub name: String,
}

pub async fn fetch_pokemon_info(pokemon_name: &str) -> Result<PokemonInfo> {
    let url = format!(
        "https://pokeapi.co/api/v2/pokemon/{}",
        pokemon_name.to_lowercase()
    );
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let pokemon: PokemonInfo = response.json().await?;
        Ok(pokemon)
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch data for {}: {}",
            pokemon_name,
            response.status()
        ))
    }
}
