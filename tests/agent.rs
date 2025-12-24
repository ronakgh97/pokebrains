use anyhow::Result;
use pokebrains::agents::{AgentBuilder, prompt, prompt_stream};
use pokebrains::dtos::Message;
use pokebrains::dtos::Role::USER;
use pokebrains::request::log_typewriter_effect;

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
        role: USER,
        content: Option::from(user_prompt.to_string()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    });

    let res = prompt(agent, history).await?;
    assert!(!res.0.is_empty());
    println!("Response: {:?}", res);
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
        role: USER,
        content: Option::from(user_prompt.to_string()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    });

    let mut stream = prompt_stream(agent, history).await?;

    log_typewriter_effect(120, &mut stream).await?;

    Ok(())
}
