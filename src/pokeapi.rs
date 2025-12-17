use anyhow::Result;
use colored::Colorize;
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
            if let Ok(resp) = response_ability
                && resp.status().is_success()
                && let Ok(ability_details) = resp.json::<AbilityDetails>().await
            {
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
        Ok(pokemon)
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch data for {}: {}",
            pokemon_name,
            response.status()
        ))
    }
}

pub async fn display(pokemon_name: &str) -> Result<()> {
    let data = fetch_pokemon_info(pokemon_name).await;
    match data {
        Ok(info) => {
            println!();
            println!(
                "{} {}",
                "Pokemon:".cyan().bold(),
                info.name.to_uppercase().bright_white().bold()
            );

            print!("{}", "Types:   ".cyan().bold());
            let types: Vec<String> = info
                .types
                .iter()
                .map(|t| t.r#type.name.yellow().to_string())
                .collect();
            println!("{}", types.join(", "));

            println!(
                "{} {} | {} {}",
                "Height:".cyan().bold(),
                info.height.to_string().green(),
                "Weight:".cyan().bold(),
                info.weight.to_string().green()
            );

            println!();
            println!("{}", "Stats:".cyan().bold());
            for stat in info.stats {
                println!(
                    "  {:>15} : {}",
                    stat.stat.name.white(),
                    stat.base_stat.to_string().green().bold()
                );
            }

            println!();
            println!("{}", "Abilities:".cyan().bold());
            for ability in info.abilities {
                let hidden_text = if ability.is_hidden {
                    "(Hidden)".red().italic()
                } else {
                    "        ".normal()
                };
                if let Some(effect) = &ability.effect {
                    println!(
                        " {:>15} {} - {}",
                        ability.ability.name.white(),
                        hidden_text,
                        effect.bright_blue()
                    );
                } else {
                    println!("  {:>15} {}", ability.ability.name.white(), hidden_text);
                }
            }

            println!();
            println!(
                "{} (Total: {})",
                "Moves:".cyan().bold(),
                info.moves.len().to_string().green()
            );
            for mv in info.moves.iter().take(4) {
                println!("  - {}", mv.r#move.name.white());
            }
            if info.moves.len() > 4 {
                println!("  ... and {} more", info.moves.len() - 4);
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error fetching data:".red().bold(), e);
        }
    }
    Ok(())
}
