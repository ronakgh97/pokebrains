#![allow(unreachable_patterns)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub multi_content: Option<Vec<MultiContent>>, // IMPORTANT: "content" and "multi_content" are mutually exclusive, this is done to match the API spec
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Separate dto to handle text and image content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiContent {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrl>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    SYSTEM,
    USER,
    ASSISTANT,
    TOOL,
}

/// Definition of a tool/function that can be called by the model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub r#type: String,
    pub function: FunctionDefinition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value, // JSON Schema
}

/// Generated when the model calls a tool/function
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String, // JSON string
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

#[tokio::test]
async fn test_message_content_serialization() -> anyhow::Result<()> {
    // Simple text content serialization
    let msg_text = Message {
        role: Role::USER,
        content: Some("Hello, world!".to_string()),
        multi_content: None,
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };
    let json_text = serde_json::to_string(&msg_text)?;
    println!("Text message: {}", json_text);
    assert!(json_text.contains(r#""content":"Hello, world!""#));

    // Multi-content with text serialization
    let msg_multi_text = Message {
        role: Role::USER,
        content: None,
        multi_content: Some(vec![MultiContent {
            r#type: "text".to_string(),
            text: Some("Describe this image".to_string()),
            image_url: None,
        }]),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };
    let json_multi_text = serde_json::to_string(&msg_multi_text)?;
    println!("Multi-content text: {}", json_multi_text);
    assert!(json_multi_text.contains(r#""type":"text""#));
    assert!(json_multi_text.contains(r#""text":"Describe this image""#));

    // Multi-content with image serialization
    let msg_multi_image = Message {
        role: Role::USER,
        content: None,
        multi_content: Some(vec![MultiContent {
            r#type: "image_url".to_string(),
            text: None,
            image_url: Some(ImageUrl {
                url: "data:image/png;base64,iVBORw0KG...".to_string(),
            }),
        }]),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };
    let json_multi_image = serde_json::to_string(&msg_multi_image)?;
    println!("Multi-content image: {}", json_multi_image);
    assert!(json_multi_image.contains(r#""type":"image_url""#));
    assert!(json_multi_image.contains(r#""url":"data:image/png;base64,iVBORw0KG...""#));

    // Deserialize API response with simple text content
    let api_response = r#"{
        "role": "assistant",
        "content": "This is the AI response"
    }"#;
    let msg_response: Message = serde_json::from_str(api_response)?;
    println!("Deserialized response: {:?}", msg_response);
    assert_eq!(
        msg_response.content,
        Some("This is the AI response".to_string())
    );
    assert!(msg_response.multi_content.is_none());
    assert_eq!(msg_response.role, Role::ASSISTANT);

    // Deserialize user message with simple text
    let user_message = r#"{
        "role": "user",
        "content": "What is the weather?"
    }"#;
    let msg_user: Message = serde_json::from_str(user_message)?;
    println!("Deserialized user message: {:?}", msg_user);
    assert_eq!(msg_user.content, Some("What is the weather?".to_string()));
    assert!(msg_user.multi_content.is_none());

    Ok(())
}
