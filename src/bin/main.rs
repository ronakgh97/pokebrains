use pokebrains::tools::PokeAPITool;
use pokebrains::tools_registry::ToolRegistry;
use pokebrains::{BattleAgent, Colorize, ModelType, Result, ShowdownClient};
use std::io::Write;
use std::io::{stdin, stdout};

#[tokio::main]
async fn main() -> Result<()> {
    let text_art = r"
    
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

    println!("{}", text_art.bright_green());

    println!();
    println!();

    println!("{}", "Enter room ID: ".yellow());
    print!("{} ", "↪".yellow());
    stdout().flush()?;
    let mut room_id = String::new();
    stdin()
        .read_line(&mut room_id)
        .expect("Failed to read input");
    println!("{}", "Enter your username: ".yellow());
    print!("{} ", "↪".yellow());
    stdout().flush()?;
    let mut player = String::new();
    stdin()
        .read_line(&mut player)
        .expect("Failed to read input");

    let mut agent = None;

    let battle_agent = BattleAgent::new("qwen/qwen3-8b", ModelType::Local);

    let mut tool_registry: ToolRegistry = ToolRegistry::new();
    tool_registry.register(PokeAPITool);

    match battle_agent.build_agent("local", tool_registry) {
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
        let mut battle_room = ShowdownClient::new("lobby", player, 30);
        battle_room.ai_agent = agent;
        if let Err(e) = battle_room.connect_to_room().await {
            eprintln!("{}", format!("Connection error: {}", e).red());
        }
    } else {
        println!("{}", format!("Connecting to room: {}", room_id).green());
        let mut battle_room = ShowdownClient::new(room_id, player, 30);
        battle_room.ai_agent = agent;
        if let Err(e) = battle_room.connect_to_room().await {
            eprintln!("{}", format!("Connection error: {}", e).red());
        }
    }

    Ok(())
}
