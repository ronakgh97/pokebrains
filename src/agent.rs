#[allow(unused)]
use crate::agents::{
    Agent, AgentBuilder, prompt, prompt_stream, prompt_with_tools, prompt_with_tools_stream,
};
use crate::dtos::{Message, Role};
use crate::parser::logs::BattleEvents;
use crate::request::log_typewriter_effect;
use crate::tools_registry::ToolRegistry;
use anyhow::Result;
use colored::Colorize;
use std::sync::Arc;

const SYSTEM_PROMPT: &str = "\
You are a Pokemon Showdown battle Assistant.\n\
\n\
RULES:\n\
- You assist the player labeled [Assist]\n\
- You play against the player labeled [Against]\n\
- Give ONE concrete action only\n\
- Keep reasoning under 2 sentences\n\
- No speculation or uncertainty\n\
- Use tools at your disposal for accurate suggestions (if needed)\n\
\n\
RESPONSE FORMAT:\n\
Action: [specific move/switch]\n\
Reason: [why in 1-2 sentences]";

pub enum ModelType {
    Local,
    Cloud,
}

pub struct BattleAgent {
    model: String,
    model_type: ModelType,
    agent: Option<Agent>,
    pub history: Vec<Message>,
}

impl BattleAgent {
    pub fn new(model: &str, model_type: ModelType) -> Self {
        BattleAgent {
            model: model.to_string(),
            model_type,
            agent: None,
            history: Vec::new(),
        }
    }

    pub fn build_agent(self, api_key: &str, tools: ToolRegistry) -> Result<Self> {
        let agent = match self.model_type {
            ModelType::Local => AgentBuilder::new()
                .model(&self.model)
                .url("http://localhost:1234/v1")
                .api_key(api_key)
                .system_prompt(SYSTEM_PROMPT)
                .temperature(0.4)
                .tool_registry(Arc::new(tools))
                .build()?,
            ModelType::Cloud => AgentBuilder::new()
                .model(&self.model)
                .url("https://openrouter.ai/api/v1")
                .api_key(api_key)
                .system_prompt(SYSTEM_PROMPT)
                .temperature(0.4)
                .tool_registry(Arc::new(tools))
                .build()?,
        };

        Ok(Self {
            agent: Some(agent),
            ..self
        })
    }

    pub async fn get_initial_suggestions(&mut self, events: BattleEvents) -> String {
        println!("\n{}\n", "Generating initial suggestions...".yellow());

        let mut prompt = events
            .init
            .iter()
            .filter(|t| match t {
                crate::parser::logs::Token::PREVIEW(s) if *s => false,
                _ => true,
            })
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        prompt.push('\n');

        let team_match_up = format!(
            "Player 1: {:?}, Team: {:?}\nPlayer 2: {:?}, Team: {:?}\n",
            events.team[0].player,
            events.team[0].pokemon,
            events.team[1].player,
            events.team[1].pokemon,
        );
        prompt.push_str(&team_match_up);
        prompt.push('\n');

        let question = "Which Pokemon should lead with and why?";
        prompt.push_str(question);
        prompt.push('\n');

        self.generate_response(prompt).await
    }

    pub async fn get_initial_suggestions_stream(&mut self, events: BattleEvents) -> Result<()> {
        println!("\n{}\n", "Generating initial suggestions...".yellow());

        let mut prompt = events
            .init
            .iter()
            .filter(|t| match t {
                crate::parser::logs::Token::PREVIEW(s) if *s => false,
                _ => true,
            })
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        prompt.push('\n');

        let team_match_up = format!(
            "Player 1: {:?}, Team: {:?}\nPlayer 2: {:?}, Team: {:?}\n",
            events.team[0].player,
            events.team[0].pokemon,
            events.team[1].player,
            events.team[1].pokemon,
        );
        prompt.push_str(&team_match_up);
        prompt.push('\n');

        let question = "Which Pokemon should lead with and why?";
        prompt.push_str(question);
        prompt.push('\n');

        self.generate_stream_response(prompt).await?;

        Ok(())
    }

    pub async fn get_turn_suggestions(&mut self, events: BattleEvents) -> String {
        println!("\n{}\n", "Generating turn suggestions...".yellow());

        let mut prompt = String::new();

        // Add the last turn's data
        if let Some(last_turn) = events.events.last() {
            let turn_text = last_turn
                .iter()
                .filter(|t| !matches!(t, crate::parser::logs::Token::TURN(_)))
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            prompt.push_str(&turn_text);
            prompt.push('\n');
        }

        let question = "Based on the current battle state, what is the optimal move or switch?";
        prompt.push_str(question);
        prompt.push('\n');

        self.generate_response(prompt).await
    }

    pub async fn get_turn_suggestions_stream(&mut self, events: BattleEvents) -> Result<()> {
        println!("\n{}\n", "Generating turn suggestions...".yellow());

        let mut prompt = String::new();

        // Add the last turn's data
        if let Some(last_turn) = events.events.last() {
            let turn_text = last_turn
                .iter()
                .filter(|t| !matches!(t, crate::parser::logs::Token::TURN(_)))
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            prompt.push_str(&turn_text);
            prompt.push('\n');
        }

        let question = "Based on the current battle state, what is the optimal move or switch?";
        prompt.push_str(question);
        prompt.push('\n');

        self.generate_stream_response(prompt).await?;

        Ok(())
    }

    async fn generate_response(&mut self, user_prompt: String) -> String {
        //DEBUG
        println!();
        println!("[DEBUG] Prompt Sent to Agent:\n{}", user_prompt.dimmed());

        self.history.push(Message {
            role: Role::USER,
            content: Option::from(user_prompt.clone()),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });

        if let Some(agent) = &self.agent {
            match prompt_with_tools(agent.clone(), self.history.clone()).await {
                Ok(response) => {
                    self.history.push(Message {
                        role: Role::ASSISTANT,
                        content: Option::from(response.trim().to_string()),
                        tool_calls: None,
                        tool_call_id: None,
                        name: None,
                    });

                    response.trim().to_string().clone()
                }
                Err(e) => format!("Error generating suggestions: {}", e),
            }
        } else {
            "Agent not initialized.".to_string()
        }
    }

    async fn generate_stream_response(&mut self, user_prompt: String) -> Result<()> {
        //DEBUG
        println!();
        println!("[DEBUG] Prompt Sent to Agent:\n{}", user_prompt.dimmed());

        self.history.push(Message {
            role: Role::USER,
            content: Option::from(user_prompt.clone()),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });

        if let Some(agent) = &self.agent {
            let stream = prompt_with_tools_stream(agent.clone(), self.history.clone()).await?;

            // Print with typewriter effect AND accumulate
            let response = log_typewriter_effect(150, stream).await?;

            // Save to history
            self.history.push(Message {
                role: Role::ASSISTANT,
                content: Some(response.trim().to_string()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            });

            Ok(())
        } else {
            Err(anyhow::anyhow!("Agent not initialized."))
        }
    }
}
