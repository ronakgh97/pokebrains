use colored::Colorize;

#[derive(Debug, Default)]
pub struct EVs {
    hp: u16,
    atk: u16,
    def: u16,
    spa: u16,
    spd: u16,
    spe: u16,
}

#[derive(Debug)]
pub struct Pokemon {
    name: String,
    item: Option<String>,
    ability: Option<String>,
    nature: Option<String>,
    gender: Option<String>,
    evs: EVs,
    moves: Vec<String>,
}

pub struct Team {
    pokemon: Vec<Pokemon>,
}

impl Team {
    pub fn parse(input: &str) -> Self {
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
                });
            }
        }

        // Don't forget the last Pokémon
        if let Some(pkmn) = current_pokemon {
            pokemon.push(pkmn);
        }

        Team { pokemon }
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
            let ev_parts: Vec<String> = vec![
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

            if tokens.len() >= 2 {
                if let Ok(value) = tokens[0].parse::<u16>() {
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
        }

        evs
    }
}
