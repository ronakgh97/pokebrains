#[derive(Clone, Debug)]
pub struct BattleEvents {
    pub team: [Team; 2], //<- Unused
    pub init: String,
    pub event_buffer: Vec<String>,
    pub events: Vec<Vec<String>>,
    pub battle_started: bool,
}

/// This only useful in team battles (currently not implemented)
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct Team {
    pub player: String,
    pub pokemons: Vec<String>,
}

impl BattleEvents {
    pub fn new() -> Self {
        BattleEvents {
            team: [Team::default(), Team::default()], //<- Unused
            init: String::new(),
            event_buffer: Vec::new(),
            events: Vec::new(),
            battle_started: false,
        }
    }

    pub fn add_event(&mut self, event: &str) {
        // Handle turn markers
        if event.contains("|turn|") {
            // Save previous turn if exists
            if !self.event_buffer.is_empty() {
                self.events.push(self.event_buffer.clone());
                self.event_buffer.clear();
            }
            self.battle_started = true;

            // Add turn marker to new buffer
            if let Some(parsed) = parse_battle_log(event) {
                self.event_buffer.push(parsed);
            }
            return;
        }

        // Before first turn = init phase
        if !self.battle_started {
            if let Some(parsed) = parse_init(event) {
                self.init.push_str(&parsed);
                self.init.push('\n');
            }
            return;
        }

        // During battle = add to current turn buffer
        if let Some(parsed) = parse_battle_log(event) {
            self.event_buffer.push(parsed);
        }
    }
}

pub fn parse_init(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 2 {
        return None;
    }

    match parts[1] {
        "title" => {
            if parts.len() >= 3 {
                Some(format!("Battle Title: {}", parts[2]))
            } else {
                None
            }
        }

        "gen" => {
            if parts.len() >= 3 {
                Some(format!("Generation: {}", parts[2]))
            } else {
                None
            }
        }

        "start" => Some("Battle started".to_string()),

        _ => None,
    }
}

pub fn parse_battle_log(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 2 {
        return None;
    }

    match parts[1] {
        "turn" => {
            if parts.len() >= 3 {
                Some(format!(" TURN {} ", parts[2]))
            } else {
                None
            }
        }

        "switch" | "drag" => {
            if parts.len() >= 4 {
                let pokemon_id = parts[2]; // e.g., "p1a: Aurorus"
                let details = parts[3]; // e.g., "Aurorus, L86, F"
                let hp = parts.get(4).unwrap_or(&"100/100");

                // Extract just the pokemon name
                let pokemon_name = details.split(',').next().unwrap_or(details);

                Some(format!(
                    "{} switched to {} ({})",
                    pokemon_id, pokemon_name, hp
                ))
            } else {
                None
            }
        }

        "move" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let move_name = parts[3];
                let target = parts.get(4).unwrap_or(&"");

                if target.is_empty() {
                    Some(format!("{} used {}", pokemon, move_name))
                } else {
                    Some(format!("{} used {} on {}", pokemon, move_name, target))
                }
            } else {
                None
            }
        }

        "-damage" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let hp = parts[3];
                let cause = parts
                    .get(4)
                    .map(|s| format!(" (from {})", s))
                    .unwrap_or_default();

                Some(format!("{} HP: {}{}", pokemon, hp, cause))
            } else {
                None
            }
        }

        "faint" => {
            if parts.len() >= 3 {
                Some(format!("{} fainted!", parts[2]))
            } else {
                None
            }
        }

        "-status" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let status = parts[3];
                Some(format!("{} was inflicted with {}", pokemon, status))
            } else {
                None
            }
        }

        "-curestatus" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let status = parts[3];
                Some(format!("{} cured of {}", pokemon, status))
            } else {
                None
            }
        }

        "-boost" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let stat = parts[3];
                let amount = parts.get(4).unwrap_or(&"1");
                Some(format!("{}'s {} rose by {}", pokemon, stat, amount))
            } else {
                None
            }
        }

        "-unboost" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let stat = parts[3];
                let amount = parts.get(4).unwrap_or(&"1");
                Some(format!("{}'s {} fell by {}", pokemon, stat, amount))
            } else {
                None
            }
        }

        "-heal" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let hp = parts[3];
                let source = parts
                    .get(4)
                    .map(|s| format!(" ({})", s))
                    .unwrap_or_default();
                Some(format!("{} healed to {}{}", pokemon, hp, source))
            } else {
                None
            }
        }

        "-weather" => {
            if parts.len() >= 3 {
                let weather = parts[2];
                if weather == "none" {
                    Some("Weather cleared".to_string())
                } else {
                    Some(format!("Weather: {}", weather))
                }
            } else {
                None
            }
        }

        "-sidestart" => {
            if parts.len() >= 3 {
                let side = parts[2].split(':').next().unwrap_or(parts[2]);
                let condition = parts[2].split('|').last().unwrap_or(parts[2]);
                Some(format!("{} set up {}", side, condition))
            } else {
                None
            }
        }

        "-sideend" => {
            if parts.len() >= 3 {
                let side = parts[2].split(':').next().unwrap_or(parts[2]);
                let condition = parts[2].split('|').last().unwrap_or(parts[2]);
                Some(format!("{}'s {} wore off", side, condition))
            } else {
                None
            }
        }

        "-ability" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let ability = parts[3];
                Some(format!("{}'s ability: {}", pokemon, ability))
            } else {
                None
            }
        }

        "-supereffective" => {
            if parts.len() >= 3 {
                Some(format!("Super effective on {}!", parts[2]))
            } else {
                None
            }
        }

        "-resisted" => {
            if parts.len() >= 3 {
                Some(format!("{} resisted the attack", parts[2]))
            } else {
                None
            }
        }

        "-crit" => {
            if parts.len() >= 3 {
                Some(format!("Critical hit on {}!", parts[2]))
            } else {
                None
            }
        }

        "-immune" => {
            if parts.len() >= 3 {
                Some(format!("{} is immune!", parts[2]))
            } else {
                None
            }
        }

        "cant" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let reason = parts[3];
                Some(format!("{} can't move ({})", pokemon, reason))
            } else {
                None
            }
        }

        "win" => {
            if parts.len() >= 3 {
                Some(format!("{} wins the battle!", parts[2]))
            } else {
                None
            }
        }

        "tie" => Some("Battle ended in a tie".to_string()),

        "-message" => {
            if parts.len() >= 3 {
                Some(format!("{}", parts[2]))
            } else {
                None
            }
        }

        // Ignore these - not relevant for battle strategy
        "player" | "teamsize" | "gametype" | "gen" | "tier" | "rule" | "rated" | "start"
        | "upkeep" | "inactive" | "t:" | "j" | "J" | "l" | "L" | "c" | "c:" | "init" | "title"
        | "raw" | "html" | "request" | "" => None,

        // Log unknown events for debugging
        _ => {
            #[cfg(debug_assertions)]
            eprintln!("[DEBUG] Unknown event type: {}", parts[1]);
            None
        }
    }
}
