use crate::api::tools_registry::Tool;
use crate::parser::team::{EVs, Pokemon};
use crate::{PokemonInfo, Team};
use anyhow::anyhow;
use serde_json::Value;

/// A tool to fetch Pokémon details from the PokeAPI
pub struct PokeAPITool;

#[async_trait::async_trait]
impl Tool for PokeAPITool {
    fn name(&self) -> &str {
        "get_pokemon_details"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
              "name": self.name(),
              "description": "Fetches a pokemon details from the PokeAPI",
              "parameters": {
                "type": "object",
                "properties": {
                  "pokemon": {
                    "type": "string",
                    "description": "Exact Pokemon Name"
                  }
                },
                "required": ["pokemon"]
              }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, args: Value) -> anyhow::Result<String> {
        let pokemon = args
            .get("pokemon")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'pokemon' argument"))?;

        let pokemon_data = crate::pokeapi::fetch_pokemon_info(pokemon).await?;

        let str = PokemonInfo::to_readable_form(&pokemon_data);

        Ok(str)
    }
}

pub struct PokemonShowdownTeamGeneratorTool;

#[async_trait::async_trait]
impl Tool for PokemonShowdownTeamGeneratorTool {
    fn name(&self) -> &str {
        "generate_pokemon_showdown_team"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Takes 6 Pokemon with name, item, ability, nature, EVs and 4 moves each, and returns a valid Pokemon Showdown team text.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "team": {
                            "type": "array",
                            "minItems": 6,
                            "maxItems": 6,
                            "items": {
                                "type": "object",
                                "properties": {
                                    "name":   { "type": "string", "description": "Pokemon species name" },
                                    "item":   { "type": "string", "description": "Held item name" },
                                    "ability":{ "type": "string", "description": "Ability name" },
                                    "nature": { "type": "string", "description": "Nature name, e.g. 'Jolly'" },
                                    "gender": { "type": "string", "description": "Optional gender, e.g. 'M' or 'F'", "nullable": true },
                                    "evs": {
                                        "type": "object",
                                        "description": "EVs per stat, values 0–252",
                                        "properties": {
                                            "hp":  { "type": "integer", "default": 0 },
                                            "atk": { "type": "integer", "default": 0 },
                                            "def": { "type": "integer", "default": 0 },
                                            "spa": { "type": "integer", "default": 0 },
                                            "spd": { "type": "integer", "default": 0 },
                                            "spe": { "type": "integer", "default": 0 }
                                        }
                                    },
                                    "moves": {
                                        "type": "array",
                                        "description": "List of 1–4 moves",
                                        "minItems": 1,
                                        "maxItems": 4,
                                        "items": { "type": "string" }
                                    }
                                },
                                "required": ["name", "item", "ability", "nature", "evs", "moves"]
                            }
                        }
                    },
                    "required": ["team"]
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        false
    }

    async fn execute_tool(&self, args: Value) -> anyhow::Result<String> {
        let team_array = args
            .get("team")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("Missing or invalid 'team' array"))?;

        if team_array.len() != 6 {
            return Err(anyhow!("Team must contain exactly 6 Pokemon"));
        }

        let mut mons: Vec<Pokemon> = Vec::with_capacity(6);

        for mon in team_array {
            let name = mon
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Each pokemon must have a 'name'"))?
                .to_string();

            let item = mon
                .get("item")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let ability = mon
                .get("ability")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let nature = mon
                .get("nature")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let gender = mon
                .get("gender")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // EVs
            let evs_json = mon
                .get("evs")
                .ok_or_else(|| anyhow!("Each pokemon must have 'evs'"))?;

            let evs = EVs {
                hp: evs_json.get("hp").and_then(|v| v.as_u64()).unwrap_or(0) as u16,
                atk: evs_json.get("atk").and_then(|v| v.as_u64()).unwrap_or(0) as u16,
                def: evs_json.get("def").and_then(|v| v.as_u64()).unwrap_or(0) as u16,
                spa: evs_json.get("spa").and_then(|v| v.as_u64()).unwrap_or(0) as u16,
                spd: evs_json.get("spd").and_then(|v| v.as_u64()).unwrap_or(0) as u16,
                spe: evs_json.get("spe").and_then(|v| v.as_u64()).unwrap_or(0) as u16,
            };

            // Moves
            let moves_json = mon
                .get("moves")
                .and_then(|v| v.as_array())
                .ok_or_else(|| anyhow!("Each pokemon must have 'moves' array"))?;

            if moves_json.is_empty() {
                return Err(anyhow!("Each pokemon must have at least 1 move"));
            }

            let mut moves: Vec<String> = Vec::with_capacity(moves_json.len());
            for mv in moves_json {
                let mv_name = mv
                    .as_str()
                    .ok_or_else(|| anyhow!("Move names must be strings"))?;
                moves.push(mv_name.to_string());
            }

            mons.push(Pokemon {
                name,
                species: None,
                item,
                ability,
                nature,
                gender,
                evs,
                ivs: None,
                shiny: None,
                level: None,
                happiness: None,
                moves,
            });
        }

        let team = Team { pokemon: mons };
        let showdown_text = team.serialize();
        Ok(showdown_text)
    }
}
