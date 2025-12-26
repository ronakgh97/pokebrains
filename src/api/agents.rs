use crate::api::dtos::{CompletionRequest, Message};
#[allow(unused)]
use crate::api::request::send_request;
use crate::api::request::send_request_stream;
use crate::api::tools_registry::ToolRegistry;
use crate::dtos::Role::{ASSISTANT, SYSTEM};
use crate::dtos::ToolCall;
use crate::request::send_completion_request;
use anyhow::Result;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub struct Agent {
    pub model: String,
    pub url: String,
    pub api_key: String,
    pub system_prompt: String,
    pub temperature: f32,
    #[serde(skip_serializing, skip_deserializing, default)]
    pub tool_registry: Option<Arc<ToolRegistry>>,
    pub top_p: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AgentBuilder {
    pub model: Option<String>,
    pub url: String,
    pub api_key: String,
    pub system_prompt: String,
    pub temperature: f32,
    #[serde(skip_serializing, skip_deserializing, default)]
    pub tool_registry: Option<Arc<ToolRegistry>>,
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
            tool_registry: None,
            temperature: 0.7,
            top_p: 0.9,
        }
    }
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_toml(path: &str) -> Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        let agent_builder = toml::from_str::<AgentBuilder>(&config_str)?;
        Ok(agent_builder)
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

    pub fn tool_registry(mut self, registry: Arc<ToolRegistry>) -> Self {
        self.tool_registry = Some(registry);
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
            tool_registry: self.tool_registry,
            top_p: self.top_p,
        })
    }
}

/// Low level function to send a prompt and get a response from the agent.
pub async fn prompt(
    agent: Agent,
    history: Vec<Message>,
) -> Result<(String, Option<Vec<ToolCall>>)> {
    // Add system prompt to the beginning of history for non-repetitive context
    let mut history = history;
    history.insert(
        0,
        Message {
            role: SYSTEM,
            content: Option::from(agent.clone().system_prompt),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        },
    );

    // // Add user prompt
    // history.push(Message {
    //     role: Role::user,
    //     content: prompt.to_string(),
    // });

    let request = CompletionRequest {
        model: agent.clone().model,
        messages: history,
        tools: agent
            .tool_registry
            .as_ref()
            .map(|reg| reg.get_tool_definitions()),
        temperature: agent.temperature,
        top_p: Some(agent.top_p),
        stream: Some(false),
    };

    let response =
        send_completion_request(agent.url.clone(), agent.api_key.clone(), request).await?;

    let get_content = &response
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No choices in response"))?
        .message
        .content
        .clone()
        .unwrap_or_default();

    let tool_call = &response
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No choices in response"))?
        .message
        .tool_calls;

    Ok((get_content.clone(), tool_call.clone()))
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
            content: Option::from(agent.clone().system_prompt),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        },
    );

    // // Add user prompt
    // history.push(Message {
    //     role: Role::user,
    //     content: prompt.to_string(),
    // });

    let request = CompletionRequest {
        model: agent.clone().model,
        messages: history,
        tools: agent
            .tool_registry
            .as_ref()
            .map(|reg| reg.get_tool_definitions()),
        temperature: agent.temperature,
        top_p: Some(agent.top_p),
        stream: Some(true),
    };

    let stream = send_request_stream(agent.url.clone(), agent.api_key.clone(), request).await?;

    Ok(Box::pin(stream))
}

/// High-level helper.
///
/// - Runs the tool loop internally until no more tool calls are needed.
/// - Does NOT expose intermediate tool or assistant messages.
/// - Suitable for stateless, one-shot queries.
/// - If you need full control over history or tools, use [`prompt`] directly.
pub async fn prompt_with_tools(agent: Agent, mut history: Vec<Message>) -> Result<String> {
    // TODO: Return history?
    let registry = match &agent.tool_registry {
        Some(r) => r,
        None => return Err(anyhow::anyhow!("No tool registry")),
    };

    loop {
        let (response, tools_list) = prompt(agent.clone(), history.clone()).await?;

        // No tool calls? STOP!!
        if tools_list.is_none() {
            return Ok(response);
        }

        let calls = tools_list.unwrap(); // Safe unwrap

        // Add assistant message with tool_calls FIRST
        history.push(Message {
            role: ASSISTANT,
            content: Some(response.clone()),
            tool_calls: Some(calls.clone()),
            tool_call_id: None,
            name: None,
        });

        let mut should_loop = false;

        // Execute each tool
        for call in calls {
            let tool_name = &call.function.name;
            let should_callback = registry.check_tool_callback(tool_name)?;

            let args: serde_json::Value = serde_json::from_str(&call.function.arguments)?;
            let result = registry.execute(tool_name, args).await?;

            if !should_callback {
                return Ok(result);
            }

            history.push(Message {
                role: crate::dtos::Role::TOOL,
                content: Some(result),
                tool_calls: None,
                tool_call_id: Some(call.id.clone()),
                name: Some(tool_name.clone()),
            });

            should_loop = true;
        }

        if !should_loop {
            // No tools wanted callback
            return Ok(response);
        }
    }
}

/// High-level streaming with automatic tool execution.
///
/// - Executes tools silently (non-streaming)
/// - Returns stream of final answer only
/// - Compatible with [`prompt_with_tools`] design
pub async fn prompt_with_tools_stream(
    agent: Agent,
    mut history: Vec<Message>,
) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
    let registry = agent
        .tool_registry
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No tool registry"))?;

    const MAX_ITERATIONS: usize = 10;

    for _iteration in 0..MAX_ITERATIONS {
        let (response, tools_list) = prompt(agent.clone(), history.clone()).await?;

        // No tool calls? STOP!!
        if tools_list.is_none() {
            return prompt_stream(agent, history).await;
        }

        let calls = tools_list.unwrap(); // Safe unwrap

        history.push(Message {
            role: ASSISTANT,
            content: Some(response.clone()),
            tool_calls: Some(calls.clone()),
            tool_call_id: None,
            name: None,
        });

        let mut should_loop = false;

        for call in calls {
            let tool_name = &call.function.name;
            let should_callback = registry.check_tool_callback(tool_name)?;

            let args: serde_json::Value = serde_json::from_str(&call.function.arguments)?;
            let result = registry.execute(tool_name, args).await?;

            if !should_callback {
                use futures_util::stream;
                let stream = stream::once(async move { Ok(result) });
                return Ok(Box::pin(stream));
            }

            history.push(Message {
                role: crate::dtos::Role::TOOL,
                content: Some(result),
                tool_calls: None,
                tool_call_id: Some(call.id.clone()),
                name: Some(tool_name.clone()),
            });

            should_loop = true;
        }

        if !should_loop {
            use futures_util::stream;
            let stream = stream::once(async move { Ok(response) });
            return Ok(Box::pin(stream));
        }
    }

    Err(anyhow::anyhow!(
        "Max iterations ({}) reached",
        MAX_ITERATIONS
    ))
}

#[tokio::test]
async fn test_agent_tool() -> Result<()> {
    use crate::dtos::Role::USER;
    use crate::tools::PokeAPITool;

    let mut tool_registry = ToolRegistry::new();
    tool_registry.register(PokeAPITool);

    let agent = AgentBuilder::new()
        .model("qwen/qwen3-8b")
        .url("http://localhost:1234/v1")
        .api_key("local")
        .tool_registry(Arc::new(tool_registry.clone()))
        .temperature(0.5)
        .build()?;

    let mut history: Vec<Message> = Vec::new();

    let user_prompt = "What is palkia's type?";
    history.push(Message {
        role: USER,
        content: Option::from(user_prompt.to_string()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    });

    let response = prompt_with_tools(agent, history.clone()).await?;

    history.push(Message {
        role: ASSISTANT,
        content: Option::from(response.to_string()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    });

    println!();
    println!("{:#?}", &response);
    println!();
    println!("{:#?}", history.clone());
    assert!(!response.trim().is_empty());

    // if let Some(calls) = tools_list {
    //     for call in calls {
    //         let args: serde_json::Value = serde_json::from_str(&call.function.arguments)?;
    //         let result = tool_registry.execute(&call.function.name, args).await?;
    //
    //         dbg!(&result);
    //     }
    // }

    Ok(())
}

#[tokio::test]
async fn test_agent_tool_stream() -> Result<()> {
    use crate::api::request::log_typewriter_effect;
    use crate::dtos::Role::USER;
    use crate::tools::PokeAPITool;
    use crate::tools::PokemonShowdownTeamGeneratorTool;

    let mut tool_registry = ToolRegistry::new();
    tool_registry.register(PokeAPITool);
    tool_registry.register(PokemonShowdownTeamGeneratorTool);

    let agent = AgentBuilder::new()
        .model("qwen/qwen3-8b")
        .url("http://localhost:1234/v1")
        .api_key("local")
        .tool_registry(Arc::new(tool_registry.clone()))
        .temperature(0.5)
        .build()?;

    let mut history: Vec<Message> = Vec::new();

    let user_prompt = "generate a pokemon showdown team with gen 5 pokemon with good type synergy";
    history.push(Message {
        role: USER,
        content: Option::from(user_prompt.to_string()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    });

    let stream_response = prompt_with_tools_stream(agent, history.clone()).await?;

    let string_response = log_typewriter_effect(100, stream_response).await?;

    history.push(Message {
        role: ASSISTANT,
        content: Option::from(string_response.trim().to_string()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    });

    history.push(Message {
        role: ASSISTANT,
        content: Option::from(string_response.to_string()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    });

    println!();
    println!("{:#?}", &string_response);
    println!();
    println!("{:#?}", history.clone());
    assert!(!string_response.trim().is_empty());

    // if let Some(calls) = tools_list {
    //     for call in calls {
    //         let args: serde_json::Value = serde_json::from_str(&call.function.arguments)?;
    //         let result = tool_registry.execute(&call.function.name, args).await?;
    //
    //         dbg!(&result);
    //     }
    // }

    Ok(())
}

#[tokio::test]
async fn test_load_from_config() -> Result<()> {
    let agent_builder = AgentBuilder::load_from_toml("agent_config.toml")?;
    let agent = agent_builder.build()?;

    assert_eq!(agent.model, "qwen/qwen3-8b");
    assert_eq!(agent.url, "http://localhost:1234/v1");
    assert_eq!(agent.api_key, "local");
    assert_eq!(
        agent.system_prompt,
        "You are a helpful assistant.\n Strict follow user instructions"
    );
    assert_eq!(agent.temperature, 0.5);
    assert_eq!(agent.top_p, 0.9);

    Ok(())
}
