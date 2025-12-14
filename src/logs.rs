#[derive(Clone, Debug)]
pub struct BattleEvents {
    pub team: [Team; 2], //<- Unused
    pub init: String,
    pub assist: String,
    pub user_slot: Option<String>, // "p1" or "p2" - which player slot the user is
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
    pub fn new(user: String) -> Self {
        BattleEvents {
            team: [Team::default(), Team::default()], //<- Unused
            init: String::new(),
            assist: user,
            user_slot: None,
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
            if let Some(parsed) = parse_battle_log(event, &self.user_slot) {
                self.event_buffer.push(parsed);
            }
            return;
        }

        // Before first turn = init phase
        if !self.battle_started {
            // Parse |player| messages to detect which slot the user is
            self.parse_player_slot(event);

            if let Some(parsed) = parse_init(event) {
                self.init.push_str(&parsed);
                self.init.push('\n');

                // Add user context once when battle starts
                if parsed.contains("Battle started") && !self.init.contains("You are assisting") {
                    let slot_info = match &self.user_slot {
                        Some(slot) => format!(" (you are {})", slot),
                        None => {
                            // Couldn't match username - warn user
                            panic!("WRONG USERNAME: Could not match username")
                        }
                    };
                    self.init.push_str(&format!(
                        "You are assisting: {}{}\n",
                        self.assist, slot_info
                    ));
                }
            }
            return;
        }

        // During battle = add to current turn buffer
        if let Some(parsed) = parse_battle_log(event, &self.user_slot) {
            self.event_buffer.push(parsed);
        }
    }

    /// Parse |player| messages to detect which player slot (p1 or p2) the user is
    /// Format: |player|p1|username|avatar|rating
    fn parse_player_slot(&mut self, line: &str) {
        if self.user_slot.is_some() {
            return; // Already detected
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 4 && parts[1] == "player" {
            let player_slot = parts[2]; // "p1" or "p2"
            let username = parts[3].trim();

            // Case-insensitive match
            if username.to_lowercase() == self.assist.trim().to_lowercase() {
                self.user_slot = Some(player_slot.to_string());
            }
        }
    }
}

/// Replace p1/p2 player IDs with [You]/[Opponent] labels
fn replace_player_ids(text: &str, user_slot: &Option<String>) -> String {
    let Some(slot) = user_slot else {
        return text.to_string(); // No slot detected yet, return unchanged
    };

    let (you_prefix, opp_prefix) = if slot == "p1" {
        ("p1", "p2")
    } else {
        ("p2", "p1")
    };

    let mut result = text.to_string();

    // Replace player slot variants (a, b for singles/doubles/triples)
    for suffix in ['a', 'b'] {
        result = result.replace(&format!("{}{}:", you_prefix, suffix), "[Assist]");
        result = result.replace(&format!("{}{}:", opp_prefix, suffix), "[Against]");
    }

    // Handle bare p1:/p2: (side conditions like Stealth Rock)
    result = result.replace(&format!("{}: ", you_prefix), "[Assist] ");
    result = result.replace(&format!("{}: ", opp_prefix), "[Against] ");
    // Also without space after colon
    result = result.replace(&format!("{}:", you_prefix), "[Assist]");
    result = result.replace(&format!("{}:", opp_prefix), "[Against]");

    result
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

pub fn parse_battle_log(line: &str, user_slot: &Option<String>) -> Option<String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 2 {
        return None;
    }

    let result = match parts[1] {
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

                // Extract just the Pokémon name
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
            if parts.len() >= 4 {
                let side = parts[2].split(':').next().unwrap_or(parts[2]);
                let condition = parts[3].split(':').last().unwrap_or(parts[3]);
                Some(format!("{} set up {}", side, condition.trim()))
            } else {
                None
            }
        }

        "-sideend" => {
            if parts.len() >= 4 {
                let side = parts[2].split(':').next().unwrap_or(parts[2]);
                let condition = parts[3].split(':').last().unwrap_or(parts[3]);
                Some(format!("{}'s {} wore off", side, condition.trim()))
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
    };

    // Apply player ID replacement
    result.map(|s| replace_player_ids(&s, user_slot))
}
