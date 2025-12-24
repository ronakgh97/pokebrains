use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonInfo {
    pub id: i32,
    pub name: String,
    pub height: i32,
    pub weight: i32,
    pub types: Vec<PokemonTypeSlot>,
    pub abilities: Vec<PokemonAbilitySlot>,
    pub moves: Vec<PokemonMoveSlot>,
    pub stats: Vec<PokemonStat>,
    pub species: NamedAPIResource,
    #[serde(rename = "past_abilities")]
    pub r#gen: Vec<PastAbilityGen>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonSpecies {
    pub generation: NamedAPIResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PastAbilityGen {
    pub generation: NamedAPIResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonTypeSlot {
    pub r#type: NamedAPIResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonAbilitySlot {
    pub is_hidden: bool,
    pub ability: NamedAPIResource,
    #[serde(skip)]
    pub effect: Option<String>, // Added to store fetched effect
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityDetails {
    #[allow(dead_code)]
    name: String,
    pub effect_entries: Vec<AbilityEffectEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityEffectEntry {
    #[allow(dead_code)]
    effect: String,
    pub short_effect: String,
    language: NamedAPIResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonMoveSlot {
    pub r#move: NamedAPIResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonStat {
    pub base_stat: i32,
    pub stat: NamedAPIResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedAPIResource {
    pub name: String,
    url: String,
}

impl PokemonInfo {
    pub fn to_readable_form(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Pokemon: {}\n", self.name.to_uppercase()));
        // Generation
        let generation_name = self
            .r#gen
            .first()
            .map(|g| g.generation.name.to_uppercase())
            .unwrap_or_else(|| "Unknown".to_string());
        s.push_str(&format!("Generation: {}\n", generation_name));
        // Types
        let types: Vec<String> = self.types.iter().map(|t| t.r#type.name.clone()).collect();
        s.push_str(&format!("Types:   {}\n", types.join(", ")));
        // Height & Weight
        s.push_str(&format!(
            "Height: {} | Weight: {}\n\n",
            self.height, self.weight
        ));
        // Stats
        s.push_str("Stats:\n");
        for stat in &self.stats {
            s.push_str(&format!("  {:>15} : {}\n", stat.stat.name, stat.base_stat));
        }
        // Abilities
        s.push_str("\nAbilities:\n");
        for ability in &self.abilities {
            let hidden_text = if ability.is_hidden {
                "(Hidden)"
            } else {
                "        "
            };
            if let Some(effect) = &ability.effect {
                s.push_str(&format!(
                    " {:>15} {} - {}\n",
                    ability.ability.name, hidden_text, effect
                ));
            } else {
                s.push_str(&format!("  {:>15} {}\n", ability.ability.name, hidden_text));
            }
        }
        // Moves
        s.push_str(&format!("\nMoves: (Total: {})\n", self.moves.len()));
        for mv in self.moves.iter().take(10) {
            s.push_str(&format!("  - {}\n", mv.r#move.name));
        }
        if self.moves.len() > 10 {
            s.push_str(&format!("  ... and {} more\n", self.moves.len() - 10));
        }
        s
    }
}

pub async fn fetch_pokemon_info(pokemon_name: &str) -> Result<PokemonInfo> {
    let url = format!(
        "https://pokeapi.co/api/v2/pokemon/{}",
        pokemon_name.to_lowercase()
    );
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let mut pokemon: PokemonInfo = response.json().await?;

        // Fetch generation from species if past_abilities is empty
        if pokemon.r#gen.is_empty() {
            if let Ok(species_resp) = reqwest::get(&pokemon.species.url).await {
                if species_resp.status().is_success() {
                    if let Ok(species_data) = species_resp.json::<PokemonSpecies>().await {
                        // Create a PastAbilityGen from species generation
                        pokemon.r#gen.push(PastAbilityGen {
                            generation: species_data.generation,
                        });
                    }
                }
            }
        }

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

pub async fn pretty_display(pokemon_name: &str) -> Result<()> {
    let data = fetch_pokemon_info(pokemon_name).await;
    match data {
        Ok(info) => {
            println!();
            println!(
                "{} {}",
                "Pokemon:".cyan().bold(),
                info.name.to_uppercase().bright_white().bold()
            );

            println!(
                "{} {}",
                "Generation:".cyan().bold(),
                info.r#gen
                    .first()
                    .map(|g| g.generation.name.to_uppercase().yellow().to_string())
                    .unwrap_or_else(|| "Unknown".yellow().to_string())
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
                    "".normal()
                };
                if let Some(effect) = &ability.effect {
                    println!(
                        " {:>15} - {} {}",
                        ability.ability.name.white(),
                        effect.bright_blue(),
                        hidden_text
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
