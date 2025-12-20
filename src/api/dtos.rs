use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    SYSTEM,
    USER,
    ASSISTANT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    id: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionStreamResponse {
    id: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamChoice {
    pub index: u32,
    pub delta: StreamChunkMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamChunkMessage {
    #[serde(default)]
    pub role: Option<Role>,

    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionChoice {
    index: u32,
    pub message: Message,
}
