pub struct BattleEvents {
    pub events: Vec<String>,
}

impl BattleEvents {
    pub fn new() -> Self {
        BattleEvents { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: &str) {
        if let Some(parsed) = parse_battle_log(event) {
            self.events.push(parsed);
        }
    }
}

pub fn parse_battle_log(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 2 {
        return None;
    }

    match parts[1] {
        "move" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let move_name = parts[3];
                Some(format!("{} used {}!", pokemon, move_name))
            } else {
                None
            }
        }
        "switch" => {
            if parts.len() >= 4 {
                let pokemon_out = parts[2];
                let pokemon_in = parts[3];
                Some(format!("{} switched out for {}!", pokemon_out, pokemon_in))
            } else {
                None
            }
        }
        "damage" => {
            if parts.len() >= 4 {
                let pokemon = parts[2];
                let damage_info = parts[3];
                Some(format!("{} took damage: {}!", pokemon, damage_info))
            } else {
                None
            }
        }
        _ => None,
    }
}
