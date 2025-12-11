use pokebrains::{BattleClient, Colorize, Result};
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

    print!("{}", "Enter room ID: ".yellow());
    stdout().flush().unwrap();
    let mut room_id = String::new();
    stdin()
        .read_line(&mut room_id)
        .expect("Failed to read input");
    let room_id = room_id.trim();
    let room_id = if room_id.contains("play.pokemonshowdown.com/") {
        room_id.split('/').next_back().unwrap_or(room_id)
    } else {
        room_id
    };
    if room_id.is_empty() || !room_id.contains("play.pokemonshowdown.com/") {
        println!(
            "{}",
            "Room ID is null or invalid, connecting to lobby".yellow()
        );
        let mut battle_room = BattleClient::new("lobby", 30);
        if let Err(e) = battle_room.connect().await {
            eprintln!("{}", format!("Connection error: {}", e).red());
        }
    } else {
        println!("{}", format!("Connecting to room: {}", room_id).green());
        let mut battle_room = BattleClient::new(room_id, 30);
        if let Err(e) = battle_room.connect().await {
            eprintln!("{}", format!("Connection error: {}", e).red());
        }
    }

    Ok(())
}
