use anyhow::Result;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::providers::openai;
use rig::providers::openai::Message;
use rig::providers::openai::responses_api::ResponsesCompletionModel;

#[allow(unused)]
pub enum ModelType {
    Local,
    Cloud,
}

#[allow(unused)]
pub struct BattleAgent {
    user: String,
    opponent: String,
    model: String,
    model_type: ModelType,
    agent: Option<Agent<ResponsesCompletionModel>>,
    memory: Vec<Message>,
}

impl BattleAgent {
    pub fn _new(user: &str, opponent: &str, model: &str, model_type: ModelType) -> Self {
        BattleAgent {
            user: user.to_string(),
            opponent: opponent.to_string(),
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
                    .base_url("https://openrouter/api/v1")
                    .api_key(api_key)
                    .build()?;

                client
                    .agent(self.model.clone())
                    .preamble("")
                    .context("")
                    .temperature(0.9)
                    .build()
            }
            ModelType::Local => {
                let client: openai::Client = openai::Client::builder()
                    .base_url("http://localhost:1234/v1")
                    .api_key(api_key)
                    .build()?;

                client
                    .agent(self.model.clone())
                    .preamble("")
                    .context("")
                    .temperature(0.8)
                    .build()
            }
        };

        Ok(Self {
            agent: Some(agent),
            ..self
        })
    }
}
