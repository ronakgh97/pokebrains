use anyhow::Result;
use colored::Colorize;
use futures_util::{SinkExt, StreamExt};
use std::io::Write;
use std::io::{stdin, stdout};
use tokio::time::{Duration, timeout};
use tokio_tungstenite::{connect_async, tungstenite::Message};

struct BattleClient {
    room_id: String,
    url: String,
    connection_timeout: u64,
    is_connected: bool,
}

impl BattleClient {
    fn new(room_id: &str, connection_timeout: u64) -> Self {
        BattleClient {
            room_id: room_id.to_string(),
            url: "wss://sim3.psim.us/showdown/websocket".to_string(),
            connection_timeout,
            is_connected: false,
        }
    }

    async fn connect(&mut self) -> Result<()> {
        let connect_future = connect_async(&self.url);
        let (ws_stream, _response) =
            timeout(Duration::from_secs(self.connection_timeout), connect_future).await??;

        self.is_connected = true;
        println!("{}", "Connection established!".green());

        let (mut write, mut read) = ws_stream.split();

        // Immediately join the room without waiting for challstr
        self.join_room(&mut write).await?;

        // Handle incoming messages
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    self.handle_message(&text).await?;
                }
                Ok(Message::Close(_)) => {
                    println!("{}", "Connection closed by server".yellow());
                    break;
                }
                Ok(Message::Ping(data)) => {
                    write.send(Message::Pong(data)).await?;
                }
                Err(e) => {
                    eprintln!("{}", format!("WebSocket error: {}", e).red());
                    break;
                }
                _ => {}
            }
        }

        self.is_connected = false;
        Ok(())
    }

    async fn join_room(
        &self,
        write: &mut futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            Message,
        >,
    ) -> Result<()> {
        let join_cmd = format!("|/join {}", self.room_id);
        write.send(Message::Text(join_cmd.clone().into())).await?;
        Ok(())
    }

    async fn handle_message(&self, text: &str) -> Result<()> {
        let mut current_room = String::new();
        for line in text.lines() {
            if line.is_empty() {
                continue;
            }
            // Room messages start with >roomid
            if line.starts_with('>') {
                current_room = line.trim_start_matches('>').to_string();
            } else if current_room == self.room_id {
                // Only print messages for the joined battle room
                if line.starts_with('|') {
                    self.handle_battle_message(line);
                } else {
                    println!("{}", format!("[RAW] {}", line).dimmed());
                }
            }
        }
        Ok(())
    }

    fn handle_battle_message(&self, line: &str) {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() < 2 {
            return;
        }
        let msg_type = parts.get(1).unwrap_or(&"");
        let content = parts.get(2).unwrap_or(&"");
        match *msg_type {
            "init" => println!(
                "{}",
                format!("[INIT] Room type: {}", content).green().bold()
            ),
            "title" => println!("{}", format!("[TITLE] {}", content).cyan().bold()),
            "player" => println!("{}", format!("[PLAYER] {}", content).blue()),
            "teamsize" => println!("{}", format!("[TEAMSIZE] {}", content).blue()),
            "gametype" => println!("{}", format!("[GAMETYPE] {}", content).blue()),
            "gen" => println!("{}", format!("[GEN] Generation {}", content).blue()),
            "tier" => println!("{}", format!("[TIER] {}", content).cyan()),
            "rule" => println!("{}", format!("[RULE] {}", content).dimmed()),
            "switch" | "drag" => println!("{}", format!("[SWITCH] {}", content).yellow()),
            "move" => println!("{}", format!("[MOVE] {}", content).magenta()),
            "turn" => println!("{}", format!(" TURN: {} ", content).white().bold()),
            "upkeep" => println!("{}", format!("[BATTLE COUNTINUES] {}", content).yellow()),
            "-damage" => println!("{}", format!("[DAMAGE] {}", content).red()),
            "-suppereffective" => println!("{}", format!("[SUPEREFFECTIVE] {}", content).yellow()),
            "-heal" => println!("{}", format!("[HEAL] {}", content).green()),
            "-status" => println!("{}", format!("[STATUS] {}", content).yellow()),
            "-curestatus" => println!("{}", format!("[CURE] {}", content).green()),
            "-ability" => println!("{}", format!("[ABILITY] {}", content).cyan()),
            "-boost" => println!("{}", format!("[BOOST] {}", content).cyan()),
            "-unboost" => println!("{}", format!("[UNBOOST] {}", content).red()),
            "-weather" => println!("{}", format!("[WEATHER] {}", content).blue()),
            "-fieldstart" | "-fieldend" => println!("{}", format!("[FIELD] {}", content).blue()),
            "-sidestart" | "-sideend" => println!("{}", format!("[SIDE] {}", content).blue()),
            "faint" => println!("{}", format!("[FAINT] {}", content).red().bold()),
            "win" => println!("{}", format!("   WINNER: {}  ", content).green().bold()),
            "tie" => println!("{}", "[TIE] The battle ended in a tie!".yellow().bold()),
            "c" | "c:" => println!("{}", format!("[CHAT] {}", content).dimmed()),
            "j" | "J" => println!("{}", format!("[JOIN] {}", content).dimmed()),
            "l" | "L" => println!("{}", format!("[LEAVE] {}", content).dimmed()),
            "raw" | "html" => println!("{}", format!("[HTML] {}", content).dimmed()),
            "request" => {}
            "" => {}
            _ => println!("{}", format!("[{}] {}", msg_type, content).dimmed()),
        }
    }
}

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
    if room_id.is_empty() {
        println!("{}", "No room ID provided, connecting to lobby...".yellow());
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
