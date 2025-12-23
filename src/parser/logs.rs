#[derive(Clone, Debug)]
pub struct BattleEvents {
    pub team: [Team; 2], // Two teams: [0] = p1, [1] = p2, Supports only singles for now
    pub init: Vec<Token>, // store Title. Generation, Which player is being assisted
    pub assist: String,  // Which username is being assisted
    pub user_slot: Option<String>, // "p1" or "p2" - which player slot the user is
    pub event_buffer: Vec<Token>, // buffer for current turn events
    pub events: Vec<Vec<Token>>, // store events per turn
    pub battle_started: bool,
    pub is_previewing_team: bool,
    pub is_init_suggestions_generated: bool,
}

/// This only useful in team battles
#[derive(Clone, Debug, Default)]
pub struct Team {
    pub player: String,
    pub slot: String, // "p1" or "p2"
    pub pokemon: Vec<String>,
}
#[derive(Clone, Debug)]
pub enum Token {
    TITLE(String),
    GEN(String),
    PLAYER(String, String),    // (slot, username)
    TEAM(String, Vec<String>), // (player_slot, [Pokémon_names])
    PREVIEW(bool),
    START(bool),
    TURN(usize),
    MOVE(String, String, String, Option<String>), // (slot, Pokémon, move_name, target)
    SWITCH(String, String, String),               // (slot, nickname, species, hp)
    DAMAGE(String, String, String, Option<String>), // (slot, Pokémon, hp, cause)
    FAINT(String),
    STATUS(String, String),
    CURESTATUS(String, String),
    BOOST(String, String, String),
    UNBOOST(String, String, String),
    HEAL(String, String, Option<String>),
    WEATHER(String),
    SIDESTART(String, String),
    SIDEEND(String, String),
    ABILITY(String, String),
    MEGA(String, String, Option<String>),
    SUPEREFFECTIVE(String),
    RESISTED(String),
    MISS(String, String),
    CRIT(String),
    IMMUNE(String),
    CANT(String, String),
    WIN(String),
    TIE,
    MESSAGE(String),
}
impl BattleEvents {
    pub fn new(user: String) -> Self {
        BattleEvents {
            team: [Team::default(), Team::default()],
            init: Vec::new(),
            assist: user,
            user_slot: None,
            event_buffer: Vec::new(),
            events: Vec::new(),
            battle_started: false,
            is_previewing_team: false,
            is_init_suggestions_generated: false,
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

    /// Returns the current turn number (0 if not started)
    pub fn get_current_turn(&self) -> usize {
        // Check buffer first (current turn in progress)
        for token in &self.event_buffer {
            if let Token::TURN(num) = token {
                return *num;
            }
        }
        // Fall back to last completed turn
        for token in self.events.last().iter().flat_map(|v| v.iter()) {
            if let Token::TURN(num) = token {
                return *num;
            }
        }
        0
    }

    /// Check if the battle has ended (win or tie)
    pub fn is_battle_ended(&self) -> bool {
        // Check in buffer first
        for token in &self.event_buffer {
            if matches!(token, Token::WIN(_) | Token::TIE) {
                return true;
            }
        }
        // Check in last completed turn
        if let Some(last_turn) = self.events.last() {
            for token in last_turn {
                if matches!(token, Token::WIN(_) | Token::TIE) {
                    return true;
                }
            }
        }
        false
    }

    fn add_setup(&mut self, event: &str) {
        if let Some(token) = parse_title_and_gen(event) {
            self.init.push(token);
        }

        // Parse player slot first
        self.parse_player_slot(event);

        // Then handle player names
        if let Some(token) = parse_player(event) {
            if let Token::PLAYER(ref slot, ref username) = token {
                if slot == "p1" {
                    self.team[0].player = username.clone();
                    self.team[0].slot = slot.clone();
                } else if slot == "p2" {
                    self.team[1].player = username.clone();
                    self.team[1].slot = slot.clone();
                }

                // Check if we've found both players and can match username
                if !self.team[0].player.is_empty()
                    && !self.team[1].player.is_empty()
                    && self.user_slot.is_none()
                {
                    eprintln!(
                        "WRONG USERNAME: Could not match username '{}' to either '{}' or '{}'",
                        self.assist, self.team[0].player, self.team[1].player
                    );
                    exit(1);
                }
            }
        }

        if let Some(token) = parse_team_setup_by_player_slot(event) {
            if let Token::TEAM(ref player_id, ref pokemon_list) = token {
                // Add Pokémon to the correct team based on player_id
                if player_id == "p1" {
                    self.team[0].pokemon.extend(pokemon_list.clone());
                } else if player_id == "p2" {
                    self.team[1].pokemon.extend(pokemon_list.clone());
                }
            }
        }

        if let Some(token) = parse_start(event) {
            // Final check before battle starts
            if self.user_slot.is_none() {
                eprintln!(
                    "WRONG USERNAME: Could not match username '{}' to either '{}' or '{}'",
                    self.assist, self.team[0].player, self.team[1].player
                );
                exit(1);
            }

            self.init.push(token);
            self.battle_started = true;
        }

        if let Some(token) = parse_is_team_previewing(event) {
            if let Token::PREVIEW(true) = token {
                self.is_previewing_team = true;
                self.init.push(token);
                self.init.push(Token::MESSAGE(format!(
                    "You are assisting: {}",
                    self.assist
                )));
            }
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
            // Parse and add turn marker to new buffer (this also triggers saving the previous turn)
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
}

/// Replace p1/p2 player IDs with [Assist]/[Against] labels with usernames
fn replace_player_ids_in_token(
    token: Token,
    user_slot: &Option<String>,
    team: &[Team; 2],
) -> Token {
    let Some(slot) = user_slot else {
        return token;
    };

    let (you_prefix, opp_prefix, your_name, opp_name) = if slot == "p1" {
        ("p1", "p2", &team[0].player, &team[1].player)
    } else {
        ("p2", "p1", &team[1].player, &team[0].player)
    };

    match token {
        Token::MOVE(s, pokemon, move_name, target) => {
            let new_slot = replace_slot(&s, you_prefix, opp_prefix, your_name, opp_name);
            let new_target =
                target.map(|t| replace_pokemon_id(&t, you_prefix, opp_prefix, your_name, opp_name));
            Token::MOVE(new_slot, pokemon, move_name, new_target)
        }
        Token::SWITCH(s, species, hp) => {
            let new_slot = replace_slot(&s, you_prefix, opp_prefix, your_name, opp_name);
            Token::SWITCH(new_slot, species, hp)
        }
        Token::DAMAGE(s, pokemon, hp, cause) => {
            let new_slot = replace_slot(&s, you_prefix, opp_prefix, your_name, opp_name);
            Token::DAMAGE(new_slot, pokemon, hp, cause)
        }
        Token::HEAL(pokemon, hp, source) => {
            // HEAL also contains Pokémon ID that needs replacing
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::HEAL(new_pokemon, hp, source)
        }
        Token::FAINT(pokemon) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::FAINT(new_pokemon)
        }
        Token::STATUS(pokemon, status) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::STATUS(new_pokemon, status)
        }
        Token::CURESTATUS(pokemon, status) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::CURESTATUS(new_pokemon, status)
        }
        Token::BOOST(pokemon, stat, amount) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::BOOST(new_pokemon, stat, amount)
        }
        Token::UNBOOST(pokemon, stat, amount) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::UNBOOST(new_pokemon, stat, amount)
        }
        Token::ABILITY(pokemon, ability) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::ABILITY(new_pokemon, ability)
        }
        Token::MEGA(pokemon, megastone, mv) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::MEGA(new_pokemon, megastone, mv)
        }
        Token::SUPEREFFECTIVE(pokemon) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::SUPEREFFECTIVE(new_pokemon)
        }
        Token::RESISTED(pokemon) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::RESISTED(new_pokemon)
        }
        Token::CRIT(pokemon) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::CRIT(new_pokemon)
        }
        Token::IMMUNE(pokemon) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::IMMUNE(new_pokemon)
        }
        Token::MISS(source, target) => {
            let new_source =
                replace_pokemon_id(&source, you_prefix, opp_prefix, your_name, opp_name);
            let new_target =
                replace_pokemon_id(&target, you_prefix, opp_prefix, your_name, opp_name);
            Token::MISS(new_source, new_target)
        }
        Token::CANT(pokemon, reason) => {
            let new_pokemon =
                replace_pokemon_id(&pokemon, you_prefix, opp_prefix, your_name, opp_name);
            Token::CANT(new_pokemon, reason)
        }
        Token::SIDESTART(side, condition) => {
            let new_side = replace_slot(&side, you_prefix, opp_prefix, your_name, opp_name);
            Token::SIDESTART(new_side, condition)
        }
        Token::SIDEEND(side, condition) => {
            let new_side = replace_slot(&side, you_prefix, opp_prefix, your_name, opp_name);
            Token::SIDEEND(new_side, condition)
        }
        _ => token,
    }
}

/// Helper to replace pokemon ID (e.g., "p1a: Pikachu" or just "p1a")
fn replace_pokemon_id(
    pokemon_id: &str,
    you_prefix: &str,
    opp_prefix: &str,
    your_name: &str,
    opp_name: &str,
) -> String {
    // If it contains a colon, replace the slot part before it
    if let Some(colon_idx) = pokemon_id.find(':') {
        let slot_part = &pokemon_id[..colon_idx];
        let rest = &pokemon_id[colon_idx..];
        let new_slot = replace_slot(slot_part, you_prefix, opp_prefix, your_name, opp_name);
        format!("{}{}", new_slot, rest)
    } else {
        // Just a slot identifier
        replace_slot(pokemon_id, you_prefix, opp_prefix, your_name, opp_name)
    }
}

/// Helper to replace player slot with [Assist]/[Against] labels
fn replace_slot(
    slot: &str,
    you_prefix: &str,
    opp_prefix: &str,
    your_name: &str,
    opp_name: &str,
) -> String {
    if slot.starts_with(you_prefix) {
        format!("[Assist: {}]", your_name)
    } else if slot.starts_with(opp_prefix) {
        format!("[Against: {}]", opp_name)
    } else {
        slot.to_string()
    }
}

pub fn parse_player(line: &str) -> Option<Token> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 4 {
        return None;
    }

    if parts[1] == "player" {
        let slot = parts[2].to_string(); // "p1" or "p2"
        let username = parts[3].trim().to_string(); // Trim whitespace from username
        Some(Token::PLAYER(slot, username))
    } else {
        None
    }
}

/// Excepted return format: (player_slot, [Pokémon_names])
pub fn parse_team_setup_by_player_slot(line: &str) -> Option<Token> {
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

        Some(Token::TEAM(player_id, vec![pokemon_name]))
    } else {
        None
    }
}

pub fn parse_title_and_gen(line: &str) -> Option<Token> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 2 {
        return None;
    }

    match parts[1] {
        "title" => {
            if parts.len() >= 3 {
                Some(Token::TITLE(parts[2].to_string()))
            } else {
                None
            }
        }

        "gen" => {
            if parts.len() >= 3 {
                Some(Token::GEN(parts[2].to_string()))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn parse_start(line: &str) -> Option<Token> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 2 {
        return None;
    }

    if parts[1] == "start" {
        Some(Token::START(true))
    } else {
        None
    }
}

pub fn parse_is_team_previewing(line: &str) -> Option<Token> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 2 {
        return None;
    }

    if parts[1] == "teampreview" {
        Some(Token::PREVIEW(true))
    } else {
        None
    }
}

pub fn parse_battle_log(line: &str, user_slot: &Option<String>, team: &[Team; 2]) -> Option<Token> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 2 {
        return None;
    }

    let token = match parts[1] {
        "turn" => {
            if parts.len() >= 3 {
                parts[2].parse::<usize>().ok().map(Token::TURN)
            } else {
                None
            }
        }

        "switch" | "drag" => {
            if parts.len() >= 4 {
                let pokemon_id = parts[2]; // e.g., "p1a: Aurorus" or "p1a: BigFist"
                let details = parts[3]; // e.g., "Aurorus, L86, F"
                let hp = parts.get(4).unwrap_or(&"100/100").to_string();

                // Extract just the Pokémon name (species)
                let pokemon_name = details
                    .split(',')
                    .next()
                    .unwrap_or(details)
                    .trim()
                    .to_string();

                let slot = pokemon_id
                    .split(':')
                    .next()
                    .unwrap_or(pokemon_id)
                    .to_string();
                Some(Token::SWITCH(slot, pokemon_name, hp))
            } else {
                None
            }
        }

        "move" if parts.len() >= 4 => {
            let pokemon_id = parts[2];
            let slot = pokemon_id
                .split(':')
                .next()
                .unwrap_or(pokemon_id)
                .trim()
                .to_string();
            let pokemon = if let Some(idx) = pokemon_id.find(':') {
                pokemon_id[idx + 2..].trim().to_string()
            } else {
                pokemon_id.trim().to_string()
            };
            let move_name = parts[3].to_string();
            let target = parts
                .get(4)
                .filter(|t| !t.is_empty())
                .map(|t| t.to_string());

            Some(Token::MOVE(slot, pokemon, move_name, target))
        }

        "-damage" if parts.len() >= 4 => {
            let pokemon_id = parts[2];
            let slot = pokemon_id
                .split(':')
                .next()
                .unwrap_or(pokemon_id)
                .trim()
                .to_string();
            let pokemon = if let Some(idx) = pokemon_id.find(':') {
                pokemon_id[idx + 2..].trim().to_string()
            } else {
                pokemon_id.trim().to_string()
            };
            let hp_status = parts[3];
            let (hp, _status) = if let Some(space_idx) = hp_status.find(' ') {
                (
                    &hp_status[..space_idx],
                    Some(hp_status[space_idx + 1..].to_string()),
                )
            } else {
                (hp_status, None)
            };
            let cause = parts.get(4).map(|s| s.trim().to_string());

            Some(Token::DAMAGE(slot, pokemon, hp.to_string(), cause))
        }

        "-heal" if parts.len() >= 4 => {
            let pokemon = parts[2].to_string();
            let hp_status = parts[3];
            let (hp, _) = if let Some(space_idx) = hp_status.find(' ') {
                (
                    &hp_status[..space_idx],
                    Some(hp_status[space_idx + 1..].to_string()),
                )
            } else {
                (hp_status, None)
            };
            let source = parts.get(4).map(|s| s.trim().to_string());
            Some(Token::HEAL(pokemon, hp.to_string(), source))
        }

        "faint" if parts.len() >= 3 => Some(Token::FAINT(parts[2].to_string())),
        "-status" if parts.len() >= 4 => {
            Some(Token::STATUS(parts[2].to_string(), parts[3].to_string()))
        }

        "-curestatus" if parts.len() >= 4 => Some(Token::CURESTATUS(
            parts[2].to_string(),
            parts[3].to_string(),
        )),
        "-boost" if parts.len() >= 4 => {
            let amount = parts.get(4).unwrap_or(&"1").to_string();
            Some(Token::BOOST(
                parts[2].to_string(),
                parts[3].to_string(),
                amount,
            ))
        }

        "-unboost" if parts.len() >= 4 => {
            let amount = parts.get(4).unwrap_or(&"1").to_string();
            Some(Token::UNBOOST(
                parts[2].to_string(),
                parts[3].to_string(),
                amount,
            ))
        }

        "-crit" if parts.len() >= 3 => Some(Token::CRIT(parts[2].to_string())),
        "-supereffective" if parts.len() >= 3 => Some(Token::SUPEREFFECTIVE(parts[2].to_string())),
        "-resisted" if parts.len() >= 3 => Some(Token::RESISTED(parts[2].to_string())),
        "-miss" if parts.len() >= 3 => {
            let source = parts[2].to_string(); // e.g., "p1a: Gengar"
            let target = parts.get(3).map(|s| s.to_string()).unwrap_or_default(); // e.g., "p2a: Dragonite"

            Some(Token::MISS(source, target))
        }

        "-immune" if parts.len() >= 3 => Some(Token::IMMUNE(parts[2].to_string())),
        "cant" if parts.len() >= 4 => Some(Token::CANT(parts[2].to_string(), parts[3].to_string())),
        "-ability" if parts.len() >= 4 => {
            Some(Token::ABILITY(parts[2].to_string(), parts[3].to_string()))
        }

        "detailschange" if parts.len() >= 3 => {
            let pokemon = parts[2].to_string();
            let mega_stone = parts[3].to_string();
            Some(Token::MEGA(pokemon, mega_stone, None))
        }

        "-weather" if parts.len() >= 3 => Some(Token::WEATHER(parts[2].to_string())),
        "win" if parts.len() >= 3 => Some(Token::WIN(parts[2].to_string())),
        "tie" => Some(Token::TIE),
        "-message" if parts.len() >= 3 => Some(Token::MESSAGE(parts[2].to_string())),

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
    token.map(|t| replace_player_ids_in_token(t, user_slot, team))
}

use std::fmt;
use std::process::exit;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::TITLE(title) => write!(f, "Battle Title: {}", title),
            Token::GEN(gn) => write!(f, "Generation: {}", gn),
            Token::PLAYER(slot, username) => write!(f, "Player {}: {}", slot, username),
            Token::TEAM(slot, pokemon) => write!(f, "Team {}: {:?}", slot, pokemon),
            Token::PREVIEW(true) => write!(f, "Team Preview Started"),
            Token::START(true) => write!(f, "Battle Started"),
            Token::TURN(num) => write!(f, " TURN {} ", num),
            Token::MOVE(slot, pokemon, move_name, target) => {
                if let Some(tgt) = target {
                    write!(f, "{}: {} used {} on {}", slot, pokemon, move_name, tgt)
                } else {
                    write!(f, "{}: {} used {}", slot, pokemon, move_name)
                }
            }
            Token::SWITCH(slot, species, hp) => {
                write!(f, "{}: {} sent out ({}) HP: {}", slot, species, species, hp)
            }
            Token::DAMAGE(slot, pokemon, hp, cause) => {
                if let Some(c) = cause {
                    write!(f, "{}: {} HP: {} ({})", slot, pokemon, hp, c)
                } else {
                    write!(f, "{}: {} HP: {}", slot, pokemon, hp)
                }
            }
            Token::FAINT(pokemon) => write!(f, "{} fainted!", pokemon),
            Token::STATUS(pokemon, status) => {
                write!(f, "{} was inflicted with {}", pokemon, status)
            }
            Token::CURESTATUS(pokemon, status) => write!(f, "{} cured of {}", pokemon, status),
            Token::BOOST(pokemon, stat, amount) => {
                write!(f, "{}'s {} rose by {}", pokemon, stat, amount)
            }
            Token::UNBOOST(pokemon, stat, amount) => {
                write!(f, "{}'s {} fell by {}", pokemon, stat, amount)
            }
            Token::HEAL(pokemon, hp, source) => {
                if let Some(src) = source {
                    write!(f, "{} healed to {} ({})", pokemon, hp, src)
                } else {
                    write!(f, "{} healed to {}", pokemon, hp)
                }
            }
            Token::WEATHER(weather) => {
                if weather == "none" {
                    write!(f, "Weather cleared")
                } else {
                    write!(f, "Weather: {}", weather)
                }
            }
            Token::SIDESTART(side, condition) => write!(f, "{} set up {}", side, condition),
            Token::SIDEEND(side, condition) => write!(f, "{}'s {} wore off", side, condition),
            Token::ABILITY(pokemon, ability) => write!(f, "{}'s ability: {}", pokemon, ability),
            Token::MEGA(pokemon, form, move_name) => {
                if let Some(mv) = move_name {
                    write!(f, "{} Mega Evolved into {} using {}", pokemon, form, mv)
                } else {
                    write!(f, "{} Mega Evolved into {}", pokemon, form)
                }
            }
            Token::SUPEREFFECTIVE(pokemon) => write!(f, "Super effective on {}!", pokemon),
            Token::RESISTED(pokemon) => write!(f, "{} resisted the attack", pokemon),
            Token::MISS(source, target) => {
                if !target.is_empty() {
                    write!(f, "{} missed {}!", source, target)
                } else {
                    write!(f, "{}'s attack missed!", source)
                }
            }

            Token::CRIT(pokemon) => write!(f, "Critical hit on {}!", pokemon),
            Token::IMMUNE(pokemon) => write!(f, "{} is immune!", pokemon),
            Token::CANT(pokemon, reason) => write!(f, "{} can't move ({})", pokemon, reason),
            Token::WIN(player) => write!(f, "{} wins the battle!", player),
            Token::TIE => write!(f, "Battle ended in a tie"),
            Token::MESSAGE(msg) => write!(f, "{}", msg),
            _ => write!(f, ""), // Handle other variants
        }
    }
}
