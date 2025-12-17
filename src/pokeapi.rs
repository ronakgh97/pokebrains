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
    #[serde(skip)]
    pub effect: Option<String>, // Added to store fetched effect
}

#[derive(Debug, Deserialize)]
pub struct AbilityDetails {
    #[allow(dead_code)]
    name: String,
    pub effect_entries: Vec<AbilityEffectEntry>,
}

#[derive(Debug, Deserialize)]
pub struct AbilityEffectEntry {
    #[allow(dead_code)]
    effect: String,
    pub short_effect: String,
    pub language: NamedAPIResource,
}

#[derive(Debug, Deserialize)]
pub struct PokemonMoveSlot {
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
        let mut pokemon: PokemonInfo = response.json().await?;

        // Fetch ability details for each ability
        for ability_slot in &mut pokemon.abilities {
            let ability_url = format!(
                "https://pokeapi.co/api/v2/ability/{}",
                ability_slot.ability.name
            );
            let response_ability = reqwest::get(&ability_url).await;
            if let Ok(resp) = response_ability {
                if resp.status().is_success() {
                    if let Ok(ability_details) = resp.json::<AbilityDetails>().await {
                        // Find the English effect entry
                        if let Some(entry) = ability_details
                            .effect_entries
                            .iter()
                            .find(|e| e.language.name == "en")
                        {
                            ability_slot.effect = Some(entry.short_effect.clone());
                        }
                    }
                }
            }
        }
        Ok(pokemon)
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch data for {}: {}",
            pokemon_name,
            response.status()
        ))
    }
}
