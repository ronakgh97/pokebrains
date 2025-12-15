use anyhow::Result;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::providers::openai;
use rig::providers::openai::Message;
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
    #[allow(unused)]
    memory: Vec<Message>,
}

impl BattleAgent {
    pub fn _new(model: &str, model_type: ModelType) -> Self {
        BattleAgent {
            model: model.to_string(),
            model_type,
            agent: None,
            memory: Vec::new(),
        }
    }

    pub fn _build_agent(self, api_key: &str) -> Result<Self> {
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
                        momentum, and win conditions. Provide clear reasoning for your recommendations, \
                        You will assist the player label as [Assist] and play against player label as [Against].",
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
                        You will assist the player label as [Assist] and play against player label as [Against].",
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
}
