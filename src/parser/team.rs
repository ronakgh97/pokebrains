use anyhow::Result;
use colored::Colorize;

#[derive(Debug, Default)]
pub struct EVs {
    pub hp: u16,
    pub atk: u16,
    pub def: u16,
    pub spa: u16,
    pub spd: u16,
    pub spe: u16,
}

#[derive(Debug, Default)]
pub struct Pokemon {
    pub name: String,
    pub species: Option<String>,
    pub item: Option<String>,
    pub ability: Option<String>,
    pub nature: Option<String>,
    pub gender: Option<String>,
    pub evs: EVs,
    pub ivs: Option<EVs>,
    pub shiny: Option<bool>,
    pub level: Option<u8>,
    pub happiness: Option<u8>,
    pub moves: Vec<String>,
}

pub struct Team {
    pub pokemon: Vec<Pokemon>,
}

impl Team {
    /// Deserialize a team from Pokémon Showdown text format
    pub async fn deserialize(input: &str) -> Self {
        let mut pokemon = Vec::new();
        let mut current_pokemon: Option<Pokemon> = None;

        for line in input.lines() {
            let mut line = line.trim();

            // Strip leading '#' if present
            if line.starts_with('#') {
                line = &line[1..];
            }

            // Skip empty lines
            if line.is_empty() {
                if let Some(pkmn) = current_pokemon.take() {
                    pokemon.push(pkmn);
                }
                continue;
            }

            // Parse move lines
            if line.starts_with('-') {
                if let Some(ref mut pkmn) = current_pokemon {
                    let move_name = line.trim_start_matches('-').trim();
                    pkmn.moves.push(move_name.to_string());
                }
                continue;
            }

            // Parse EVs line
            if line.starts_with("EVs:") {
                if let Some(ref mut pkmn) = current_pokemon {
                    pkmn.evs = Self::parse_evs(line);
                }
                continue;
            }

            // Parse Ability line
            if line.starts_with("Ability:") {
                if let Some(ref mut pkmn) = current_pokemon {
                    pkmn.ability = Some(line.trim_start_matches("Ability:").trim().to_string());
                }
                continue;
            }

            // Parse Nature line
            if line.ends_with("Nature") {
                if let Some(ref mut pkmn) = current_pokemon {
                    pkmn.nature = Some(line.trim_end_matches("Nature").trim().to_string());
                }
                continue;
            }

            // Parse first line (Pokemon name, item, gender)
            if !line.starts_with('-')
                && !line.starts_with("EVs:")
                && !line.starts_with("Ability:")
                && !line.ends_with("Nature")
            {
                // Save previous pokemon
                if let Some(pkmn) = current_pokemon.take() {
                    pokemon.push(pkmn);
                }

                let (name, item, gender) = Self::parse_first_line(line);
                current_pokemon = Some(Pokemon {
                    name,
                    item,
                    ability: None,
                    nature: None,
                    gender,
                    evs: EVs::default(),
                    moves: Vec::new(),
                    species: None,
                    ivs: None,
                    shiny: None,
                    level: None,
                    happiness: None,
                });
            }
        }

        // Don't forget the last Pokémon
        if let Some(pkmn) = current_pokemon {
            pokemon.push(pkmn);
        }

        Team { pokemon }
    }

    /// Serialize the team back to Pokémon Showdown text format
    pub fn serialize(&self) -> String {
        let mut output = String::new();

        for (idx, pkmn) in self.pokemon.iter().enumerate() {
            // Add blank line between Pokemon (except before the first one)
            if idx > 0 {
                output.push('\n');
            }

            // First line: Name (Gender) @ Item
            output.push_str(&pkmn.name);
            if let Some(ref gender) = pkmn.gender {
                output.push_str(&format!(" ({})", gender));
            }
            if let Some(ref item) = pkmn.item {
                output.push_str(&format!(" @ {}", item));
            }
            output.push('\n');

            // Ability line
            if let Some(ref ability) = pkmn.ability {
                output.push_str(&format!("Ability: {}\n", ability));
            }

            // EVs line (only if any EVs are set)
            let ev_parts: Vec<String> = [
                (pkmn.evs.hp, "HP"),
                (pkmn.evs.atk, "Atk"),
                (pkmn.evs.def, "Def"),
                (pkmn.evs.spa, "SpA"),
                (pkmn.evs.spd, "SpD"),
                (pkmn.evs.spe, "Spe"),
            ]
            .iter()
            .filter(|(val, _)| *val > 0)
            .map(|(val, stat)| format!("{} {}", val, stat))
            .collect();

            if !ev_parts.is_empty() {
                output.push_str(&format!("EVs: {}\n", ev_parts.join(" / ")));
            }

            // Nature line
            if let Some(ref nature) = pkmn.nature {
                output.push_str(&format!("{} Nature\n", nature));
            }

            // Move lines
            for mv in &pkmn.moves {
                output.push_str(&format!("- {}\n", mv));
            }
        }

        output
    }

    pub async fn deserialize_from_file(path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        Ok(Self::deserialize(&content).await)
    }

    /// Deserialize a team from Pokémon Showdown packed format (official spec)
    pub fn deserialize_packed(input: &str) -> Self {
        let mut pokemon = Vec::new();
        for mon in input
            .lines()
            .flat_map(|l| l.split(']'))
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let fields: Vec<&str> = mon.split('|').collect();
            let mut f = fields.clone();
            f.resize(12, "");
            let name = f[0].to_string();
            let species = if f[1].is_empty() {
                None
            } else {
                Some(f[1].to_string())
            };
            let item = if f[2].is_empty() {
                None
            } else {
                Some(f[2].to_string())
            };
            let ability = if f[3].is_empty() {
                None
            } else {
                Some(f[3].to_string())
            };
            let moves = if f[4].is_empty() {
                Vec::new()
            } else {
                f[4].split(',').map(|m| m.to_string()).collect()
            };
            let nature = if f[5].is_empty() {
                None
            } else {
                Some(f[5].to_string())
            };
            let evs = if f[6].is_empty() {
                EVs::default()
            } else {
                let mut vals = f[6]
                    .split(',')
                    .map(|v| v.parse::<u16>().unwrap_or(0))
                    .collect::<Vec<_>>();
                vals.resize(6, 0);
                EVs {
                    hp: vals[0],
                    atk: vals[1],
                    def: vals[2],
                    spa: vals[3],
                    spd: vals[4],
                    spe: vals[5],
                }
            };
            let gender = if f[7].is_empty() {
                None
            } else {
                Some(f[7].to_string())
            };
            let ivs = if f[8].is_empty() {
                None
            } else {
                let mut vals = f[8]
                    .split(',')
                    .map(|v| v.parse::<u16>().unwrap_or(31))
                    .collect::<Vec<_>>();
                vals.resize(6, 31);
                Some(EVs {
                    hp: vals[0],
                    atk: vals[1],
                    def: vals[2],
                    spa: vals[3],
                    spd: vals[4],
                    spe: vals[5],
                })
            };
            let shiny = match f[9] {
                "S" => Some(true),
                _ => None,
            };
            let level = if f[10].is_empty() {
                None
            } else {
                f[10].parse::<u8>().ok()
            };
            let happiness = if f[11].is_empty() {
                None
            } else {
                f[11].split(',').next().and_then(|h| h.parse::<u8>().ok())
            };
            pokemon.push(Pokemon {
                name,
                species,
                item,
                ability,
                nature,
                gender,
                evs,
                ivs,
                shiny,
                level,
                happiness,
                moves,
            });
        }
        Team { pokemon }
    }

    /// Serialize the team to Pokémon Showdown packed format (official spec, one Pokémon per line)
    pub fn serialize_packed(&self) -> String {
        self.pokemon
            .iter()
            .map(|pkmn| {
                // Normalize helper
                let normalize = |s: &str| -> String {
                    s.chars()
                        .filter(|c| c.is_alphanumeric())
                        .collect::<String>()
                        .to_lowercase()
                };

                // EVs: blank for 0, keep all 6 values
                let evs = [
                    pkmn.evs.hp,
                    pkmn.evs.atk,
                    pkmn.evs.def,
                    pkmn.evs.spa,
                    pkmn.evs.spd,
                    pkmn.evs.spe,
                ]
                .iter()
                .map(|v| {
                    if *v == 0 {
                        "".to_string()
                    } else {
                        v.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(",");

                // IVs: same logic
                let ivs = if let Some(ref iv) = pkmn.ivs {
                    let arr = [iv.hp, iv.atk, iv.def, iv.spa, iv.spd, iv.spe];
                    if arr.iter().all(|&v| v == 31) {
                        "".to_string()
                    } else {
                        arr.iter()
                            .map(|v| {
                                if *v == 31 {
                                    "".to_string()
                                } else {
                                    v.to_string()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(",")
                    }
                } else {
                    "".to_string()
                };

                // Compose fields
                vec![
                    pkmn.name.clone(), // Nickname (usually empty or same as species)
                    normalize(pkmn.species.as_ref().unwrap_or(&pkmn.name)), // Species
                    pkmn.item.as_ref().map(|i| normalize(i)).unwrap_or_default(),
                    pkmn.ability
                        .as_ref()
                        .map(|a| normalize(a))
                        .unwrap_or_default(),
                    pkmn.moves
                        .iter()
                        .map(|m| normalize(m))
                        .collect::<Vec<_>>()
                        .join(","),
                    pkmn.nature.clone().unwrap_or_default(),
                    evs,
                    pkmn.gender.clone().unwrap_or_default(),
                    ivs,
                    if pkmn.shiny.unwrap_or(false) {
                        "S".to_string()
                    } else {
                        "".to_string()
                    },
                    if pkmn.level.unwrap_or(100) == 100 {
                        "".to_string()
                    } else {
                        pkmn.level.unwrap().to_string()
                    },
                    if pkmn.happiness.unwrap_or(255) == 255 {
                        "".to_string()
                    } else {
                        pkmn.happiness.unwrap().to_string()
                    },
                ]
                .join("|")
            })
            .collect::<Vec<_>>()
            .join("]") // Join with ] not \n
    }

    pub fn deserialize_packed_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(Self::deserialize_packed(&content))
    }

    /// Alias for deserialize (backwards compatibility)
    #[deprecated(note = "Use deserialize() instead")]
    pub async fn parse(input: &str) -> Self {
        Self::deserialize(input).await
    }

    pub fn display(&self) {
        println!();
        println!("{}", "POKEMON SHOWDOWN TEAM".bright_white().bold());
        println!();

        for (idx, pkmn) in self.pokemon.iter().enumerate() {
            println!(
                "{} {} {}",
                format!("[{}]", idx + 1).bright_yellow().bold(),
                pkmn.name.bright_cyan().bold(),
                pkmn.gender
                    .as_ref()
                    .map(|g| format!("({})", g).magenta())
                    .unwrap_or_default()
            );

            if let Some(ref item) = pkmn.item {
                println!("  {} {}", "Item:".white().bold(), item.yellow());
            }

            if let Some(ref ability) = pkmn.ability {
                println!("  {} {}", "Ability:".white().bold(), ability.green());
            }

            if let Some(ref nature) = pkmn.nature {
                println!("  {} {}", "Nature:".white().bold(), nature.bright_magenta());
            }

            // Display EVs
            let ev_parts: Vec<String> = [
                (pkmn.evs.hp, "HP"),
                (pkmn.evs.atk, "Atk"),
                (pkmn.evs.def, "Def"),
                (pkmn.evs.spa, "SpA"),
                (pkmn.evs.spd, "SpD"),
                (pkmn.evs.spe, "Spe"),
            ]
            .iter()
            .filter(|(val, _)| *val > 0)
            .map(|(val, stat)| format!("{} {}", val.to_string().bright_blue(), stat.white()))
            .collect();

            if !ev_parts.is_empty() {
                println!("  {} {}", "EVs:".white().bold(), ev_parts.join(" / "));
            }

            // Display moves
            if !pkmn.moves.is_empty() {
                println!("  {}", "Moves:".white().bold());
                for mv in &pkmn.moves {
                    println!("    {} {}", "•".bright_white(), mv.cyan());
                }
            }

            println!();
        }

        println!(
            "{} {}",
            "Total Pokemon:".bright_white().bold(),
            self.pokemon.len().to_string().bright_yellow().bold()
        );
        println!();
    }

    fn parse_first_line(line: &str) -> (String, Option<String>, Option<String>) {
        let mut item = None;
        let mut gender = None;

        // Check for gender in parentheses
        let working_line = if let Some(start) = line.find('(') {
            if let Some(end) = line.find(')') {
                gender = Some(line[start + 1..end].to_string());
                format!("{}{}", &line[..start].trim(), &line[end + 1..].trim())
            } else {
                line.to_string()
            }
        } else {
            line.to_string()
        };

        // Split by @ for item
        let name = if let Some(at_pos) = working_line.find('@') {
            item = Some(working_line[at_pos + 1..].trim().to_string());
            working_line[..at_pos].trim().to_string()
        } else {
            working_line.trim().to_string()
        };

        (name, item, gender)
    }

    fn parse_evs(line: &str) -> EVs {
        let mut evs = EVs::default();
        let ev_str = line.trim_start_matches("EVs:").trim();

        for part in ev_str.split('/') {
            let part = part.trim();
            let tokens: Vec<&str> = part.split_whitespace().collect();

            if tokens.len() >= 2
                && let Ok(value) = tokens[0].parse::<u16>()
            {
                match tokens[1] {
                    "HP" => evs.hp = value,
                    "Atk" => evs.atk = value,
                    "Def" => evs.def = value,
                    "SpA" => evs.spa = value,
                    "SpD" => evs.spd = value,
                    "Spe" => evs.spe = value,
                    _ => {}
                }
            }
        }

        evs
    }
}
