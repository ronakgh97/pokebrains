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
#[derive(Clone, Debug, Default)]
pub struct Team {
    pub player: String,
    pub pokemon: Vec<String>,
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

    fn add_setup(&mut self, event: &str) {
        if let Some(title) = parse_title(event) {
            if !self.init.is_empty() {
                self.init.push('\n');
            }
            self.init.push_str(&title);
        }

        // Parse player slot first
        self.parse_player_slot(event);

        // Then handle player names
        if let Some((slot, username)) = parse_player(event) {
            if slot == "p1" {
                self.team[0].player = username;
            } else if slot == "p2" {
                self.team[1].player = username;
            }

            // Check if we've found both players and can match username
            if !self.team[0].player.is_empty()
                && !self.team[1].player.is_empty()
                && self.user_slot.is_none()
            {
                panic!(
                    "WRONG USERNAME: Could not match username '{}' to either '{}' or '{}'",
                    self.assist, self.team[0].player, self.team[1].player
                );
            }
        }

        if let Some((player_id, pokemon_list)) = parse_team_setup_by_player(event) {
            // Add Pokémon to the correct team based on player_id
            if player_id == "p1" {
                self.team[0].pokemon.extend(pokemon_list);
            } else if player_id == "p2" {
                self.team[1].pokemon.extend(pokemon_list);
            }
        }
        if let Some(start_msg) = parse_start(event) {
            // Final check before battle starts
            if self.user_slot.is_none() {
                panic!(
                    "WRONG USERNAME: Could not match username '{}' to either '{}' or '{}'",
                    self.assist, self.team[0].player, self.team[1].player
                );
            }

            self.init.push_str(&start_msg);

            // Add user context when battle starts
            if !self.battle_started {
                self.init.push('\n');
                self.init
                    .push_str(&format!("You are assisting: {}", self.assist));
            }

            self.battle_started = true;
        }
    }

    fn add_turns(&mut self, event: &str) {
        // Handle turn markers
        if event.contains("|turn|") {
            // Save previous turn if exists
            if !self.event_buffer.is_empty() {
                self.events.push(self.event_buffer.clone());
                self.event_buffer.clear();
            }
            // Add turn marker to new buffer
            if let Some(parsed) = parse_battle_log(event, &self.user_slot, &self.team) {
                self.event_buffer.push(parsed);
            }
            return;
        }

        // Add event to current turn buffer
        if let Some(parsed) = parse_battle_log(event, &self.user_slot, &self.team) {
            self.event_buffer.push(parsed);

            // Check if this is a game-ending event
            if event.contains("|win|") || event.contains("|tie|") {
                // Flush the buffer immediately for game-ending events
                if !self.event_buffer.is_empty() {
                    self.events.push(self.event_buffer.clone());
                    self.event_buffer.clear();
                }
            }
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

            // Case-insensitive match with trimming on both sides
            if username.to_lowercase().trim().to_lowercase() == self.assist.trim().to_lowercase() {
                self.user_slot = Some(player_slot.to_string());
            }
        }
    }

    /// Main entry point for adding battle events - routes to setup or turns based on battle state
    pub fn add_event(&mut self, event: &str) {
        if !self.battle_started {
            self.add_setup(event);
        } else {
            self.add_turns(event);
        }
    }
}

/// Replace p1/p2 player IDs with [Assist]/[Against] labels with usernames
fn replace_player_ids(text: &str, user_slot: &Option<String>, team: &[Team; 2]) -> String {
    let Some(slot) = user_slot else {
        return text.to_string(); // No slot detected yet, return unchanged
    };

    let (you_prefix, opp_prefix, your_name, opp_name) = if slot == "p1" {
        ("p1", "p2", &team[0].player, &team[1].player)
    } else {
        ("p2", "p1", &team[1].player, &team[0].player)
    };

    let mut result = text.to_string();

    // Replace player slot variants (a, b for singles/doubles/triples)
    for suffix in ['a', 'b'] {
        // With colon (e.g., "p1a: Pikachu")
        result = result.replace(
            &format!("{}{}:", you_prefix, suffix),
            &format!("[Assist: {}]", your_name),
        );
        result = result.replace(
            &format!("{}{}:", opp_prefix, suffix),
            &format!("[Against: {}]", opp_name),
        );
    }

    // Handle bare p1:/p2: (side conditions like Stealth Rock)
    result = result.replace(
        &format!("{}: ", you_prefix),
        &format!("[Assist: {}] ", your_name),
    );
    result = result.replace(
        &format!("{}: ", opp_prefix),
        &format!("[Against: {}] ", opp_name),
    );
    // Also without space after colon
    result = result.replace(
        &format!("{}:", you_prefix),
        &format!("[Assist: {}]", your_name),
    );
    result = result.replace(
        &format!("{}:", opp_prefix),
        &format!("[Against: {}]", opp_name),
    );

    result
}

pub fn parse_player(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 4 {
        return None;
    }

    if parts[1] == "player" {
        let slot = parts[2].to_string(); // "p1" or "p2"
        let username = parts[3].trim().to_string(); // Trim whitespace from username
        Some((slot, username))
    } else {
        None
    }
}

/// Excepted return format: (player_slot, [Pokémon_names])
pub fn parse_team_setup_by_player(line: &str) -> Option<(String, Vec<String>)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 4 {
        return None;
    }

    if parts[1] == "poke" {
        let slot = parts[2].to_string(); // "p1a", "p2b", etc.
        let details = parts[3].to_string(); // e.g., "Pikachu, L50, M"

        // Extract player ID (p1 or p2) from slot (p1a, p2b, etc.)
        let player_id = slot.chars().take(2).collect::<String>();

        // Extract Pokémon name (first part before comma)
        let pokemon_name = details.split(',').next().unwrap_or(&details).to_string();

        Some((player_id, vec![pokemon_name]))
    } else {
        None
    }
}

pub fn parse_title(line: &str) -> Option<String> {
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
        _ => None,
    }
}

pub fn parse_start(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 2 {
        return None;
    }

    if parts[1] == "start" {
        Some("Battle started".to_string())
    } else {
        None
    }
}

pub fn parse_battle_log(
    line: &str,
    user_slot: &Option<String>,
    team: &[Team; 2],
) -> Option<String> {
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

        // TODO: Handle team battles (not implemented yet)
        // TODO: First switch implementation, mixed up of normal switch and drag
        "switch" | "drag" => {
            if parts.len() >= 4 {
                let pokemon_id = parts[2]; // e.g., "p1a: Aurorus" or "p1a: BigFist"
                let details = parts[3]; // e.g., "Aurorus, L86, F"
                let hp = parts.get(4).unwrap_or(&"100/100");

                // Extract just the Pokémon name (species)
                let pokemon_name = details.split(',').next().unwrap_or(details).trim();

                // Extract nickname (after colon and space, or just species if no nickname)
                let nickname = if let Some(idx) = pokemon_id.find(':') {
                    pokemon_id[idx + 1..].trim()
                } else {
                    pokemon_id.trim()
                };

                if nickname == pokemon_name {
                    // No nickname, or nickname is same as species
                    Some(format!(
                        "{} switched in {} ({})",
                        pokemon_id, pokemon_name, hp
                    ))
                } else {
                    // Nickname is different from species
                    Some(format!(
                        "{} switched to {} ({})",
                        nickname, pokemon_name, hp
                    ))
                }
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
                    .map(|s| {
                        let s = s.trim();
                        if let Some(rest) = s.strip_prefix("[from] ") {
                            format!(" (from {})", rest)
                        } else {
                            format!(" ({})", s)
                        }
                    })
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
    result.map(|s| replace_player_ids(&s, user_slot, team))
}
