use pokebrains::{BattleAgent, BattleClient, Colorize, ModelType, Result};
use std::io::Write;
use std::io::{stdin, stdout};

#[tokio::main]
async fn main() -> Result<()> {
    let asscii_art = r"
    
   ▄███████▄  ▄██████▄     ▄█   ▄█▄    ▄████████ ▀█████████▄     ▄████████    ▄████████  ▄█  ███▄▄▄▄      ▄████████ 
  ███    ███ ███    ███   ███ ▄███▀   ███    ███   ███    ███   ███    ███   ███    ███ ███  ███▀▀▀██▄   ███    ███ 
  ███    ███ ███    ███   ███▐██▀     ███    █▀    ███    ███   ███    ███   ███    ███ ███▌ ███   ███   ███    █▀  
  ███    ███ ███    ███  ▄█████▀     ▄███▄▄▄      ▄███▄▄▄██▀   ▄███▄▄▄▄██▀   ███    ███ ███▌ ███   ███   ███        
▀█████████▀  ███    ███ ▀▀█████▄    ▀▀███▀▀▀     ▀▀███▀▀▀██▄  ▀▀███▀▀▀▀▀   ▀███████████ ███▌ ███   ███ ▀███████████ 
  ███        ███    ███   ███▐██▄     ███    █▄    ███    ██▄ ▀███████████   ███    ███ ███  ███   ███          ███ 
  ███        ███    ███   ███ ▀███▄   ███    ███   ███    ███   ███    ███   ███    ███ ███  ███   ███    ▄█    ███ 
 ▄████▀       ▀██████▀    ███   ▀█▀   ██████████ ▄█████████▀    ███    ███   ███    █▀  █▀    ▀█   █▀   ▄████████▀  
                          ▀                                     ███    ███
    ";

    println!("{}", asscii_art.bright_green());

    println!();
    println!();

    println!("{}", "Enter room ID: ".yellow());
    print!("{} ", ">>>".yellow());
    stdout().flush()?;
    let mut room_id = String::new();
    stdin()
        .read_line(&mut room_id)
        .expect("Failed to read input");
    println!("{}", "Enter your username: ".yellow());
    print!("{} ", ">>>".yellow());
    stdout().flush()?;
    let mut player = String::new();
    stdin()
        .read_line(&mut player)
        .expect("Failed to read input");

    let mut agent = None;

    let battle_agent = BattleAgent::new("qwen/qwen3-vl-8b", ModelType::Local);
    match battle_agent.build_agent("local") {
        Ok(a) => {
            println!("{}", "AI Agent initialized successfully!".green());
            agent = Some(a);
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to initialize AI Agent: {}", e).red());
        }
    }

    let room_id = room_id.trim();
    let room_id = if room_id.contains("play.pokemonshowdown.com/") {
        room_id.split('/').next_back().unwrap_or(room_id)
    } else {
        room_id
    };
    if room_id.is_empty() {
        println!(
            "{}",
            "Room ID is null or invalid, connecting to lobby".yellow()
        );
        let mut battle_room = BattleClient::new("lobby", player, 30);
        battle_room.ai_agent = agent;
        if let Err(e) = battle_room.connect().await {
            eprintln!("{}", format!("Connection error: {}", e).red());
        }
    } else {
        println!("{}", format!("Connecting to room: {}", room_id).green());
        let mut battle_room = BattleClient::new(room_id, player, 30);
        battle_room.ai_agent = agent;
        if let Err(e) = battle_room.connect().await {
            eprintln!("{}", format!("Connection error: {}", e).red());
        }
    }

    Ok(())
}
