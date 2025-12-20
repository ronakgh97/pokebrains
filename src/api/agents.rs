use crate::api::dtos::Role::system;
use crate::api::dtos::{CompletionRequest, Message};
use crate::api::request::{send_request, send_request_stream};
use anyhow::Result;
use futures_util::Stream;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct Agent {
    pub model: String,
    pub url: String,
    pub api_key: String,
    pub system_prompt: String,
    pub temperature: f32,
    pub top_p: f32,
}

#[derive(Debug, Clone)]
pub struct AgentBuilder {
    model: Option<String>,
    url: String,
    api_key: String,
    system_prompt: String,
    temperature: f32,
    pub top_p: f32,
}

impl Default for AgentBuilder {
    fn default() -> Self {
        Self {
            model: None,
            url: "http://localhost:1234/v1".to_string(),
            api_key: "local".to_string(),
            system_prompt: "You are a helpful assistant.\n Strict follow user instructions"
                .to_string(),

            temperature: 0.7,
            top_p: 0.9,
        }
    }
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = prompt.to_string();
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn build(self) -> Result<Agent> {
        Ok(Agent {
            model: self
                .model
                .ok_or_else(|| anyhow::anyhow!("Model is required"))?,
            url: self.url,
            api_key: self.api_key,
            system_prompt: self.system_prompt,
            temperature: self.temperature,
            top_p: self.top_p,
        })
    }
}

pub async fn prompt(agent: Agent, prompt: &str, history: Vec<Message>) -> Result<String> {
    // Add system prompt to the beginning of history for non-repetitive context
    let mut history = history;
    history.insert(
        0,
        Message {
            role: system,
            content: agent.system_prompt,
        },
    );

    // // Add user prompt
    // history.push(Message {
    //     role: Role::user,
    //     content: prompt.to_string(),
    // });

    let request = CompletionRequest {
        model: agent.model,
        messages: history,
        temperature: agent.temperature,
        top_p: Some(agent.top_p),
        stream: Some(false),
    };

    // Debug: print the request payload to verify system prompt is included
    println!("Request payload: {:?}", request);

    let response = send_request(agent.url.clone(), agent.api_key.clone(), request).await?;

    Ok(response)
}

pub async fn prompt_stream(
    agent: Agent,
    prompt: &str,
    history: Vec<Message>,
) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
    // Add system prompt to the beginning of history for non-repetitive context
    let mut history = history;

    history.insert(
        0,
        Message {
            role: system,
            content: agent.system_prompt,
        },
    );

    // // Add user prompt
    // history.push(Message {
    //     role: Role::user,
    //     content: prompt.to_string(),
    // });

    let request = CompletionRequest {
        model: agent.model,
        messages: history,
        temperature: agent.temperature,
        top_p: Some(agent.top_p),
        stream: Some(true),
    };

    let stream = send_request_stream(agent.url.clone(), agent.api_key.clone(), request).await?;

    Ok(Box::pin(stream))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::dtos::Role::user;
    use crate::api::request::pretty_print_stream;
    use anyhow::Result;

    #[tokio::test]
    async fn test_agent_builder_missing_model() {
        let result = AgentBuilder::new()
            .url("http://localhost:1234/v1")
            .api_key("test_key")
            .temperature(0.5)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_agent_builder_default_values() {
        let builder = AgentBuilder::default();
        assert_eq!(builder.url, "http://localhost:1234/v1");
        assert_eq!(builder.api_key, "local");
        assert_eq!(
            builder.system_prompt,
            "You are a helpful assistant.\n Strict follow user instructions"
        );
        assert_eq!(builder.temperature, 0.7);
        assert_eq!(builder.top_p, 0.9);
        assert!(builder.model.is_none());
    }

    const SYSTEM_PROMPT: &str = "\
You are a Pokemon Showdown battle Assistant.\n\
\n\
RULES:\n\
- You assist the player labeled [Assist]\n\
- You play against the player labeled [Against]\n\
- Give ONE concrete action only\n\
- Keep reasoning under 2 sentences\n\
- No speculation or uncertainty\n\
\n\
RESPONSE FORMAT:\n\
Action: [specific move/switch]\n\
Reason: [why in 1-2 sentences]";

    #[tokio::test]
    async fn test_agent_builder_run() -> Result<()> {
        let agent = AgentBuilder::new()
            .model("qwen/qwen3-8b")
            .url("http://localhost:1234/v1")
            .api_key("local")
            .system_prompt(SYSTEM_PROMPT)
            .temperature(0.5)
            .build()?;

        let mut history: Vec<Message> = Vec::new();

        let user_prompt = "\
        Battle Title: kashimo777 vs. ronak777\n\n\
        Generation: 6\n\n\
        You are assisting: ronak777\n\n\
        Player 1: \"kashimo777\", Team: [\"Amoonguss\", \"Bisharp\", \"Clefable\", \"Dragonite\", \"Excadrill\", \"Latios\"]\n\
        Player 2: \"ronak777\", Team: [\"Dragonite\", \"Zoroark\", \"Chansey\", \"Azumarill\", \"Charizard\", \"Gengar\"]\n\n\
        Question: Which Pokemon should lead with and why?";
        history.push(Message {
            role: user,
            content: user_prompt.to_string(),
        });

        let res = prompt(agent, user_prompt, history).await?;
        assert_eq!(!res.is_empty(), true);
        println!("Response: {}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_agent_builder_run_stream() -> Result<()> {
        let agent = AgentBuilder::new()
            .model("qwen/qwen3-8b")
            .url("http://localhost:1234/v1")
            .api_key("local")
            .system_prompt(SYSTEM_PROMPT)
            .temperature(0.5)
            .build()?;

        let mut history: Vec<Message> = Vec::new();

        let user_prompt = "Who are you? Explain in detail.";
        history.push(Message {
            role: user,
            content: user_prompt.to_string(),
        });

        let mut stream = prompt_stream(agent, user_prompt, history).await?;

        pretty_print_stream(100, &mut stream).await?;

        Ok(())
    }
}
