use colored::Colorize;
use forge::api::agents::{AgentBuilder, prompt_with_tools_stream};
use forge::api::dtos::Message;
use forge::api::dtos::Role::{ASSISTANT, USER};
use forge::api::request::log_typewriter_effect;
use forge::api::tools_registry::ToolRegistry;
use pokebrains::tools::{PokeAPITool, PokemonShowdownTeamGeneratorTool, TeamValidatorTool};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tool_registry = ToolRegistry::new();
    tool_registry.register(PokeAPITool);
    tool_registry.register(PokemonShowdownTeamGeneratorTool);
    tool_registry.register(TeamValidatorTool);

    let agent = AgentBuilder::new()
        .model("qwen/qwen3-8b")
        .system_prompt("You are Pokemon Master, who helps users build competitive valid Pokemon teams (Pokemon Showdown format) using tools at your disposal.")
        .url("http://localhost:1234/v1")
        .api_key("local")
        .tool_registry(Arc::new(tool_registry))
        .build()?;

    let mut chat_history: Vec<Message> = Vec::new();

    print_ascii_art();
    loop {
        println!();
        println!("{}", "Enter your team requirements: ".yellow());
        print!("{} ", "↪".yellow());
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");
        println!();

        let user_input = user_input.trim();
        if user_input.eq_ignore_ascii_case("exit") || user_input.eq_ignore_ascii_case("quit") {
            println!("{}", "Exiting...".red());
            std::process::exit(0);
        }

        chat_history.push(Message {
            role: USER,
            content: Some(user_input.to_string()),
            multi_content: None,
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });

        println!("{}", "Generating team...\n".magenta());

        let stream_response =
            prompt_with_tools_stream(agent.clone(), chat_history.clone(), 10).await?;

        let stream_string = log_typewriter_effect(150, stream_response).await?;
        chat_history.push(Message {
            role: ASSISTANT,
            content: Some(stream_string),
            multi_content: None,
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });
    }
}

fn print_ascii_art() {
    let text_art = r"

    ███        ▄████████    ▄████████   ▄▄▄▄███▄▄▄▄        ████████▄     ▄████████ ▀████    ▐████▀
▀█████████▄   ███    ███   ███    ███ ▄██▀▀▀███▀▀▀██▄      ███   ▀███   ███    ███   ███▌   ████▀
   ▀███▀▀██   ███    █▀    ███    ███ ███   ███   ███      ███    ███   ███    █▀     ███  ▐███
    ███   ▀  ▄███▄▄▄       ███    ███ ███   ███   ███      ███    ███  ▄███▄▄▄        ▀███▄███▀
    ███     ▀▀███▀▀▀     ▀███████████ ███   ███   ███      ███    ███ ▀▀███▀▀▀        ████▀██▄
    ███       ███    █▄    ███    ███ ███   ███   ███      ███    ███   ███    █▄    ▐███  ▀███
    ███       ███    ███   ███    ███ ███   ███   ███      ███   ▄███   ███    ███  ▄███     ███▄
   ▄████▀     ██████████   ███    █▀   ▀█   ███   █▀       ████████▀    ██████████ ████       ███▄

    ";

    println!("{}", text_art.cyan());
}
