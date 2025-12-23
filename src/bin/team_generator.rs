use colored::Colorize;
use pokebrains::agents::{AgentBuilder, prompt_with_tools_stream};
use pokebrains::dtos::{Message, Role};
use pokebrains::request::log_typewriter_effect;
use pokebrains::tools::{PokeAPITool, PokemonShowdownTeamGeneratorTool};
use pokebrains::tools_registry::ToolRegistry;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tool_registry = ToolRegistry::new();
    tool_registry.register(PokeAPITool);
    tool_registry.register(PokemonShowdownTeamGeneratorTool);

    let agent = AgentBuilder::new()
        .model("qwen-v1")
        .system_prompt("You are Pokemon Master, which helps users build competitive Pokemon teams(Pokemon Showdown format) using tools at your disposal.")
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
            role: Role::USER,
            content: Some(user_input.to_string()),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });

        println!("{}", "Generating team...".magenta());

        let stream_response = prompt_with_tools_stream(agent.clone(), chat_history.clone()).await?;

        let stream_string = log_typewriter_effect(150, stream_response).await?;
        chat_history.push(Message {
            role: Role::ASSISTANT,
            content: Some(stream_string),
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
