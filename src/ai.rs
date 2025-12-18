use crate::parser::logs::BattleEvents;
use anyhow::Result;
#[allow(unused)]
use colored::Colorize;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::completion::{Chat, Message};
use rig::providers::openai;
use rig::providers::openai::responses_api::ResponsesCompletionModel;

static SYSTEM_PROMPT: &str = "\
You are a Pokemon Showdown battle bot.\n\
\n\
RULES:\n\
- You assist the player labeled [Assist]\n\
- You play against the player labeled [Against]\n\
- Give ONE concrete action only\n\
- Keep reasoning under 2 sentences\n\
- No speculation or uncertainty\n\
\n\
OUTPUT FORMAT:\n\
Action: [specific move/switch]\n\
Reason: [why in 1-2 sentences]";

pub enum ModelType {
    Local,
    Cloud,
}

pub struct BattleAgent {
    model: String,
    model_type: ModelType,
    agent: Option<Agent<ResponsesCompletionModel>>,
    history: Vec<Message>,
}

impl BattleAgent {
    pub fn new(model: &str, model_type: ModelType) -> Self {
        BattleAgent {
            model: model.to_string(),
            model_type,
            agent: None,
            history: Vec::new(),
        }
    }

    pub fn build_agent(self, api_key: &str) -> Result<Self> {
        let agent = match self.model_type {
            ModelType::Cloud => {
                let client: openai::Client = openai::Client::builder()
                    .base_url("https://openrouter.ai/api/v1")
                    .api_key(api_key)
                    .build()?;

                client
                    .agent(&self.model)
                    .preamble(SYSTEM_PROMPT)
                    .temperature(0.3)
                    .build()
            }
            ModelType::Local => {
                let client: openai::Client = openai::Client::builder()
                    .base_url("http://localhost:1234/v1")
                    .api_key(api_key)
                    .build()?;

                client
                    .agent(&self.model)
                    .preamble(SYSTEM_PROMPT)
                    .temperature(0.3)
                    .build()
            }
        };

        Ok(Self {
            agent: Some(agent),
            ..self
        })
    }
    pub async fn get_initial_suggestions(&mut self, events: BattleEvents) -> String {
        let mut prompt = events
            .init
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        prompt.push('\n');

        let team_match_up = format!(
            "Player 1: {:?}, Team: {:?}\nPlayer 2: {:?}, Team: {:?}\n",
            events.team[0].player,
            events.team[0].pokemon,
            events.team[1].player,
            events.team[1].pokemon,
        );
        prompt.push_str(&team_match_up);
        prompt.push('\n');

        let question = "Which Pokemon should lead with and why?";
        prompt.push_str(question);
        prompt.push('\n');

        self.call_ai_api(prompt).await
    }

    pub async fn get_turn_suggestions(&mut self, events: BattleEvents) -> String {
        let mut prompt = String::new();

        // Add turns details
        if let Some(last_turn) = events.events.last() {
            let turn_text = last_turn
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            prompt.push_str(&turn_text);
        }

        prompt.push('\n');
        let question = "Based on the current battle state, what is the optimal move or switch?";
        prompt.push_str(question);
        prompt.push('\n');

        self.call_ai_api(prompt).await
    }

    async fn call_ai_api(&mut self, prompt: String) -> String {
        //DEBUG
        println!();
        println!("[DEBUG] Prompt Sent to Agent:\n{}", prompt.dimmed());

        if let Some(agent) = &self.agent {
            match agent.chat(&prompt.clone(), self.history.clone()).await {
                Ok(response) => {
                    self.history.push(Message::user(prompt.clone()));
                    self.history.push(Message::assistant(response.clone()));
                    response.clone()
                }
                Err(e) => format!("Error generating suggestions: {}", e),
            }
        } else {
            "Agent not initialized.".to_string()
        }
    }
}
