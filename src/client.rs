use crate::agent::BattleAgent;
use crate::parser::logs::BattleEvents;
use crate::{Colorize, Result, Team};
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

pub struct ShowdownClient {
    room_id: String,
    url: String,
    connection_timeout: u64,
    is_connected: bool,
    pub event_logs: BattleEvents,
    pub ai_agent: Option<BattleAgent>,
    last_turn: usize, // Track the last processed turn
}

impl ShowdownClient {
    pub fn new(room_id: &str, user: String, connection_timeout: u64) -> Self {
        ShowdownClient {
            room_id: room_id.to_string(),
            url: "wss://sim3.psim.us/showdown/websocket".to_string(),
            connection_timeout,
            is_connected: false,
            event_logs: BattleEvents::new(user),
            ai_agent: None,
            last_turn: 0, // Initialize last_turn
        }
    }

    pub async fn connect_to_room(&mut self) -> Result<()> {
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

    pub async fn validate_team(&mut self, team_data: &str, format: &str) -> Result<String> {
        let connect_future = connect_async(&self.url);
        let (ws_stream, _response) =
            timeout(Duration::from_secs(self.connection_timeout), connect_future).await??;

        self.is_connected = true;
        println!("{}", "Connection established!".green());

        let (mut write, mut read) = ws_stream.split();

        let packed_team = Team::deserialize(&team_data).await.serialize_packed();
        let utm_cmd = format!("|/utm {}", packed_team);
        println!("{}", format!("[SENDING] {}", utm_cmd).dimmed());
        write.send(Message::Text(Utf8Bytes::from(utm_cmd))).await?;

        tokio::time::sleep(Duration::from_millis(100)).await;

        let validate_cmd = format!("|/vtm {}", format);
        println!("{}", format!("[SENDING] {}", validate_cmd).dimmed());
        write
            .send(Message::Text(Utf8Bytes::from(validate_cmd)))
            .await?;

        let timeout_duration = Duration::from_secs(5);
        let start = tokio::time::Instant::now();

        while start.elapsed() < timeout_duration {
            if let Ok(Some(Ok(Message::Text(text)))) =
                timeout(Duration::from_millis(500), read.next()).await
            {
                if text.contains("|popup|") {
                    let content = text.split("|popup|").nth(1).unwrap_or("").trim();

                    // Close connection
                    let _ = write.send(Message::Close(None)).await;

                    if content.to_lowercase().contains("valid") {
                        println!("{}", format!("[VALID] {}", content).green().bold());
                        return Ok("Team is valid".to_string());
                    } else {
                        println!("{}", format!("[INVALID] {}", content).red().bold());
                        return Err(anyhow::anyhow!("Validation failed: {}", content));
                    }
                }
            }
        }

        // Timeout - close connection
        let _ = write.send(Message::Close(None)).await;
        Err(anyhow::anyhow!("Validation timeout"))
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
                    //self.parse_log(line);
                    self.event_logs.add_event(line);

                    // AI Integration
                    if let Some(agent) = &mut self.ai_agent {
                        // Detect transition: battle just started
                        //TODO: Holy!! Refactor this mess
                        if self.event_logs.is_previewing_team
                            && !self.event_logs.battle_started
                            && !self.event_logs.is_init_suggestions_generated
                            && !self.event_logs.team[0].player.is_empty()
                            && !self.event_logs.team[1].player.is_empty()
                            && self.event_logs.team[0].pokemon.len() == 6
                            && self.event_logs.team[1].pokemon.len() == 6
                        {
                            println!();
                            let _suggestion = agent
                                .get_initial_suggestions_stream(self.event_logs.clone())
                                .await?;
                            self.event_logs.is_init_suggestions_generated = true;
                            // match suggestion {
                            //     Ok(stream) => {
                            //         log_typewriter_effect(150, stream).await?;
                            //     }
                            //     Err(e) => {
                            //         return Err(anyhow::anyhow!("Error in stream: {}", e));
                            //     }
                            // }
                        }
                        // Battle is ongoing and we have new turn data
                        let current_turn = self.event_logs.get_current_turn();

                        // Debug logging
                        if line.contains("|turn|") {
                            //println!(
                            //    "[DEBUG] Turn event detected! Current turn: {}, Last turn: {}, Battle started: {}",
                            //    current_turn, self.last_turn, self.event_logs.battle_started
                            //);
                        }

                        if self.event_logs.battle_started && current_turn > self.last_turn {
                            let _suggestion = agent
                                .get_turn_suggestions_stream(self.event_logs.clone())
                                .await?;
                            // match suggestion {
                            //     Ok(stream) => {
                            //         log_typewriter_effect(120, stream).await?;
                            //     }
                            //     Err(e) => {
                            //         return Err(anyhow::anyhow!("Error in stream: {}", e));
                            //     }
                            // }
                            self.last_turn = current_turn;
                        }
                    }
                } else {
                    println!("{}", format!("[RAW] {}", line).dimmed());
                }
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn debug_turn_print(&self, line: &str) {
        if line.contains("|turn|") {
            let parts: Vec<&str> = line.splitn(3, '|').collect();
            if parts.len() >= 3 {
                let turn_number = parts[2];
                println!(
                    "{}",
                    format!("[DEBUG] Turn detected: {}", turn_number)
                        .yellow()
                        .bold()
                );
            }
        }
    }

    #[allow(dead_code)]
    fn parse_log(&self, line: &str) {
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

#[tokio::test]
async fn test_team_validate() -> anyhow::Result<()> {
    static TEAM: &str = "#\
Dragonite @ Choice Scarf
Ability: Inner Focus
EVs: 100 HP / 64 Atk / 52 Def / 132 SpA / 84 SpD / 76 Spe
- Blizzard
- Draco Meteor
- Body Slam
- Earthquake

Zoroark @ Assault Vest
Ability: Illusion
EVs: 60 HP / 36 Atk / 116 Def / 84 SpA / 68 SpD / 120 Spe
Lonely Nature
- Calm Mind
- Foul Play
- Shadow Claw
- Dark Pulse

Chansey (F) @ Lucky Punch
Ability: Natural Cure
EVs: 208 HP / 156 Def / 144 SpD
- Return
- Blizzard
- Aromatherapy
- Facade

Azumarill @ Life Orb
Ability: Thick Fat
EVs: 80 HP / 100 Atk / 84 Def / 60 SpA / 68 SpD / 116 Spe
- Aqua Tail
- Play Rough
- Ice Punch
- Body Slam

Charizard-Mega-X @ Charizardite X
Ability: Tough Claws
EVs: 124 HP / 80 Atk / 64 Def / 72 SpA / 72 SpD / 96 Spe
- Fire Blast
- Fire Punch
- Crunch
- Aerial Ace

Gengar @ Rocky Helmet
Ability: Levitate
EVs: 104 HP / 36 Atk / 100 Def / 116 SpA / 72 SpD / 80 Spe
- Dark Pulse
- Destiny Bond
- Drain Punch
- Hex
";

    let mut client = ShowdownClient::new("testroom", "TestUser".to_string(), 10);
    client.validate_team(TEAM, "gen5ou").await?;
    Ok(())
}
