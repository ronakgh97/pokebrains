use crate::logs::BattleEvents;
use crate::{Colorize, Result};
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub struct BattleClient {
    room_id: String,
    url: String,
    connection_timeout: u64,
    is_connected: bool,
    pub event_logs: BattleEvents,
}

impl BattleClient {
    pub fn new(room_id: &str, user: String, connection_timeout: u64) -> Self {
        BattleClient {
            room_id: room_id.to_string(),
            url: "wss://sim3.psim.us/showdown/websocket".to_string(),
            connection_timeout,
            is_connected: false,
            event_logs: BattleEvents::new(user),
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let connect_future = connect_async(&self.url);
        let (ws_stream, _response) =
            timeout(Duration::from_secs(self.connection_timeout), connect_future).await??;

        self.is_connected = true;
        println!("{}", "Connection established!".green());

        let (mut write, mut read) = ws_stream.split();

        // Immediately join the room
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

    pub async fn join_room(
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

    pub async fn handle_message(&mut self, text: &str) -> Result<()> {
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
                    //self.parse_battle_log(line);
                    self.event_logs.add_event(line);
                    println!();
                    println!("Event count: {}", &self.event_logs.events.len());
                    println!("Event init: {}", &self.event_logs.init);
                    for (i, event) in self.event_logs.events.clone().into_iter().enumerate() {
                        println!("Index: {}\n Event: {:?}", i, event);
                    }
                } else {
                    println!("{}", format!("[RAW] {}", line).dimmed());
                }
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn parse_battle_log(&self, line: &str) {
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
            "upkeep" => println!("{}", format!("[BATTLE CONTINUES] {}", content).yellow()),
            "-damage" => println!("{}", format!("[DAMAGE] {}", content).red()),
            "-suppereffective" => println!("{}", format!("[SUPEREFFECTIVE] {}", content).yellow()),
            "-heal" => println!("{}", format!("[HEAL] {}", content).green()),
            "-status" => println!("{}", format!("[STATUS] {}", content).yellow()),
            "-curestatus" => println!("{}", format!("[CURE] {}", content).green()),
            "-ability" => println!("{}", format!("[ABILITY] {}", content).cyan()),
            "-boost" => println!("{}", format!("[BUFF] {}", content).cyan()),
            "-unboost" => println!("{}", format!("[NERF] {}", content).red()),
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
