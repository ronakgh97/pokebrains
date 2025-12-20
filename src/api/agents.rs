use crate::api::dtos::{CompletionRequest, Message};
use crate::api::request::{send_request, send_request_stream};
use crate::dtos::Role::SYSTEM;
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
    pub model: Option<String>,
    pub url: String,
    pub api_key: String,
    pub system_prompt: String,
    pub temperature: f32,
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

pub async fn prompt(agent: Agent, history: Vec<Message>) -> Result<String> {
    // Add system prompt to the beginning of history for non-repetitive context
    let mut history = history;
    history.insert(
        0,
        Message {
            role: SYSTEM,
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

    let response = send_request(agent.url.clone(), agent.api_key.clone(), request).await?;

    Ok(response)
}

pub async fn prompt_stream(
    agent: Agent,
    history: Vec<Message>,
) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
    // Add system prompt to the beginning of history for non-repetitive context
    let mut history = history;

    history.insert(
        0,
        Message {
            role: SYSTEM,
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
