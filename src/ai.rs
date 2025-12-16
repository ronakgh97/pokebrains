use crate::logs::BattleEvents;
use anyhow::Result;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::openai;
use rig::providers::openai::responses_api::ResponsesCompletionModel;

pub enum ModelType {
    Local,
    Cloud,
}

pub struct BattleAgent {
    model: String,
    model_type: ModelType,
    #[allow(unused)]
    agent: Option<Agent<ResponsesCompletionModel>>,
    history: Vec<String>,
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
                    .preamble(
                        "You are an expert Pokémon Showdown competitive battle analyst. \
                        Your job is to recommend the optimal move or switch based on the current battle state. \
                        Consider: type matchups, stat changes, HP percentages, hazards, abilities, \
                        momentum, and win conditions. Provide clear, concise and short reasoning for your recommendations, \
                        You will assist the player label as [Assist] and play against player label as [Against], \
                        Your responses should be very brief and to the point, and should include only two parts: Suggestion and Reasoning.",
                    )
                    .temperature(0.7)
                    .build()
            }
            ModelType::Local => {
                let client: openai::Client = openai::Client::builder()
                    .base_url("http://localhost:1234/v1")
                    .api_key(api_key)
                    .build()?;

                client
                    .agent(&self.model)
                    .preamble(
                        "You are an expert Pokémon Showdown competitive battle analyst. \
                        Your job is to recommend the optimal move or switch based on the current battle state. \
                        Consider: type matchups, stat changes, HP percentages, hazards, abilities, \
                        momentum, and win conditions. Provide clear reasoning for your recommendations, \
                        You will assist the player label as [Assist] and play against player label as [Against].\
                        Your responses should be very brief and to the point, and should include only two parts: Suggestion and Reasoning.",
                    )
                    .temperature(0.7)
                    .build()
            }
        };

        Ok(Self {
            agent: Some(agent),
            ..self
        })
    }
    pub async fn get_initial_suggestions(&mut self, events: BattleEvents) -> String {
        let mut prompt = events.init;
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

        let question =
            "Based on the initial teams, what strategy should I consider for this battle?";
        prompt.push_str(question);
        prompt.push('\n');

        //DEBUG
        println!("Initial Prompt Sent to Agent:\n{}", prompt);

        if let Some(agent) = &self.agent {
            match agent.prompt(&prompt.clone()).await {
                Ok(response) => {
                    self.history.push(prompt.clone());
                    self.history.push(response.clone());
                    response.clone()
                }
                Err(e) => format!("Error generating suggestions: {}", e),
            }
        } else {
            "Agent not initialized.".to_string()
        }
    }

    pub async fn get_turn_suggestions(&mut self, events: BattleEvents) -> String {
        let mut prompt = String::new();

        // Add the history of previous turns
        prompt.push_str(&self.history.join("\n"));

        // Add turns details
        for turn in &events.events {
            prompt.push_str(&turn.join("\n"));
            prompt.push('\n');
        }

        let question = "Based on the current battle state, what is the optimal move or switch?";
        prompt.push_str(question);
        prompt.push('\n');

        //DEBUG
        println!("Initial Prompt Sent to Agent:\n{}", prompt);

        if let Some(agent) = &self.agent {
            match agent.prompt(&prompt.clone()).await {
                Ok(response) => {
                    self.history.push(prompt.clone());
                    self.history.push(response.clone());
                    response.clone()
                }
                Err(e) => format!("Error generating suggestions: {}", e),
            }
        } else {
            "Agent not initialized.".to_string()
        }
    }
}
